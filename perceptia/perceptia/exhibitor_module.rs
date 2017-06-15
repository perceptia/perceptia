// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Exhibitor.

// -------------------------------------------------------------------------------------------------

use dharma::{Module, ModuleConstructor, SignalId};
use qualia::{OutputType, perceptron, Perceptron};
use outputs::{DrmOutput, VirtualOutput};
use coordination::{Context, Coordinator};
use exhibitor::{Exhibitor, Strategist};

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Exhibitor.
pub struct ExhibitorModule {
    last_output_id: i32,
    exhibitor: Exhibitor<Coordinator>,
}

// -------------------------------------------------------------------------------------------------

impl ExhibitorModule {
    /// Constructs new `ExhibitorModule`.
    pub fn new(context: &mut Context) -> Self {
        let coordinator = context.get_coordinator().clone();
        let config = context.get_config().get_exhibitor_config();
        ExhibitorModule {
            last_output_id: 0,
            exhibitor: Exhibitor::new(coordinator,
                                      context.get_reference_time(),
                                      Strategist::new_from_config(config.strategist.clone()),
                                      config.compositor.clone()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for ExhibitorModule {
    type T = Perceptron;
    type C = Context;

    fn get_signals(&self) -> Vec<SignalId> {
        vec![perceptron::NOTIFY,
             perceptron::SUSPEND,
             perceptron::WAKEUP,
             perceptron::PAGE_FLIP,
             perceptron::OUTPUT_FOUND,
             perceptron::COMMAND,
             perceptron::MODE,
             perceptron::INPUT_POINTER_MOTION,
             perceptron::INPUT_POINTER_POSITION,
             perceptron::INPUT_POINTER_BUTTON,
             perceptron::INPUT_POINTER_POSITION_RESET,
             perceptron::CURSOR_SURFACE_CHANGE,
             perceptron::BACKGROUND_SURFACE_CHANGE,
             perceptron::SURFACE_READY,
             perceptron::DOCK_SURFACE,
             perceptron::SURFACE_DESTROYED,
             perceptron::KEYBOARD_FOCUS_CHANGED,
             perceptron::TAKE_SCREENSHOT]
    }

    fn initialize(&mut self) {
        log_info1!("Exhibitor module initialized");
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::Notify => self.exhibitor.on_notify(),
            Perceptron::OutputFound(ref bundle) => self.on_output_found(bundle.clone()),
            Perceptron::PageFlip(id) => self.exhibitor.on_pageflip(id),
            Perceptron::Command(ref command) => self.exhibitor.on_command(command.clone()),

            Perceptron::InputPointerMotion(vector) => self.exhibitor.on_motion(vector),
            Perceptron::InputPointerPosition(position) => self.exhibitor.on_position(position),
            Perceptron::InputPointerButton(button) => self.exhibitor.on_button(button),
            Perceptron::InputPointerPositionReset(position) => {
                self.exhibitor.on_position_reset(position);
            }
            Perceptron::CursorSurfaceChange(sid) => self.exhibitor.on_cursor_surface_change(sid),

            Perceptron::SurfaceReady(sid) => self.exhibitor.on_surface_ready(sid),
            Perceptron::DockSurface(sid, size, display_id) => {
                self.exhibitor.on_dock_surface(sid, size, display_id);
            }
            Perceptron::SurfaceDestroyed(sid) => self.exhibitor.on_surface_destroyed(sid),

            Perceptron::KeyboardFocusChanged(_, sid) => {
                self.exhibitor.on_keyboard_focus_changed(sid);
            }

            Perceptron::Mode { active, mode } => {
                self.exhibitor.on_mode_switched(active, mode);
            }

            Perceptron::Suspend => self.exhibitor.on_suspend(),
            Perceptron::WakeUp => self.exhibitor.on_wakeup(),
            Perceptron::TakeScreenshot(id) => self.exhibitor.take_screenshot(id),
            Perceptron::BackgroundSurfaceChange(sid) => {
                self.exhibitor.on_background_surface_change(sid);
            }
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Exhibitor module finalized");
    }
}

// -------------------------------------------------------------------------------------------------

// Event handling helpers
impl ExhibitorModule {
    /// Helper method for handling new output.
    ///
    /// For unit testing construction of the output must be done outside `Exhibitor`.
    fn on_output_found(&mut self, output_type: OutputType) {
        self.last_output_id += 1;
        match output_type {
            OutputType::Drm(bundle) => {
                match DrmOutput::new(bundle, self.last_output_id) {
                    Ok(output) => {
                        log_info2!("Created DRM output: {}", output.get_info().make);
                        self.exhibitor.on_output_found(output);
                    }
                    Err(err) => {
                        log_error!("Could not create DRM output: {}", err);
                        return;
                    }
                }
            }
            OutputType::Virtual(bundle) => {
                match VirtualOutput::new(bundle, self.last_output_id) {
                    Ok(output) => {
                        log_info2!("Created virtual output: {}", output.get_info().make);
                        self.exhibitor.on_output_found(output);
                    }
                    Err(err) => {
                        log_error!("Could not create virtual output: {}", err);
                        return;
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ExhibitorModuleConstructor {}

// -------------------------------------------------------------------------------------------------

impl ExhibitorModuleConstructor {
    /// Constructs new `ExhibitorModuleConstructor`.
    pub fn new() -> Box<ModuleConstructor<T = Perceptron, C = Context>> {
        Box::new(ExhibitorModuleConstructor {})
    }
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructor for ExhibitorModuleConstructor {
    type T = Perceptron;
    type C = Context;

    fn construct(&self, context: &mut Self::C) -> Box<Module<T = Self::T, C = Self::C>> {
        Box::new(ExhibitorModule::new(context))
    }
}

// -------------------------------------------------------------------------------------------------
