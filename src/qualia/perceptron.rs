// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions of events used in whole application.

// -------------------------------------------------------------------------------------------------

use std;

use dharma::{SignalId, Transportable};

use defs::{Command, DrmBundle, SurfaceId, OptionalPosition, SurfacePosition, Vector, Button, Key};

// -------------------------------------------------------------------------------------------------

pub const NOTIFY: SignalId = 0;
pub const VERTICAL_BLANK: SignalId = 1;
pub const PAGE_FLIP: SignalId = 2;
pub const OUTPUT_FOUND: SignalId = 3;
pub const COMMAND: SignalId = 5;
pub const INPUT_POINTER_MOTION: SignalId = 10;
pub const INPUT_POINTER_POSITION: SignalId = 11;
pub const INPUT_POINTER_BUTTON: SignalId = 12;
pub const INPUT_POINTER_AXIS: SignalId = 13;
pub const INPUT_POINTER_POSITION_RESET: SignalId = 14;
pub const INPUT_KEYBOARD: SignalId = 15;
pub const SURFACE_READY: SignalId = 20;
pub const SURFACE_DESTROYED: SignalId = 21;
pub const SURFACE_RECONFIGURED: SignalId = 22;
pub const CURSOR_SURFACE_CHANGE: SignalId = 25;
pub const SURFACE_FRAME: SignalId = 30;
pub const POINTER_FOCUS_CHANGED: SignalId = 31;
pub const POINTER_RELATIVE_MOTION: SignalId = 32;
pub const KEYBOARD_FOCUS_CHANGED: SignalId = 33;

// -------------------------------------------------------------------------------------------------

/// Data passed along with signals. Convention it to use enum values only with corresponding signal
/// identifies.
#[repr(C)]
#[derive(Clone)]
pub enum Perceptron {
    Notify,
    VerticalBlank(i32),
    PageFlip(i32),
    OutputFound(DrmBundle),
    Command(Command),
    InputPointerMotion(Vector),
    InputPointerPosition(OptionalPosition),
    InputPointerButton(Button),
    InputPointerAxis(Vector),
    InputPointerPositionReset,
    InputKeyboard(Key),
    SurfaceReady(SurfaceId),
    SurfaceDestroyed(SurfaceId),
    SurfaceReconfigured(SurfaceId),
    CursorSurfaceChange(SurfaceId),
    SurfaceFrame(SurfaceId),
    PointerFocusChanged(SurfacePosition),
    PointerRelativeMotion(SurfacePosition),
    KeyboardFocusChanged(SurfaceId, SurfaceId),
}

// -------------------------------------------------------------------------------------------------

impl Transportable for Perceptron {}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for Perceptron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Perceptron::Notify => write!(f, "Notify"),
            Perceptron::VerticalBlank(ref data) => write!(f, "VerticalBlank({:?})", data),
            Perceptron::PageFlip(ref data) => write!(f, "PageFlip({:?})", data),
            Perceptron::OutputFound(ref bundle) => write!(f, "OutputFound({:?})", bundle),
            Perceptron::Command(ref command) => write!(f, "Command({:?})", command),

            Perceptron::InputPointerMotion(ref vector) => {
                write!(f, "InputPointerMotion({:?})", vector)
            }
            Perceptron::InputPointerPosition(ref pos) => {
                write!(f, "InputPointerPosition({:?})", pos)
            }
            Perceptron::InputPointerButton(ref btn) => write!(f, "InputPointerButton({:?})", btn),
            Perceptron::InputPointerAxis(ref axis) => write!(f, "InputPointerAxis({:?})", axis),
            Perceptron::InputPointerPositionReset => write!(f, "InputPointerPositionReset"),
            Perceptron::InputKeyboard(ref key) => write!(f, "InputKeyboard({:?})", key),

            Perceptron::SurfaceReady(ref sid) => write!(f, "SurfaceReady({})", sid),
            Perceptron::SurfaceDestroyed(ref sid) => write!(f, "SurfaceDestroyed({})", sid),
            Perceptron::SurfaceReconfigured(ref sid) => write!(f, "SurfaceReconfigured({})", sid),
            Perceptron::CursorSurfaceChange(ref sid) => write!(f, "CursorSurfaceChange({})", sid),

            Perceptron::SurfaceFrame(ref sid) => write!(f, "SurfaceFrame({})", sid),
            Perceptron::PointerFocusChanged(ref pos) => write!(f, "PointerFocusChanged({:?})", pos),
            Perceptron::PointerRelativeMotion(ref pos) => {
                write!(f, "PointerRelativeMotion({:?})", pos)
            }
            Perceptron::KeyboardFocusChanged(ref old_sid, ref new_sid) => {
                write!(f, "KeyboardFocusChanged({:?}, {:?})", old_sid, new_sid)
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
