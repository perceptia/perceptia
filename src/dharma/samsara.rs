// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Samsara` is event loop. It organizes work flow of a single thread. Independent logical parts
//! of application called `Module`s can freely add to  given `Samsara`s creating flexible
//! multi-threading framework. `Samsara` contains one receiver which can be used to push events and
//! data to event queue.

// -------------------------------------------------------------------------------------------------

use std;
use std::collections::btree_map::BTreeMap as Map;

use bridge::{self, ReceiveResult};
use signaler;

// -------------------------------------------------------------------------------------------------

/// Result of initialization of module. Module should return vector of signal identifiers it wants
/// to be subscribed for.
pub type InitResult = Vec<signaler::SignalId>;

// -------------------------------------------------------------------------------------------------

/// `Module` defines independent part of application. One module is bounded only to single thread.
/// More modules may be run in the same thread. Modules do not share memory, communicate with
/// signals.
pub trait Module: Send {
    type T: bridge::Transportable;
    fn initialize(&mut self, context: &mut Context<Self::T>) -> InitResult;
    fn execute(&mut self, package: &Self::T);
    fn finalize(&mut self);
}

// -------------------------------------------------------------------------------------------------

/// Application `Context` lets `Module` communicate with other `Module`s by mean of signals.
pub struct Context<'a, P: 'a + bridge::Transportable> {
    signaler: &'a mut signaler::Signaler<P>,
}

// -------------------------------------------------------------------------------------------------

impl<'a, P: bridge::Transportable> Context<'a, P> {
    /// Context constructor.
    fn new(signaler: &'a mut signaler::Signaler<P>) -> Self {
        Context { signaler: signaler }
    }

    /// Emit signal with given `id` and `package` data.
    pub fn emit(&mut self, id: signaler::SignalId, package: P) {
        self.signaler.emit(id, package);
    }
}

// -------------------------------------------------------------------------------------------------

/// Thread loop with event queue with communication over `bridge`s.
pub struct Samsara<P: bridge::Transportable + 'static> {
    name: String,
    modules: Vec<Box<Module<T = P>>>,
    receiver: bridge::Receiver<signaler::Event<P>>,
    signaler: signaler::Signaler<P>,
    subscriptions: Map<signaler::SignalId, Vec<usize>>,
}

// -------------------------------------------------------------------------------------------------

impl<P: bridge::Transportable + std::fmt::Display> Samsara<P> {
    /// `Samsara` constructor.
    pub fn new(name: String, signaler: signaler::Signaler<P>) -> Self {
        Samsara {
            name: name,
            modules: Vec::new(),
            receiver: bridge::Receiver::new(),
            signaler: signaler,
            subscriptions: Map::new(),
        }
    }

    /// Take `Samsara` to start event loop in new thread.
    pub fn start(self) -> std::io::Result<std::thread::JoinHandle<()>> {
        std::thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || self.run())
    }

    /// Add module.
    pub fn add_module(&mut self, module: Box<Module<T = P>>) {
        self.modules.push(module);
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
            let signals = m.initialize(&mut Context::new(&mut self.signaler));
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

    /// Helper method implementing main loop of `Samsara`.
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
