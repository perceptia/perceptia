// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains definitions of various useful structures.

// -------------------------------------------------------------------------------------------------

pub type Point = Position;
pub type Vector = Position;

// -------------------------------------------------------------------------------------------------

/// Type defining position, point coordinates or 2D vector.
#[repr(C)]
#[derive(Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D size, dimensions or resolution.
#[repr(C)]
#[derive(Clone)]
pub struct Size {
    width: u32,
    height: u32,
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D area.
#[repr(C)]
#[derive(Clone)]
pub struct Area {
    pos: Position,
    size: Size,
}

// -------------------------------------------------------------------------------------------------
