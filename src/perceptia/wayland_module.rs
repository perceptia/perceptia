// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Wayland Frontend.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, perceptron, Perceptron};
use wayland_frontend::WaylandFrontend;

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
    fn initialize(&mut self, context: Self::C) -> InitResult {
        log_info1!("Started Wayland module");
        // TODO: Simplify when Wayland part is rewritten in Rust.
        self.context = Some(context);
        if let Some(ref mut context) = self.context {
            let mut keymap = context.get_settings().get_keymap();
            WaylandFrontend::init(context.get_coordinator(), &mut keymap);
        }
        vec![perceptron::OUTPUT_FOUND,
             perceptron::INPUT_KEYBOARD,
             perceptron::SURFACE_FRAME,
             perceptron::POINTER_FOCUS_CHANGED,
             perceptron::POINTER_RELATIVE_MOTION,
             perceptron::KEYBOARD_FOCUS_CHANGED,
             perceptron::SURFACE_RECONFIGURED]
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::OutputFound(_) => WaylandFrontend::on_output_found(),
            Perceptron::InputKeyboard(ref key) => WaylandFrontend::on_keyboard_input(key.clone()),
            Perceptron::SurfaceFrame(sid) => WaylandFrontend::on_surface_frame(sid),
            Perceptron::PointerFocusChanged(ref surface_position) => {
                WaylandFrontend::on_pointer_focus_changed(surface_position.clone())
            }
            Perceptron::PointerRelativeMotion(ref surface_position) => {
                WaylandFrontend::on_pointer_relative_motion(surface_position.clone())
            }
            Perceptron::KeyboardFocusChanged(sid) => {
                WaylandFrontend::on_keyboard_focus_changed(sid)
            }
            Perceptron::SurfaceReconfigured(sid) => {
                if let Some(ref mut context) = self.context {
                    if let Some(info) = context.get_coordinator().get_surface(sid) {
                        WaylandFrontend::on_surface_reconfigured(sid,
                                                                 info.desired_size,
                                                                 info.state_flags as u32);
                    }
                }
            }
            _ => {}
        }
    }

    fn finalize(&mut self) {}
}

// -------------------------------------------------------------------------------------------------
