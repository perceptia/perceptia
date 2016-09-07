// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Wayland Frontend.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, Perceptron};
use wayland_frontend;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Device Manager.
pub struct WaylandModule {
    i: i32,
}

// -------------------------------------------------------------------------------------------------

impl WaylandModule {
    /// `WaylandModule` constructor.
    pub fn new() -> Self {
        WaylandModule { i: 9 }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for WaylandModule {
    type T = Perceptron;
    type C = Context;

    #[allow(unused_variables)]
    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        log_info1!("Started Wayland module");
        wayland_frontend::WaylandFrontend::init(context.get_coordinator());
        Vec::new()
    }

    #[allow(unused_variables)]
    fn execute(&mut self, package: &Self::T) {}

    fn finalize(&mut self) {}
}

// -------------------------------------------------------------------------------------------------
