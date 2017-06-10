// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions for various input related functionalities.

// -------------------------------------------------------------------------------------------------

use timing;
use defs::{Slide, Vector};

// -------------------------------------------------------------------------------------------------

pub type Key = Button;

pub type InputCode = u16;
pub type InputValue = i32;

// -------------------------------------------------------------------------------------------------

/// These flags describe key modifiers.
pub mod modifier {
    pub type ModifierType = u16;
    pub const NONE: ModifierType = 0b00000000;
    pub const LCTL: ModifierType = 0b00000001;
    pub const RCTL: ModifierType = 0b00000010;
    pub const LSHF: ModifierType = 0b00000100;
    pub const RSHF: ModifierType = 0b00001000;
    pub const LALT: ModifierType = 0b00010000;
    pub const RALT: ModifierType = 0b00100000;
    pub const LMTA: ModifierType = 0b01000000;
    pub const RMTA: ModifierType = 0b10000000;
    pub const CTRL: ModifierType = LCTL | RCTL;
    pub const SHIFT: ModifierType = LSHF | RSHF;
    pub const ALT: ModifierType = LALT | RALT;
    pub const META: ModifierType = LMTA | RMTA;
}

// -------------------------------------------------------------------------------------------------

/// Enumeration for possible results of catching key.
#[derive(PartialEq)]
pub enum CatchResult {
    Caught,
    Passed,
}

// -------------------------------------------------------------------------------------------------

/// Structure for identifying key binding.
///
/// Used as key in hash maps.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Binding {
    code: InputCode,
    modifiers: modifier::ModifierType,
}

// -------------------------------------------------------------------------------------------------

impl Binding {
    /// Constructs new `Binding`.
    ///
    /// `uinput_sys` defines codes as `i32` so second constructor added to avoid casting in other
    /// places.
    pub fn new(code: i32, modifiers: modifier::ModifierType) -> Self {
        Binding {
            code: code as InputCode,
            modifiers: modifiers,
        }
    }

    /// `Binding` constructor.
    pub fn create(code: InputCode, modifiers: modifier::ModifierType) -> Self {
        Binding {
            code: code,
            modifiers: modifiers,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Data for button event.
#[derive(Clone, Copy, Debug)]
pub struct Button {
    pub code: u16,
    pub value: i32,
    pub time: timing::Milliseconds,
}

// -------------------------------------------------------------------------------------------------

impl Button {
    /// Constructs `Button`.
    pub fn new(code: u16, value: i32, milliseconds: timing::Milliseconds) -> Self {
        Button {
            code: code,
            value: value,
            time: milliseconds,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Data for axis event.
#[derive(Clone, Copy, Debug)]
pub struct Axis {
    pub discrete: Vector,
    pub continuous: Slide,
    pub time: timing::Milliseconds,
}

// -------------------------------------------------------------------------------------------------

impl Axis {
    pub fn new(discrete: Vector, continuous: Slide, time: timing::Milliseconds) -> Self {
        Axis {
            discrete: discrete,
            continuous: continuous,
            time: time,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait implemented by handlers of input events like key strokes.
pub trait InputHandling: Send {
    /// Catches and processes the keyboard event.
    fn catch_key(&mut self,
                 code: InputCode,
                 value: InputValue,
                 modifiers: modifier::ModifierType)
                 -> CatchResult;

    /// Catches and processes the button event.
    fn catch_button(&mut self,
                    code: InputCode,
                    value: InputValue,
                    modifiers: modifier::ModifierType)
                    -> CatchResult;

    /// Clones the instance of `InputHandling`.
    fn duplicate(&self) -> Box<InputHandling>;
}

// -------------------------------------------------------------------------------------------------

/// Trait defining interface for input drivers to access the application.
pub trait InputForwarding: Send {
    /// Emits key event.
    fn emit_key(&mut self, code: u16, value: i32);

    /// Emits pointer motion event.
    fn emit_motion(&mut self, x: isize, y: isize);

    /// Emits pointer position event.
    fn emit_position(&mut self, x: Option<isize>, y: Option<isize>);

    /// Emits button event.
    fn emit_button(&mut self, code: u16, value: i32);

    /// Emits exist event.
    fn emit_axis(&mut self, horizontal: isize, vertical: isize);

    /// Emits position reset event.
    fn emit_position_reset(&mut self);

    /// Emits system activity event.
    fn emit_system_activity_event(&mut self);
}

// -------------------------------------------------------------------------------------------------
