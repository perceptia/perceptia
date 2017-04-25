// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Aesthetics.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module, ModuleConstructor};
use qualia::{perceptron, Perceptron};
use coordination::{Context, Coordinator};
use aesthetics::Aesthetics;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Aesthetics.
pub struct AestheticsModule {
    aesthetics: Aesthetics<Coordinator>,
}

// -------------------------------------------------------------------------------------------------

impl AestheticsModule {
    /// Constructs new `AestheticsModule`.
    pub fn new(context: &mut Context) -> Self {
        AestheticsModule {
            aesthetics: Aesthetics::new(context.get_coordinator().clone(),
                                        context.get_config().get_aesthetics_config().clone()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for AestheticsModule {
    type T = Perceptron;
    type C = Context;

    fn initialize(&mut self) -> InitResult {
        log_info1!("Starting Aesthetics module");
        vec![perceptron::DISPLAY_CREATED,
             perceptron::CURSOR_SURFACE_CHANGE,
             perceptron::BACKGROUND_SURFACE_CHANGE,
             perceptron::POINTER_FOCUS_CHANGED,
             perceptron::SURFACE_DESTROYED]
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::CursorSurfaceChange(sid) => self.aesthetics.on_cursor_surface_change(sid),
            Perceptron::PointerFocusChanged(old_pfsid, new_pfsid, _) => {
                self.aesthetics.on_pointer_focus_changed(old_pfsid, new_pfsid);
            }
            Perceptron::SurfaceDestroyed(sid) => self.aesthetics.on_surface_destroyed(sid),
            Perceptron::DisplayCreated(_) => self.aesthetics.on_display_created(),
            Perceptron::BackgroundSurfaceChange(sid) => {
                self.aesthetics.on_background_surface_change(sid);
            }
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Finalized Aesthetics module");
    }
}

// -------------------------------------------------------------------------------------------------

pub struct AestheticsModuleConstructor {}

// -------------------------------------------------------------------------------------------------

impl AestheticsModuleConstructor {
    /// Constructs new `AestheticsModuleConstructor`.
    pub fn new() -> Box<ModuleConstructor<T = Perceptron, C = Context>> {
        Box::new(AestheticsModuleConstructor {})
    }
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructor for AestheticsModuleConstructor {
    type T = Perceptron;
    type C = Context;

    fn construct(&self, context: &mut Self::C) -> Box<Module<T = Self::T, C = Self::C>> {
        Box::new(AestheticsModule::new(context))
    }
}

// -------------------------------------------------------------------------------------------------
