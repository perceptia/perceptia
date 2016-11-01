// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Wayland Frontend.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module};
use qualia::{Context, perceptron, Perceptron, Size};
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
             perceptron::INPUT_POINTER_BUTTON,
             perceptron::INPUT_POINTER_AXIS,
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
            Perceptron::InputPointerButton(ref btn) => {
                WaylandFrontend::on_pointer_button(btn.clone());
            }
            Perceptron::InputPointerAxis(ref axis) => {
                WaylandFrontend::on_pointer_axis(axis.clone());
            }
            Perceptron::SurfaceFrame(sid) => WaylandFrontend::on_surface_frame(sid),
            Perceptron::PointerFocusChanged(ref surface_position) => {
                WaylandFrontend::on_pointer_focus_changed(surface_position.clone())
            }
            Perceptron::PointerRelativeMotion(ref surface_position) => {
                WaylandFrontend::on_pointer_relative_motion(surface_position.clone())
            }
            Perceptron::KeyboardFocusChanged(old_sid, new_sid) => {
                if let Some(ref mut context) = self.context {
                    let (old_size, old_flags) =
                    if let Some(info) = context.get_coordinator().get_surface(old_sid) {
                        (info.desired_size, info.state_flags as u32)
                    } else {
                        (Size::default(), 0)
                    };
                    let (new_size, new_flags) =
                    if let Some(info) = context.get_coordinator().get_surface(new_sid) {
                        (info.desired_size, info.state_flags as u32)
                    } else {
                        (Size::default(), 0)
                    };
                    WaylandFrontend::on_keyboard_focus_changed(old_sid, old_size, old_flags,
                                                               new_sid, new_size, new_flags);
                }
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
