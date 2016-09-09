// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Dispatcher` provides functionality to implement main program loop by waiting for system events
//! using `epoll` mechanism.

// -------------------------------------------------------------------------------------------------

use std::ops::IndexMut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::unix::io::RawFd;
use nix::sys::{epoll, signal};

use system;

// -------------------------------------------------------------------------------------------------

/// `epoll_wait` waits infinitely when passed negative number as timeout.
const WAIT_INFINITELY: isize = -1;

// -------------------------------------------------------------------------------------------------

/// Trait for all structures supposed to be handlers for events registered in `Dispatcher`.
/// `EventHandler` is responsible for processing events. `EventHandler::process_event` will be
/// called when handlers file descriptor becomes readable in thread where `Dispatcher::start` was
/// called.
pub trait EventHandler: Send {
    fn get_fd(&self) -> RawFd;
    fn process_event(&mut self);
}

// -------------------------------------------------------------------------------------------------

/// Helper structure guarded by mutex.
struct InnerDispatcher {
    epfd: RawFd,
    handlers: Vec<Box<EventHandler>>,
}

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between `Dispatcher`s from different threads.
struct OuterDispatcher {
    inner: Mutex<InnerDispatcher>,
    run: AtomicBool,
}

// -------------------------------------------------------------------------------------------------

/// Structure representing dispatcher of system events.
#[derive(Clone)]
pub struct Dispatcher {
    state: Arc<OuterDispatcher>,
}

// -------------------------------------------------------------------------------------------------

impl Dispatcher {
    /// `Dispatcher` constructor.
    pub fn new() -> Self {
        Dispatcher {
            state: Arc::new(OuterDispatcher {
                inner: Mutex::new(InnerDispatcher {
                    epfd: epoll::epoll_create().expect("Failed to create epoll!"),
                    handlers: Vec::new(),
                }),
                run: AtomicBool::new(false),
            }),
        }
    }

    /// Add `EventHandler`.
    pub fn add_source(&mut self, source: Box<EventHandler>) {
        let mut mine = self.state.inner.lock().unwrap();

        let fd = source.get_fd();
        let pos = mine.handlers.len();
        mine.handlers.push(source);

        let event = epoll::EpollEvent {
            events: epoll::EPOLLIN,
            data: pos as u64,
        };

        epoll::epoll_ctl(mine.epfd, epoll::EpollOp::EpollCtlAdd, fd, &event)
            .expect("Failed to perform `epoll_ctl`");
    }

    /// Starts processing events in current thread.
    pub fn start(&self) {
        system::block_signals();

        // We will process epoll events one by one
        let mut events: [epoll::EpollEvent; 1] = [epoll::EpollEvent {
                                                      events: epoll::EpollEventKind::empty(),
                                                      data: 0,
                                                  }];

        // Initial setup
        let epfd;
        {
            let mut mine = self.state.inner.lock().unwrap();
            self.state.run.store(true, Ordering::Relaxed);
            epfd = mine.epfd;
        }

        loop {
            let wait_result = epoll::epoll_wait(epfd, &mut events[0..1], WAIT_INFINITELY);

            {
                let mut mine = self.state.inner.lock().unwrap();
                match wait_result {
                    Ok(_) => {
                        let ref mut handler = mine.handlers.index_mut(events[0].data as usize);
                        handler.process_event();
                    }
                    Err(err) => {
                        panic!("Error occurred during processing epoll events! ({:?})", err);
                    }
                }
            }

            if !self.state.run.load(Ordering::Relaxed) {
                break;
            }
        }
    }

    /// Stops `Dispatcher`s loop.
    pub fn stop(&self) {
        self.state.run.store(false, Ordering::Relaxed);
    }
}

// -------------------------------------------------------------------------------------------------
