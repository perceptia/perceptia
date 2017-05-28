// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code common for all input drivers.

// -------------------------------------------------------------------------------------------------

use uinput_sys;

use dharma::Signaler;
use qualia::{perceptron, Perceptron, InputConfig};
use qualia::{Axis, Button, Key, OptionalPosition, Slide, Vector};
use qualia::{modifier, KeyCode, KeyValue, KeyState};

use gears::{InputManager, KeyCatchResult};

// For built-in VT switching
use virtual_terminal::VirtualTerminal;

// -------------------------------------------------------------------------------------------------

pub struct InputGateway {
    modifiers: modifier::ModifierType,
    config: InputConfig,
    input_manager: InputManager,
    signaler: Signaler<Perceptron>,
    vt: Option<VirtualTerminal>,
    modifier_keys: Vec<(KeyCode, modifier::ModifierType)>,
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// `InputGateway` constructor.
    pub fn new(config: InputConfig,
               input_manager: InputManager,
               signaler: Signaler<Perceptron>,
               vt: Option<VirtualTerminal>)
               -> Self {
        InputGateway {
            modifiers: modifier::NONE,
            config: config,
            input_manager: input_manager,
            signaler: signaler,
            vt: vt,
            modifier_keys: vec![(uinput_sys::KEY_LEFTCTRL as KeyCode, modifier::LCTL),
                                (uinput_sys::KEY_RIGHTCTRL as KeyCode, modifier::RCTL),
                                (uinput_sys::KEY_LEFTSHIFT as KeyCode, modifier::LSHF),
                                (uinput_sys::KEY_RIGHTSHIFT as KeyCode, modifier::RSHF),
                                (uinput_sys::KEY_LEFTALT as KeyCode, modifier::LALT),
                                (uinput_sys::KEY_RIGHTALT as KeyCode, modifier::RALT),
                                (uinput_sys::KEY_LEFTMETA as KeyCode, modifier::LMTA),
                                (uinput_sys::KEY_RIGHTMETA as KeyCode, modifier::RMTA)],
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// Emits keyboards event.
    pub fn emit_key(&mut self, code: u16, value: i32) {
        // Ignore repeats
        if (value != KeyState::Pressed as KeyValue) && (value != KeyState::Released as KeyValue) {
            return;
        }

        // Update modifiers
        if self.update_modifiers(code, value) != KeyCatchResult::Passed {
            return;
        }

        // Catch built-in key bindings
        if self.catch_key(code, value) != KeyCatchResult::Passed {
            return;
        }

        // Try to execute key binding
        if self.input_manager.catch_key(code, value, self.modifiers) == KeyCatchResult::Passed {
            // If no binding found inform the rest of the world
            let key = Key::new_now(code, value);
            self.signaler.emit(perceptron::INPUT_KEYBOARD, Perceptron::InputKeyboard(key));
        }
    }

    /// Scales displacements and emits pointer motion event.
    pub fn emit_motion(&mut self, x: isize, y: isize) {
        // Scale event values
        let vector = Vector::new(x, y).scaled(self.config.mouse_scale as f32);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_MOTION,
                           Perceptron::InputPointerMotion(vector));
    }

    /// Scales position and emits pointer position event.
    pub fn emit_position(&mut self, x: Option<isize>, y: Option<isize>) {
        // Scale event values. Skip scaling invalid values
        let pos = OptionalPosition::new(x, y).scaled(self.config.touchpad_scale as f32);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_POSITION,
                           Perceptron::InputPointerPosition(pos));
    }

    /// Emits button event.
    pub fn emit_button(&mut self, code: u16, value: i32) {
        let btn = Button::new_now(code, value);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_BUTTON, Perceptron::InputPointerButton(btn));
    }

    /// Emits exist event.
    pub fn emit_axis(&mut self, horizontal: isize, vertical: isize) {
        let axis = Axis::new_now(Vector::new(horizontal, vertical),
                                 Slide::new(10.0 * horizontal as f32, 10.0 * vertical as f32));

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_AXIS, Perceptron::InputPointerAxis(axis));
    }

    /// Emits position reset event.
    pub fn emit_position_reset(&mut self) {
        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_POSITION_RESET,
                           Perceptron::InputPointerPositionReset);
    }

    /// Emits system activity event.
    pub fn emit_system_activity_event(&mut self) {
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// Helper method for updating modifiers.
    fn update_modifiers(&mut self, code: KeyCode, value: KeyValue) -> KeyCatchResult {
        let mut result = KeyCatchResult::Passed;
        for &(modifier_code, modifier_flag) in self.modifier_keys.iter() {
            if code == modifier_code {
                if value == KeyState::Pressed as KeyValue {
                    if (self.modifiers & modifier_flag) != 0x0 {
                        result = KeyCatchResult::Caught;
                    } else {
                        self.modifiers |= modifier_flag;
                    }
                } else {
                    self.modifiers &= !modifier_flag;
                }
                break;
            }
        }
        result
    }

    /// Helper method for executing built-in key bindings.
    fn catch_key(&self, code: KeyCode, value: KeyValue) -> KeyCatchResult {
        let code = code as i32;
        if (uinput_sys::KEY_F1 <= code) && (code <= uinput_sys::KEY_F12) {
            if (self.modifiers == (modifier::LALT | modifier::LCTL)) ||
               (self.modifiers == (modifier::LALT | modifier::RCTL)) ||
               (self.modifiers == (modifier::RALT | modifier::LCTL)) ||
               (self.modifiers == (modifier::RALT | modifier::RCTL)) {
                if value == KeyState::Pressed as KeyValue {
                    self.switch_vt(code + 1 - uinput_sys::KEY_F1);
                }
                return KeyCatchResult::Caught;
            }
        }

        KeyCatchResult::Passed
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
