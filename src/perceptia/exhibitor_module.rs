// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Exhibitor.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, perceptron, Perceptron};
use exhibitor::Exhibitor;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Device Manager.
pub struct ExhibitorModule {
    exhibitor: Option<Exhibitor>,
}

// -------------------------------------------------------------------------------------------------

impl ExhibitorModule {
    /// `ExhibitorModule` constructor.
    pub fn new() -> Self {
        ExhibitorModule { exhibitor: None }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for ExhibitorModule {
    type T = Perceptron;
    type C = Context;

    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        log_info1!("Starting Exhibitor module");
        self.exhibitor = Some(Exhibitor::new(context.get_coordinator().clone()));
        vec![perceptron::SURFACE_READY]
    }

    fn execute(&mut self, package: &Self::T) {
        if let Some(ref mut exhibitor) = self.exhibitor {
            match *package {
                Perceptron::SurfaceReady(sid) => exhibitor.on_surface_ready(sid),
                _ => {}
            }
        }
    }

    fn finalize(&mut self) {
        log_info1!("Finalized Exhibitor module");
    }
}

// -------------------------------------------------------------------------------------------------
