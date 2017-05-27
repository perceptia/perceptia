// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Aesthetics.

// -------------------------------------------------------------------------------------------------

use dharma::{Module, ModuleConstructor, SignalId};
use qualia::{perceptron, Perceptron};
use coordination::{Context, Coordinator};
use aesthetics::Aesthetics;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Aesthetics.
pub struct AestheticsModule<'a> {
    aesthetics: Aesthetics<'a, Coordinator>,
}

// -------------------------------------------------------------------------------------------------

impl<'a> AestheticsModule<'a> {
    /// Constructs new `AestheticsModule`.
    pub fn new(context: &mut Context) -> Self {
        AestheticsModule {
            aesthetics: Aesthetics::new(context.get_coordinator().clone(),
                                        context.get_config().get_aesthetics_config().clone()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Module for AestheticsModule<'a> {
    type T = Perceptron;
    type C = Context;

    fn get_signals(&self) -> Vec<SignalId> {
        vec![perceptron::DISPLAY_CREATED,
             perceptron::CURSOR_SURFACE_CHANGE,
             perceptron::BACKGROUND_SURFACE_CHANGE,
             perceptron::POINTER_FOCUS_CHANGED,
             perceptron::SURFACE_DESTROYED]
    }

    fn initialize(&mut self) {
        log_info1!("Aesthetics module initialized");
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::CursorSurfaceChange(sid) => self.aesthetics.on_cursor_surface_change(sid),
            Perceptron::PointerFocusChanged(old_pfsid, new_pfsid, _) => {
                self.aesthetics.on_pointer_focus_changed(old_pfsid, new_pfsid);
            }
            Perceptron::SurfaceDestroyed(sid) => self.aesthetics.on_surface_destroyed(sid),
            Perceptron::DisplayCreated(ref output) => self.aesthetics.on_display_created(output),
            Perceptron::BackgroundSurfaceChange(sid) => {
                self.aesthetics.on_background_surface_change(sid);
            }
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Aesthetics module finalized");
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
