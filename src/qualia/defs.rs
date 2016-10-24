// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains definitions of various useful structures.

// -------------------------------------------------------------------------------------------------

use std;

use enums;

// -------------------------------------------------------------------------------------------------

pub type Point = Position;
pub type Vector = Position;
pub type Key = Button;

/// Type of surface ID.
pub type SurfaceIdType = u64;

pub type Fd = i32;

pub type KeyCode = u16;
pub type KeyValue = i32;

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

pub mod mode_name {
    pub const COMMON: &'static str = "common";
    pub const INSERT: &'static str = "insert";
    pub const NORMAL: &'static str = "normal";
}

// -------------------------------------------------------------------------------------------------

/// Type defining position, point coordinates or 2D vector.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// -------------------------------------------------------------------------------------------------

impl Position {
    /// `Position` constructor.
    pub fn new(x: i32, y: i32) -> Self {
        Position { x: x, y: y }
    }

    /// Return new position scaled by given factor.
    pub fn scaled(&self, scale: f32) -> Self {
        Position {
            x: (scale * self.x as f32) as _,
            y: (scale * self.y as f32) as _,
        }
    }

    /// Check if position is inside given area.
    pub fn is_inside(&self, area: &Area) -> bool {
        area.contains(self)
    }

    /// Return new position casted into given area.
    /// - if `self` if inside area - return copy of `self`
    /// - if `self` if outside area - return closes point inside area
    pub fn casted(&self, area: &Area) -> Self {
        let mut position = self.clone();

        if position.x < area.pos.x {
            position.x = area.pos.x;
        }

        if position.x > (area.pos.x + area.size.width as i32 - 1) {
            position.x = area.pos.x + area.size.width as i32 - 1;
        }

        if position.y < area.pos.y {
            position.y = area.pos.y;
        }

        if position.y > (area.pos.y + area.size.height as i32 - 1) {
            position.y = area.pos.y + area.size.height as i32 - 1;
        }

        position
    }
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining position, point coordinates or 2D vector.
#[derive(Clone, Debug)]
pub struct OptionalPosition {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

// -------------------------------------------------------------------------------------------------

impl OptionalPosition {
    /// `OptionalPosition` constructor.
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self {
        OptionalPosition { x: x, y: y }
    }

    /// Return new optional position scaled by given factor.
    pub fn scaled(&self, scale: f32) -> Self {
        OptionalPosition {
            x: if let Some(v) = self.x {
                Some((scale * v as f32) as i32)
            } else {
                None
            },
            y: if let Some(v) = self.y {
                Some((scale * v as f32) as i32)
            } else {
                None
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for OptionalPosition {
    fn default() -> Self {
        OptionalPosition { x: None, y: None }
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure combines information about surface ID with position.
#[derive(Clone, Debug)]
pub struct SurfacePosition {
    pub sid: SurfaceId,
    pub pos: Position,
}

// -------------------------------------------------------------------------------------------------

impl SurfacePosition {
    /// `SurfacePosition` constructor.
    pub fn new(sid: SurfaceId, pos: Position) -> Self {
        SurfacePosition {
            sid: sid,
            pos: pos,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D size, dimensions or resolution.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct Area {
    pub pos: Position,
    pub size: Size,
}

// -------------------------------------------------------------------------------------------------

impl Area {
    /// `Area` constructor.
    pub fn new(pos: Position, size: Size) -> Self {
        Area {
            pos: pos,
            size: size,
        }
    }

    /// Check if area contains given position.
    pub fn contains(&self, pos: &Position) -> bool {
        let margin_top = self.pos.y;
        let margin_bottom = self.size.height as i32 + margin_top;
        let margin_left = self.pos.x;
        let margin_right = self.size.width as i32 + margin_left;

        (margin_top <= pos.y) && (pos.y < margin_bottom) && (margin_left <= pos.x) &&
        (pos.x < margin_right)
    }

    /// Calculate position in center of the area.
    pub fn calculate_center(&self) -> Position {
        Position::new((self.pos.x + self.size.width as i32) / 2,
                      (self.pos.y + self.size.height as i32) / 2)
    }
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

/// Data for button event.
#[derive(Clone, Debug)]
pub struct Button {
    pub code: u16,
    pub value: i32,
}

// -------------------------------------------------------------------------------------------------

impl Button {
    /// `Button` constructor.
    pub fn new(code: u16, value: i32) -> Self {
        Button {
            code: code,
            value: value,
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

/// Command context for compositor.
#[derive(Clone, Debug)]
pub struct Command {
    pub action: enums::Action,
    pub direction: enums::Direction,
    pub magnitude: i32,
    pub string: String,
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Command {
    fn default() -> Self {
        Command {
            action: enums::Action::None,
            direction: enums::Direction::None,
            magnitude: 0,
            string: "".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
