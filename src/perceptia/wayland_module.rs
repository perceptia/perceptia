// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Wayland Frontend.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, perceptron, Perceptron};
use wayland_frontend;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Device Manager.
pub struct WaylandModule {
    context: Option<Context>,
}

// -------------------------------------------------------------------------------------------------

impl WaylandModule {
    /// `WaylandModule` constructor.
    pub fn new() -> Self {
        WaylandModule { context: None }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for WaylandModule {
    type T = Perceptron;
    type C = Context;

    #[allow(unused_variables)]
    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        log_info1!("Started Wayland module");
        // TODO: Simplify when Wayland part is rewritten in Rust.
        self.context = Some(context);
        if let Some(ref mut context) = self.context {
            wayland_frontend::WaylandFrontend::init(context.get_coordinator());
        }
        vec![perceptron::OUTPUT_FOUND]
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::OutputFound(_) => wayland_frontend::WaylandFrontend::on_output_found(),
            _ => {}
        }
    }

    fn finalize(&mut self) {}
}

// -------------------------------------------------------------------------------------------------
