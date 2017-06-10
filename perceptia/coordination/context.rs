// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains application context used to share data between threads.

// -------------------------------------------------------------------------------------------------

use std::time::Instant;

use dharma::{DispatcherController, EventHandler, EventKind, Signaler, SignalId};

use qualia::{Settings, Perceptron};
use gears::Config;

use coordinator::Coordinator;

// -------------------------------------------------------------------------------------------------

/// Application `Context` lets `Module`s communicate with other `Module`s by mean of signals.
#[derive(Clone)]
pub struct Context {
    config: Config,
    settings: Settings,
    signaler: Signaler<Perceptron>,
    dispatcher: DispatcherController,
    coordinator: Coordinator,
    reference_time: Instant,
}

// -------------------------------------------------------------------------------------------------

impl Context {
    /// Context constructor.
    pub fn new(config: Config,
               settings: Settings,
               signaler: Signaler<Perceptron>,
               dispatcher: DispatcherController,
               coordinator: Coordinator)
               -> Self {
        Context {
            config: config,
            settings: settings,
            signaler: signaler,
            dispatcher: dispatcher,
            coordinator: coordinator,
            reference_time: Instant::now(),
        }
    }

    /// Emits signal with given `id` and `package` data.
    pub fn emit(&mut self, id: SignalId, package: Perceptron) {
        self.signaler.emit(id, package);
    }

    /// Adds new event handler.
    pub fn add_event_handler(&mut self,
                             event_handler: Box<EventHandler + Send>,
                             event_kind: EventKind) {
        self.dispatcher.add_source(event_handler, event_kind);
    }

    /// Returns global configuration.
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Returns global settings.
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    /// Returns reference to `Signaler`.
    pub fn get_signaler(&mut self) -> &mut Signaler<Perceptron> {
        &mut self.signaler
    }

    /// Returns reference to `DispatcherController`.
    pub fn get_dispatcher(&mut self) -> &mut DispatcherController {
        &mut self.dispatcher
    }

    /// Returns reference to `Coordinator`.
    pub fn get_coordinator(&mut self) -> &mut Coordinator {
        &mut self.coordinator
    }

    /// Returns `Instant` created during construction used as reference time for obtaining
    /// timestamps for events.
    pub fn get_reference_time(&self) -> Instant {
        self.reference_time
    }
}

// -------------------------------------------------------------------------------------------------
