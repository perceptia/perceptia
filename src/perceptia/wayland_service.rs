// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides implementation of `dharma::Service` for Wayland related functionality.

// -------------------------------------------------------------------------------------------------

use std::time;
use std::any::Any;

use dharma;

use qualia::{perceptron, Perceptron, SurfaceViewer};

use wayland_frontend::{Engine, Gateway, constants};

use coordination::Context;

// -------------------------------------------------------------------------------------------------

/// Helper macro for printing warning when receiver message with data of wrong type.
macro_rules! warn_wrong {
    ($id:expr, $pkg:expr) => {
        log_warn1!("WaylandService: wrong data for message '{}' - {}", $id, $pkg);
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure binds `dharma` thread framework with `wayland_framework` functionality.
pub struct WaylandService {
    engine: Engine,
    context: Context,
    receiver: dharma::Receiver<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

/// Public methods.
impl WaylandService {
    /// Creates new `WaylandService`.
    pub fn new(mut context: Context) -> Self {
        dharma::system::block_signals();
        let engine = Engine::new(context.get_coordinator().clone(),
                                 context.get_settings().clone(),
                                 context.get_config().get_keyboard_config().clone());

        WaylandService {
            engine: engine,
            context: context,
            receiver: dharma::Receiver::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::Service for WaylandService {
    fn run(&mut self) {
        self.initialize();
        self.do_run();
        self.finalize();
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods.
impl WaylandService {
    /// Initializes `WaylandService`.
    fn initialize(&mut self) {
        self.context.get_signaler().register(&self.receiver);
        for s in vec![perceptron::DISPLAY_CREATED,
                      perceptron::OUTPUT_FOUND,
                      perceptron::INPUT_KEYBOARD,
                      perceptron::INPUT_POINTER_BUTTON,
                      perceptron::INPUT_POINTER_AXIS,
                      perceptron::SURFACE_FRAME,
                      perceptron::POINTER_FOCUS_CHANGED,
                      perceptron::POINTER_RELATIVE_MOTION,
                      perceptron::KEYBOARD_FOCUS_CHANGED,
                      perceptron::SURFACE_RECONFIGURED,
                      perceptron::TRANSFER_OFFERED,
                      perceptron::TRANSFER_REQUESTED,
                      perceptron::SCREENSHOT_DONE] {
            self.context.get_signaler().subscribe(s, &self.receiver);
        }
        log_info1!("Started Wayland service");
    }

    /// Runs main loop.
    #[inline]
    fn do_run(&mut self) {
        // NOTE: This is not best solution - alternatively listen to messages from the rest of
        // application and from Wayland clients. Maybe move receiving from `Engine` to global
        // `Dispatcher`?
        let mut sender = dharma::Sender::new();
        dharma::connect(&mut sender, &self.receiver);
        self.engine.start(sender);
        let d = time::Duration::from_millis(10);
        while self.listen(d) {
            self.engine.receive();
        }
    }

    /// Listens to messages from the rest of application.
    #[inline]
    fn listen(&mut self, duration: time::Duration) -> bool {
        match self.receiver.recv_timeout(duration) {
            dharma::ReceiveResult::Plain(package) => {
                self.execute_plain(package);
                true
            }
            dharma::ReceiveResult::Defined(_, package) => {
                self.execute_defined(package);
                true
            }
            dharma::ReceiveResult::Custom(id, package) => {
                self.execute_custom(id, package);
                true
            }
            dharma::ReceiveResult::Any(id, package) => {
                self.execute_direct(id, package);
                true
            }
            dharma::ReceiveResult::Special(command) => self.execute_special(command),
            dharma::ReceiveResult::Timeout => true,
            dharma::ReceiveResult::Empty => false,
            dharma::ReceiveResult::Err => false,
        }
    }

    /// Executes plain message.
    #[inline]
    fn execute_plain(&mut self, _package: Perceptron) {
        // Nothing to do.
    }

    /// Executes message with defined id.
    #[inline]
    fn execute_defined(&mut self, package: Perceptron) {
        match package {
            Perceptron::DisplayCreated(info) => {
                self.engine.on_display_created(info);
            }
            Perceptron::InputKeyboard(key) => {
                self.engine.on_keyboard_input(key, None);
            }
            Perceptron::InputPointerButton(btn) => {
                self.engine.on_pointer_button(btn);
            }
            Perceptron::InputPointerAxis(axis) => {
                self.engine.on_pointer_axis(axis);
            }
            Perceptron::SurfaceFrame(sid, milliseconds) => {
                self.engine.on_surface_frame(sid, milliseconds);
            }
            Perceptron::PointerFocusChanged(old_sid, new_sid, pos) => {
                self.engine.on_pointer_focus_changed(old_sid, new_sid, pos);
            }
            Perceptron::PointerRelativeMotion(sid, pos, time) => {
                self.engine.on_pointer_relative_motion(sid, pos, time);
            }
            Perceptron::KeyboardFocusChanged(old_sid, new_sid) => {
                self.engine.on_keyboard_focus_changed(old_sid, new_sid);
            }
            Perceptron::TransferOffered => {
                self.engine.on_transfer_offered();
            }
            Perceptron::TransferRequested(mime_type, fd) => {
                self.engine.on_transfer_requested(mime_type, fd);
            }
            Perceptron::SurfaceReconfigured(sid) => {
                if let Some(info) = self.context.get_coordinator().get_surface(sid) {
                    self.engine.on_surface_reconfigured(sid, info.desired_size, info.state_flags);
                }
            }
            Perceptron::ScreenshotDone => {
                self.engine.on_screenshot_done();
            }
            Perceptron::OutputFound(bundle) => self.engine.on_output_found(bundle),
            _ => {}
        }
    }

    /// Executes message with custom id.
    #[inline]
    fn execute_custom(&mut self, id: &'static str, package: Perceptron) {
        if id == constants::PROCESS_EVENTS {
            match package {
                Perceptron::CustomId(handler_id) => self.engine.process_events(handler_id),
                _ => warn_wrong!(constants::PROCESS_EVENTS, package),
            }
        } else if id == constants::HANDLE_NEW_CLIENT {
            let mut sender = dharma::DirectSender::new();
            dharma::direct_connect(&mut sender, &self.receiver);
            self.engine.handle_new_client(sender);
        } else if id == constants::TERMINATE_CLIENT {
            match package {
                Perceptron::CustomId(handler_id) => self.engine.terminate_client(handler_id),
                _ => warn_wrong!(constants::TERMINATE_CLIENT, package),
            }
        }
    }

    /// Executes direct message.
    #[inline]
    fn execute_direct(&mut self, _id: String, _any: Box<Any>) {
        // Nothing to do.
    }

    /// Handles special command.
    #[inline]
    fn execute_special(&mut self, command: dharma::SpecialCommand) -> bool {
        match command {
            dharma::SpecialCommand::Terminate => false,
        }
    }

    /// Finalizes service.
    fn finalize(&mut self) {
        log_info1!("Stopped Wayland service");
    }
}

// -------------------------------------------------------------------------------------------------

pub struct WaylandServiceConstructor {
    context: Context,
}

// -------------------------------------------------------------------------------------------------

impl WaylandServiceConstructor {
    /// Constructs new `WaylandServiceConstructor`.
    pub fn new(context: Context) -> Box<dharma::ServiceConstructor> {
        Box::new(WaylandServiceConstructor { context: context })
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::ServiceConstructor for WaylandServiceConstructor {
    fn construct(&self) -> Box<dharma::Service> {
        Box::new(WaylandService::new(self.context.clone()))
    }
}

// -------------------------------------------------------------------------------------------------
