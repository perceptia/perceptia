// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Dispatcher` provides functionality to implement main program loop by waiting for system events
//! using `epoll` mechanism.

// -------------------------------------------------------------------------------------------------

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::unix::io::RawFd;
use std::collections::HashMap;
use std::convert::From;

use nix;
use nix::sys::epoll;

use system;

// -------------------------------------------------------------------------------------------------

/// `epoll_wait` waits infinitely when passed negative number as timeout.
const WAIT_INFINITELY: isize = -1;

// -------------------------------------------------------------------------------------------------

/// This module contains flags defining kind of event.
pub mod event_kind {
    use std::convert::From;
    use nix::sys::epoll;

    bitflags!(
        /// Type defining event kind.
        pub flags EventKind: u32 {
            /// Defines read event.
            const READ = 0x1,
            /// Defines write event.
            const WRITE = 0x2,
            /// Defines error or hangup. Can be omitted when adding source. It will be subscribed
            /// even if not specified, so it has to be always handled by `EventHandler`
            /// implementation.
            const HANGUP = 0x4
        }
    );

    impl From<epoll::EpollEventKind> for EventKind {
        fn from(flags: epoll::EpollEventKind) -> Self {
            let mut result = EventKind::empty();
            if flags.intersects(epoll::EPOLLIN) {
                result.insert(READ);
            }
            if flags.intersects(epoll::EPOLLOUT) {
                result.insert(WRITE);
            }
            if flags.intersects(epoll::EPOLLERR | epoll::EPOLLHUP) {
                result.insert(HANGUP);
            }
            result
        }
    }

    impl Into<epoll::EpollEventKind> for EventKind {
        fn into(self) -> epoll::EpollEventKind {
            let mut result = epoll::EpollEventKind::empty();
            if self.intersects(READ) {
                result.insert(epoll::EPOLLIN);
            }
            if self.intersects(WRITE) {
                result.insert(epoll::EPOLLOUT);
            }
            if self.intersects(HANGUP) {
                // Only for completeness. This is not necessary to pass these flags to `epoll_ctl`.
                result.insert(epoll::EPOLLERR | epoll::EPOLLHUP);
            }
            result
        }
    }
}

pub use event_kind::EventKind;

// -------------------------------------------------------------------------------------------------

/// Id of `EventHandler` (generated by `Dispatcher`).
pub type EventHandlerId = u64;

// -------------------------------------------------------------------------------------------------

/// Trait for all structures supposed to be handlers for events registered in `Dispatcher`.
/// `EventHandler` is responsible for processing events. `EventHandler::process_event` will be
/// called when handlers file descriptor becomes readable in thread where `Dispatcher::start` was
/// called.
pub trait EventHandler: Send {
    /// Returns file descriptor.
    fn get_fd(&self) -> RawFd;

    /// Callback function executed on event received.
    fn process_event(&mut self, event_kind: EventKind);

    /// This method is called by `Dispatcher` right after adding this `EventHandler`. Passed value
    /// is newly assigned ID of `EventHandler` which can be later used to delete it from
    /// `Dispatcher`.
    fn set_id(&mut self, _id: EventHandlerId) {}
}

// -------------------------------------------------------------------------------------------------

/// Helper alias.
type EventHandlerMap = HashMap<EventHandlerId, Box<EventHandler>>;

// -------------------------------------------------------------------------------------------------

/// Helper structure guarded by mutex.
struct LocalDispatcher {
    epfd: RawFd,
    last_id: EventHandlerId,
    handlers: EventHandlerMap,
}

// -------------------------------------------------------------------------------------------------

impl LocalDispatcher {
    /// `LocalDispatcher` constructor.
    pub fn new() -> Self {
        LocalDispatcher {
            epfd: epoll::epoll_create().expect("Failed to create epoll!"),
            last_id: 0,
            handlers: HashMap::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between `Dispatcher`s from different threads.
struct InnerDispatcher {
    inner: Mutex<LocalDispatcher>,
    run: AtomicBool,
}

// -------------------------------------------------------------------------------------------------

/// Structure representing dispatcher of system events.
#[derive(Clone)]
pub struct Dispatcher {
    state: Arc<InnerDispatcher>,
}

// -------------------------------------------------------------------------------------------------

impl Dispatcher {
    /// `Dispatcher` constructor.
    pub fn new() -> Self {
        Dispatcher {
            state: Arc::new(InnerDispatcher {
                inner: Mutex::new(LocalDispatcher::new()),
                run: AtomicBool::new(false),
            }),
        }
    }

    /// Adds `EventHandler`.
    /// Return ID assigned to the added `EventHandler` which can be used to later delete it.
    pub fn add_source(&mut self,
                      mut source: Box<EventHandler>,
                      event_kind: EventKind)
                      -> EventHandlerId {
        let mut mine = self.state.inner.lock().expect("Locking Dispatcher inner state");

        mine.last_id += 1;
        let last_id = mine.last_id;
        source.set_id(last_id);
        let fd = source.get_fd();
        mine.handlers.insert(last_id, source);

        let event = epoll::EpollEvent {
            events: event_kind.into(),
            data: last_id,
        };

        epoll::epoll_ctl(mine.epfd, epoll::EpollOp::EpollCtlAdd, fd, &event)
            .expect("Failed to perform `epoll_ctl`");

        last_id
    }

    /// Deleted `EventHandler`.
    pub fn delete_source(&mut self, id: EventHandlerId) -> Option<Box<EventHandler>> {
        let mut mine = self.state.inner.lock().expect("Locking Dispatcher inner state");

        let result = mine.handlers.remove(&id);
        if let Some(ref handler) = result {
            let event = epoll::EpollEvent {
                events: epoll::EpollEventKind::empty(),
                data: 0,
            };
            epoll::epoll_ctl(mine.epfd,
                             epoll::EpollOp::EpollCtlDel,
                             handler.get_fd(),
                             &event)
                .expect("Failed to delete epoll source");
        }
        result
    }

    /// Starts processing events in current thread.
    pub fn start(&self) {
        // Initial setup
        system::block_signals();
        let epfd = self.get_epfd();

        // Main loop
        loop {
            self.do_wait_and_process(epfd, WAIT_INFINITELY);

            if !self.state.run.load(Ordering::Relaxed) {
                break;
            }
        }
    }

    /// Waits for events and processes first one.
    pub fn wait_and_process(&self, timeout: Option<usize>) {
        let timeout = if let Some(t) = timeout {
            t as isize
        } else {
            WAIT_INFINITELY
        };
        let epfd = self.get_epfd();
        self.do_wait_and_process(epfd, timeout);
    }

    /// Stops `Dispatcher`s loop.
    pub fn stop(&self) {
        self.state.run.store(false, Ordering::Relaxed);
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods.
impl Dispatcher {
    /// Helper method for waiting for events and then processing them.
    fn do_wait_and_process(&self, epfd: RawFd, timeout: isize) {
        // We will process epoll events one by one.
        let mut events: [epoll::EpollEvent; 1] = [epoll::EpollEvent {
                                                      events: epoll::EpollEventKind::empty(),
                                                      data: 0,
                                                  }];

        let wait_result = epoll::epoll_wait(epfd, &mut events[0..1], timeout);

        match wait_result {
            Ok(ready) => {
                if ready > 0 {
                    let mut mine = self.state.inner.lock().unwrap();
                    if let Some(handler) = mine.handlers.get_mut(&events[0].data) {
                        handler.process_event(EventKind::from(events[0].events));
                    }
                }
            }
            Err(err) => {
                if let nix::Error::Sys(errno) = err {
                    if errno != nix::Errno::EINTR {
                        panic!("Error occurred during processing epoll events! ({:?})", err);
                    }
                }
            }
        }
    }

    /// Helper method for getting epoll file descriptor.
    fn get_epfd(&self) -> RawFd {
        let mine = self.state.inner.lock().unwrap();
        self.state.run.store(true, Ordering::Relaxed);
        mine.epfd
    }
}

// -------------------------------------------------------------------------------------------------
