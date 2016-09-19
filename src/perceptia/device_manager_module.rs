// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Device Manager.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, perceptron, Perceptron};
use device_manager::DeviceManager;

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModule<'a> {
    manager: Option<DeviceManager<'a>>,
}

// -------------------------------------------------------------------------------------------------

impl<'a> DeviceManagerModule<'a> {
    /// `DeviceManagerModule` constructor.
    pub fn new() -> Self {
        DeviceManagerModule { manager: None }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Module for DeviceManagerModule<'a> {
    type T = Perceptron;
    type C = Context;

    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        self.manager = Some(DeviceManager::new(context));
        Vec::new()
    }

    // FIXME: Finnish handling signals in `DeviceManagerModule`.
    fn execute(&mut self, package: &Self::T) {}

    fn finalize(&mut self) {
        log_info1!("Finalized Device Manager module");
    }
}

// -------------------------------------------------------------------------------------------------
