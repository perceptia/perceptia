// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code common for all input drivers.

// -------------------------------------------------------------------------------------------------

use qualia::{perceptron, Perceptron, InputConfig, Button, Key, KeyState, OptionalPosition, Vector};
use dharma::Signaler;

// -------------------------------------------------------------------------------------------------

pub struct InputGateway {
    config: InputConfig,
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// `InputGateway` constructor.
    pub fn new(config: InputConfig, signaler: Signaler<Perceptron>) -> Self {
        InputGateway {
            config: config,
            signaler: signaler,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl InputGateway {
    /// Emit keyboards event.
    pub fn emit_key(&mut self, code: u16, value: i32) {
        // If no binding found inform the rest of the world
        let key = Key::new(code, value);
        self.signaler.emit(perceptron::INPUT_KEYBOARD, Perceptron::InputKeyboard(key));
    }

    /// Scale displacements and emit pointer motion event.
    pub fn emit_motion(&mut self, x: i32, y: i32) {
        // Scale event values
        let vector = Vector::new(x, y).scaled(self.config.mouse_scale);

        // Signal event
        self.signaler.emit(perceptron::INPUT_POINTER_MOTION,
                           Perceptron::InputPointerMotion(vector))
    }

    /// Scale position and emit pointer position event.
    pub fn emit_position(&mut self, x: Option<i32>, y: Option<i32>) {
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
    pub fn emit_axis(&mut self, horizontal: i32, vertical: i32) {
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
