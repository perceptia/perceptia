// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains application context used to share data between threads.

// -------------------------------------------------------------------------------------------------

use dharma::{Dispatcher, EventHandler, Signaler, SignalId};

use perceptron::Perceptron;

use coordinator::Coordinator;

// -------------------------------------------------------------------------------------------------

/// Application `Context` lets `Module`s communicate with other `Module`s by mean of signals.
#[derive(Clone)]
pub struct Context {
    signaler: Signaler<Perceptron>,
    dispatcher: Dispatcher,
    coordinator: Coordinator,
}

// -------------------------------------------------------------------------------------------------

impl Context {
    /// Context constructor.
    pub fn new(signaler: Signaler<Perceptron>,
               dispatcher: Dispatcher,
               coordinator: Coordinator)
               -> Self {
        Context {
            signaler: signaler,
            dispatcher: dispatcher,
            coordinator: coordinator,
        }
    }

    /// Emit signal with given `id` and `package` data.
    pub fn emit(&mut self, id: SignalId, package: Perceptron) {
        self.signaler.emit(id, package);
    }

    /// Add new event handler.
    pub fn add_event_handler(&mut self, event_handler: Box<EventHandler>) {
        self.dispatcher.add_source(event_handler);
    }

    /// Get reference to `Coordinator`.
    pub fn get_coordinator(&mut self) -> &mut Coordinator {
        &mut self.coordinator
    }
}

// -------------------------------------------------------------------------------------------------
