// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Device Manager.

// -------------------------------------------------------------------------------------------------

use dharma::{Context, InitResult, Module};
use qualia::perceptron::Perceptron;

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModule {
    i: i32,
}

// -------------------------------------------------------------------------------------------------

impl DeviceManagerModule {
    /// `DeviceManagerModule` constructor.
    pub fn new() -> Self {
        DeviceManagerModule { i: 5 }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for DeviceManagerModule {
    type T = Perceptron;

    #[allow(unused_variables)]
    fn initialize(&mut self, context: &mut Context<Self::T>) -> InitResult {
        Vec::new()
    }

    fn execute(&mut self, package: &Self::T) {
    }

    fn finalize(&mut self) {
    }
}

// -------------------------------------------------------------------------------------------------
