// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `EventLoop` organizes work flow of a single thread. Independent logical parts of application
//! called `Module`s can be freely added to given `EventLoop`s creating flexible multi-threading
//! framework. `EventLoop` contains one receiver which can be used to push events and data to event
//! queue.
//!
//! `Module`s are created inside new thread so do not have to implement Send. User passes only
//! their constructors to `EventLoopInfo` structure which is context for creation on `EventLoop`.

// -------------------------------------------------------------------------------------------------

use std;
use std::collections::btree_map::BTreeMap as Map;
use std::boxed::FnBox;

use bridge::{self, ReceiveResult};
use signaler;
use system;

// -------------------------------------------------------------------------------------------------

/// Result of initialization of module. Module should return vector of signal identifiers it wants
/// to be subscribed for.
pub type InitResult = Vec<signaler::SignalId>;

// -------------------------------------------------------------------------------------------------

/// `Module` defines independent part of application. One module is bounded only to single thread.
/// More modules may be run in the same thread. Modules do not share memory, communicate with
/// signals.
pub trait Module {
    type T: bridge::Transportable;
    type C: Send + Sync + Clone;
    fn initialize(&mut self, mut context: Self::C) -> InitResult;
    fn execute(&mut self, package: &Self::T);
    fn finalize(&mut self);
}

// -------------------------------------------------------------------------------------------------

/// Context for creation of `EventLoop`.
pub struct EventLoopInfo<P, C>
    where P: bridge::Transportable + 'static,
          C: Send + Sync + Clone + 'static
{
    name: String,
    signaler: signaler::Signaler<P>,
    constructors: Vec<Box<FnBox() -> Box<Module<T = P, C = C>> + Send + Sync>>,
    context: C,
}

// -------------------------------------------------------------------------------------------------

impl<P, C> EventLoopInfo<P, C>
    where P: bridge::Transportable + std::fmt::Display,
          C: Send + Sync + Clone
{
    /// `EventLoop` constructor.
    pub fn new(name: String, signaler: signaler::Signaler<P>, context: C) -> Self {
        EventLoopInfo {
            name: name,
            signaler: signaler,
            constructors: Vec::new(),
            context: context,
        }
    }

    /// Add module constructor.
    pub fn add_module(&mut self,
                      constructor: Box<FnBox() -> Box<Module<T = P, C = C>> + Send + Sync>) {
        self.constructors.push(constructor);
    }

    /// Consume `EventLoopInfo` to start event loop in new thread.
    pub fn start_event_loop(self) -> std::io::Result<std::thread::JoinHandle<()>> {
        std::thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || EventLoop::new(self).run())
    }
}

// -------------------------------------------------------------------------------------------------

/// Thread loop with event queue with communication over `bridge`s.
pub struct EventLoop<P, C>
    where P: bridge::Transportable + 'static,
          C: Send + Sync + Clone
{
    signaler: signaler::Signaler<P>,
    modules: Vec<Box<Module<T = P, C = C>>>,
    receiver: bridge::Receiver<signaler::Event<P>>,
    subscriptions: Map<signaler::SignalId, Vec<usize>>,
    context: C,
}

// -------------------------------------------------------------------------------------------------

impl<P, C> EventLoop<P, C>
    where P: bridge::Transportable + std::fmt::Display,
          C: Send + Sync + Clone
{
    /// `EventLoop` constructor. Constructs `EventLoop` and all assigned modules.
    pub fn new(mut info: EventLoopInfo<P, C>) -> Self {
        // Block chosen signals for this thread.
        system::block_signals();

        // Create event loop
        let mut event_loop = EventLoop {
            signaler: info.signaler,
            modules: Vec::new(),
            receiver: bridge::Receiver::new(),
            subscriptions: Map::new(),
            context: info.context,
        };

        // Consume constructors to return module instances
        loop {
            match info.constructors.pop() {
                Some(constructor) => event_loop.modules.push(constructor.call_box(())),
                None => break,
            }
        }

        event_loop
    }

    /// Thread body.
    fn run(mut self) {
        self.initialize();
        self.do_run();
        self.finalize();
    }

    /// Helper method for initializing modules.
    fn initialize(&mut self) {
        self.signaler.register(&self.receiver);
        let mut i = 0;
        for mut m in self.modules.iter_mut() {
            let signals = m.initialize(self.context.clone());
            for s in signals {
                if self.subscriptions.contains_key(&s) {
                    match self.subscriptions.get_mut(&s) {
                        Some(ref mut subscribers) => {
                            subscribers.push(i);
                        }
                        None => {} // FIXME warn
                    }
                } else {
                    self.subscriptions.insert(s, vec![i]);
                    self.signaler.subscribe(s, &self.receiver);
                }
            }
            i += 1;
        }
    }

    /// Helper method implementing main loop of `EventLoop`.
    fn do_run(&mut self) {
        loop {
            match self.receiver.recv() {
                ReceiveResult::Ok(event) => {
                    match event {
                        signaler::Event::Package { id, package } => {
                            match self.subscriptions.get_mut(&id) {
                                Some(ref mut subscribers) => {
                                    // Inform all subscriber about notification.
                                    for i in subscribers.iter() {
                                        self.modules[*i].execute(&package);
                                    }
                                }
                                None => {
                                    // Received signal we did not subscribe for.
                                    // Ignore it.
                                }
                            }
                        }
                        signaler::Event::Terminate => break,
                    }
                }
                ReceiveResult::Timeout => {}
                ReceiveResult::Empty => break,
                ReceiveResult::Err => break,
            }
        }
    }

    /// Helper method for finalizing modules.
    fn finalize(&mut self) {
        for mut m in self.modules.iter_mut() {
            m.finalize();
        }
    }
}

// -------------------------------------------------------------------------------------------------
