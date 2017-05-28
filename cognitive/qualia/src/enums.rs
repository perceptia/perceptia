// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Enum definitions for perceptia.

// -------------------------------------------------------------------------------------------------

use std::fmt;

// -------------------------------------------------------------------------------------------------

/// Enum describing kind of input device.
#[derive(PartialEq)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
    Touchpad,
    Unknown,
}

// -------------------------------------------------------------------------------------------------

impl fmt::Debug for DeviceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviceKind::Keyboard => write!(f, "keyboard"),
            DeviceKind::Mouse => write!(f, "mouse"),
            DeviceKind::Touchpad => write!(f, "touchpad"),
            DeviceKind::Unknown => write!(f, "unknown device"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Enum for key states.
#[derive(Debug, PartialEq)]
pub enum KeyState {
    Released = 0,
    Pressed = 1,
}

// -------------------------------------------------------------------------------------------------

/// Action type for Exhibitor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Dummy; do/parametrize nothing.
    None,

    /// Anchorize; de-anchorize.
    Anchor,

    /// Change configuration.
    Configure,

    /// Change focus.
    Focus,

    /// Swap.
    Swap,

    /// Change position.
    Move,

    /// Change placement by jumping over.
    Jump,

    /// Change placement by diving in.
    Dive,

    /// Change size.
    Resize,
}

// -------------------------------------------------------------------------------------------------

/// Enum representing directions on screen, in time and between frames.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Dummy; point nowhere.
    None,

    /// North; up; above.
    North,

    /// East; right.
    East,

    /// South; down; below.
    South,

    /// West; left.
    West,

    /// Back in time; most recently used.
    Backward,

    /// Forward in time; the oldest used.
    Forward,

    /// Begin; start; head.
    Begin,

    /// End; finish; tail.
    End,

    /// Trunk; parent; up in frame hierarchy.
    Up,

    /// Workspace.
    Workspace,
}

// -------------------------------------------------------------------------------------------------

impl Direction {
    /// Reverse the direction.
    pub fn reversed(self) -> Self {
        match self {
            Direction::None => Direction::None,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::South,
            Direction::Backward => Direction::Forward,
            Direction::Forward => Direction::Backward,
            Direction::Begin => Direction::End,
            Direction::End => Direction::Begin,
            Direction::Up => Direction::Up,
            Direction::Workspace => Direction::Workspace,
        }
    }
}

// -------------------------------------------------------------------------------------------------
