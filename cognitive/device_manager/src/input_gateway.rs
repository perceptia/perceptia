// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code common for all input drivers.

// -------------------------------------------------------------------------------------------------

use qualia::{modifier, CatchResult, InputCode, InputValue, KeyState, Position};
use qualia::{InputForwarding, InputHandling};
use inputs::{codes, ModState};

// For built-in VT switching
use virtual_terminal::VirtualTerminal;

// -------------------------------------------------------------------------------------------------

/// Common functionality for input drivers.
///
/// `InputGateway` of a gateway for input drivers to the rest of application. Drivers pass input
/// events here and `InputGateway` handles them by catching build-in key bindings (like switching
/// virtual terminal) or consulting (implementation-dependent) `InputHandler` which may catch them
/// as used-defined bindings. If neither caught the event it is emitted to the rest of application
/// using `InputForwarder`.
pub struct InputGateway {
    modifiers: ModState,
    handler: Box<InputHandling>,
    forwarder: Box<InputForwarding>,
    vt: Option<VirtualTerminal>,
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// `InputGateway` constructor.
    pub fn new(handler: Box<InputHandling>,
               forwarder: Box<InputForwarding>,
               vt: Option<VirtualTerminal>)
               -> Self {
        InputGateway {
            modifiers: ModState::new(),
            handler: handler,
            forwarder: forwarder,
            vt: vt,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl InputForwarding for InputGateway {
    /// Emits keyboards event.
    fn emit_key(&mut self, code: u16, value: i32) {
        // Ignore repeats
        if (value != KeyState::Pressed as InputValue) &&
           (value != KeyState::Released as InputValue) {
            return;
        }

        // Update modifiers
        if self.modifiers.update(code, value) != CatchResult::Passed {
            return;
        }

        // Catch built-in key bindings
        if self.catch_key(code, value) != CatchResult::Passed {
            return;
        }

        // Try to execute key binding
        if self.handler.catch_key(code, value, self.modifiers.get()) == CatchResult::Passed {
            self.forwarder.emit_key(code, value);
        }
    }

    /// Scales displacements and emits pointer motion event.
    fn emit_motion(&mut self, x: isize, y: isize) {
        self.forwarder.emit_motion(x, y);
    }

    /// Scales position and emits pointer position event.
    fn emit_position(&mut self, x: Option<isize>, y: Option<isize>) {
        self.forwarder.emit_position(x, y);
    }

    /// Emits button event.
    fn emit_button(&mut self, code: u16, value: i32) {
        // Try to execute button binding
        if self.handler.catch_button(code, value, self.modifiers.get()) == CatchResult::Passed {
            self.forwarder.emit_button(code, value);
        }
    }

    /// Emits exist event.
    fn emit_axis(&mut self, horizontal: isize, vertical: isize) {
        self.forwarder.emit_axis(horizontal, vertical);
    }

    /// Emits position reset event.
    fn emit_position_reset(&mut self, position: Option<Position>) {
        self.forwarder.emit_position_reset(position);
    }

    /// Emits system activity event.
    fn emit_system_activity_event(&mut self) {
        self.forwarder.emit_system_activity_event();
    }
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// Helper method for executing built-in key bindings.
    fn catch_key(&self, code: InputCode, value: InputValue) -> CatchResult {
        if (codes::KEY_F1 <= code) && (code <= codes::KEY_F12) {
            if (self.modifiers.get() == (modifier::LALT | modifier::LCTL)) ||
               (self.modifiers.get() == (modifier::LALT | modifier::RCTL)) ||
               (self.modifiers.get() == (modifier::RALT | modifier::LCTL)) ||
               (self.modifiers.get() == (modifier::RALT | modifier::RCTL)) {
                if value == KeyState::Pressed as InputValue {
                    self.switch_vt((code + 1 - codes::KEY_F1) as i32);
                }
                return CatchResult::Caught;
            }
        }

        CatchResult::Passed
    }

    /// Helper method for switching virtual terminals.
    fn switch_vt(&self, num: i32) {
        log_info1!("Switching to virtual terminal {}", num);
        if let Some(vt) = self.vt {
            if let Err(err) = vt.switch_to(num as u8) {
                log_warn1!("Failed to switch terminals: {:?}", err);
            }
        } else {
            log_warn1!("Virtual terminals were not set up properly");
        }
    }
}

// -------------------------------------------------------------------------------------------------
