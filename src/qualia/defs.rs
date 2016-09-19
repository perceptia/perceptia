// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains definitions of various useful structures.

// -------------------------------------------------------------------------------------------------

use std;

pub type Point = Position;
pub type Vector = Position;

/// Type of surface ID.
pub type SurfaceIdType = u64;

pub type Fd = i32;

// -------------------------------------------------------------------------------------------------

pub const INVALID_SURFACE_ID: SurfaceIdType = 0;

// -------------------------------------------------------------------------------------------------

/// Structure representing surface ID.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct SurfaceId(SurfaceIdType);

// -------------------------------------------------------------------------------------------------

impl SurfaceId {
    /// Create new surface ID.
    pub fn new(sid: SurfaceIdType) -> Self {
        SurfaceId(sid)
    }

    /// Create new invalid surface ID.
    pub fn invalid() -> Self {
        SurfaceId(INVALID_SURFACE_ID)
    }

    /// Check if the surface ID is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 != INVALID_SURFACE_ID
    }

    /// Cast surface ID as number.
    pub fn as_number(&self) -> SurfaceIdType {
        self.0
    }
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Debug for SurfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_valid() {
            write!(f, "SID({:?})", self.0)
        } else {
            write!(f, "<invalid>")
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for SurfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_valid() {
            write!(f, "SID({})", self)
        } else {
            write!(f, "<invalid>")
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining position, point coordinates or 2D vector.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D size, dimensions or resolution.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

// -------------------------------------------------------------------------------------------------

impl Size {
    /// `Size` constructor.
    pub fn new(width: u32, height: u32) -> Self {
        Size {
            width: width,
            height: height,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Size {
    fn default() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D area.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Area {
    pub pos: Position,
    pub size: Size,
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Area {
    fn default() -> Self {
        Area {
            pos: Position::default(),
            size: Size::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure containing all data needed to initialize DRM output.
#[derive(Clone, Copy, Debug)]
pub struct DrmBundle {
    pub fd: Fd,
    pub crtc_id: u32,
    pub connector_id: u32,
}

// -------------------------------------------------------------------------------------------------