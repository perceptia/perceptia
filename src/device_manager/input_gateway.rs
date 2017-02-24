// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code common for all input drivers.

// -------------------------------------------------------------------------------------------------

use uinput_sys;

use qualia::{perceptron, Perceptron, InputConfig, Button, Key, OptionalPosition, Vector};
use qualia::{modifier, InputManager, KeyCatchResult, KeyCode, KeyValue, KeyState};
use dharma::Signaler;

// -------------------------------------------------------------------------------------------------

pub struct InputGateway {
    modifiers: modifier::ModifierType,
    config: InputConfig,
    input_manager: InputManager,
    signaler: Signaler<Perceptron>,
    modifier_keys: Vec<(KeyCode, modifier::ModifierType)>,
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// `InputGateway` constructor.
    pub fn new(config: InputConfig,
               input_manager: InputManager,
               signaler: Signaler<Perceptron>)
               -> Self {
        InputGateway {
            modifiers: modifier::NONE,
            config: config,
            input_manager: input_manager,
            signaler: signaler,
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
    /// Emit keyboards event.
    pub fn emit_key(&mut self, code: u16, value: i32) {
        // Ignore repeats
        if (value != KeyState::Pressed as KeyValue) && (value != KeyState::Released as KeyValue) {
            return;
        }

        // Update modifiers
        if self.update_modifiers(code, value) != KeyCatchResult::Passed {
            return;
        }

        // Try to execute key binding
        if self.input_manager.catch_key(code, value, self.modifiers) == KeyCatchResult::Passed {
            // If no binding found inform the rest of the world
            let key = Key::new(code, value);
            self.signaler.emit(perceptron::INPUT_KEYBOARD, Perceptron::InputKeyboard(key));
        }
    }

    /// Scale displacements and emit pointer motion event.
    pub fn emit_motion(&mut self, x: isize, y: isize) {
        // Scale event values
        let vector = Vector::new(x, y).scaled(self.config.mouse_scale);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_MOTION,
                           Perceptron::InputPointerMotion(vector))
    }

    /// Scale position and emit pointer position event.
    pub fn emit_position(&mut self, x: Option<isize>, y: Option<isize>) {
        // Scale event values. Skip scaling invalid values
        let pos = OptionalPosition::new(x, y).scaled(self.config.touchpad_scale);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_POSITION,
                           Perceptron::InputPointerPosition(pos))
    }

    /// Emit button event.
    pub fn emit_button(&mut self, code: u16, value: i32) {
        let btn = Button::new(code, value);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_BUTTON,
                           Perceptron::InputPointerButton(btn))
    }

    /// Emit exist event.
    pub fn emit_axis(&mut self, horizontal: isize, vertical: isize) {
        let axis = Vector::new(horizontal, vertical);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_AXIS,
                           Perceptron::InputPointerAxis(axis))
    }

    /// Emit position reset event.
    pub fn emit_position_reset(&mut self) {
        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_POSITION_RESET,
                           Perceptron::InputPointerPositionReset)
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
}

// -------------------------------------------------------------------------------------------------
