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
        self.exhibitor = Some(Exhibitor::new(context.get_signaler().clone(),
                                             context.get_coordinator().clone()));
        vec![perceptron::NOTIFY,
             perceptron::PAGE_FLIP,
             perceptron::OUTPUT_FOUND,
             perceptron::INPUT_POINTER_MOTION,
             perceptron::INPUT_POINTER_POSITION,
             perceptron::INPUT_POINTER_BUTTON,
             perceptron::INPUT_POINTER_POSITION_RESET,
             perceptron::SURFACE_READY]
    }

    fn execute(&mut self, package: &Self::T) {
        if let Some(ref mut exhibitor) = self.exhibitor {
            match *package {
                Perceptron::Notify => exhibitor.on_notify(),
                Perceptron::OutputFound(bundle) => exhibitor.on_output_found(bundle),
                Perceptron::PageFlip(id) => exhibitor.on_pageflip(id),

                Perceptron::InputPointerMotion(ref vector) => exhibitor.on_motion(vector.clone()),
                Perceptron::InputPointerPosition(ref pos) => exhibitor.on_position(pos.clone()),
                Perceptron::InputPointerButton(ref btn) => exhibitor.on_button(btn.clone()),
                Perceptron::InputPointerPositionReset => exhibitor.on_position_reset(),

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
