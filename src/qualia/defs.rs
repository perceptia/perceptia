// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains definitions of various useful structures.

// -------------------------------------------------------------------------------------------------

use std;

use enums;
use timing;

// -------------------------------------------------------------------------------------------------

pub type Point = Position;
pub type Vector = Position;
pub type Key = Button;

/// Type of surface ID.
pub type SurfaceIdType = u64;

pub type KeyCode = u16;
pub type KeyValue = i32;

// -------------------------------------------------------------------------------------------------

pub const INVALID_SURFACE_ID: SurfaceIdType = 0;

// -------------------------------------------------------------------------------------------------

/// Structure representing surface ID.
/// TODO: Define `SurfaceId` using `define_id` macro.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct SurfaceId(SurfaceIdType);

define_id!(pub MemoryPoolId: usize);
define_id!(pub MemoryViewId: usize);
define_id!(pub HwImageId: usize);

/// Type alias for signal IDs.
pub type SignalId = usize;

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
            write!(f, "SID({})", self.0)
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

// -------------------------------------------------------------------------------------------------

impl Position {
    /// `Position` constructor.
    pub fn new(x: isize, y: isize) -> Self {
        Position { x: x, y: y }
    }

    /// Check if `Position` points at (0,0).
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Check if position is inside given area.
    pub fn is_inside(&self, area: &Area) -> bool {
        area.contains(self)
    }

    /// Return new position scaled by given factor.
    pub fn scaled(&self, scale: f32) -> Self {
        Position {
            x: (scale * self.x as f32) as _,
            y: (scale * self.y as f32) as _,
        }
    }

    /// Returns opposite position (with negated coordinates).
    pub fn opposite(&self) -> Self {
        Position {
            x: -1 * self.x,
            y: -1 * self.y,
        }
    }

    /// Return new position casted into given area.
    /// - if `self` if inside area - return copy of `self`
    /// - if `self` if outside area - return closes point inside area
    pub fn casted(&self, area: &Area) -> Self {
        let mut position = self.clone();

        if !area.is_zero() {
            if position.x < area.pos.x {
                position.x = area.pos.x;
            }

            if position.x > (area.pos.x + area.size.width as isize - 1) {
                position.x = area.pos.x + area.size.width as isize - 1;
            }

            if position.y < area.pos.y {
                position.y = area.pos.y;
            }

            if position.y > (area.pos.y + area.size.height as isize - 1) {
                position.y = area.pos.y + area.size.height as isize - 1;
            }
        } else {
            position = area.pos;
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
#[derive(Clone, Copy, Debug)]
pub struct OptionalPosition {
    pub x: Option<isize>,
    pub y: Option<isize>,
}

// -------------------------------------------------------------------------------------------------

impl OptionalPosition {
    /// `OptionalPosition` constructor.
    pub fn new(x: Option<isize>, y: Option<isize>) -> Self {
        OptionalPosition { x: x, y: y }
    }

    /// Return new optional position scaled by given factor.
    pub fn scaled(&self, scale: f32) -> Self {
        OptionalPosition {
            x: if let Some(v) = self.x {
                Some((scale * v as f32) as isize)
            } else {
                None
            },
            y: if let Some(v) = self.y {
                Some((scale * v as f32) as isize)
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

/// `Vector` with continuous coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Slide {
    pub x: f32,
    pub y: f32,
}

// -------------------------------------------------------------------------------------------------

impl Slide {
    /// `Slide` constructor.
    pub fn new(x: f32, y: f32) -> Self {
        Slide { x: x, y: y }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::default::Default for Slide {
    fn default() -> Self {
        Slide { x: 0.0, y: 0.0 }
    }
}

// -------------------------------------------------------------------------------------------------

/// Type defining 2D size, dimensions or resolution.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

// -------------------------------------------------------------------------------------------------

impl Size {
    /// `Size` constructor.
    pub fn new(width: usize, height: usize) -> Self {
        Size {
            width: width,
            height: height,
        }
    }

    /// Returns new scaled `Size`.
    pub fn scaled(&self, scale: f32) -> Size {
        Size::new((scale * self.width as f32) as usize, (scale * self.height as f32) as usize)
    }

    /// Check if `Size` has zero size.
    pub fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    pub pos: Position,
    pub size: Size,
}

// -------------------------------------------------------------------------------------------------

impl Area {
    /// Constructs `Area` from `Position` and `Size`.
    pub fn new(pos: Position, size: Size) -> Self {
        Area {
            pos: pos,
            size: size,
        }
    }

    /// Constructs `Area` from coordinates and dimensions.
    pub fn create(x: isize, y: isize, width: usize, height: usize) -> Self {
        Area {
            pos: Position::new(x, y),
            size: Size::new(width, height),
        }
    }

    /// Return this area but with position set to origin (0,0).
    pub fn rebased(&self) -> Self {
        Area {
            pos: Position::default(),
            size: self.size,
        }
    }

    /// Check if `Area` has zero area.
    pub fn is_zero(&self) -> bool {
        self.size.is_zero()
    }

    /// Check if `Area` contains given position.
    pub fn contains(&self, pos: &Position) -> bool {
        let margin_top = self.pos.y;
        let margin_bottom = self.size.height as isize + margin_top;
        let margin_left = self.pos.x;
        let margin_right = self.size.width as isize + margin_left;

        (margin_top <= pos.y) && (pos.y < margin_bottom) && (margin_left <= pos.x) &&
        (pos.x < margin_right)
    }

    /// Calculate position in center of the area.
    pub fn calculate_center(&self) -> Position {
        Position::new((self.pos.x + self.size.width as isize) / 2,
                      (self.pos.y + self.size.height as isize) / 2)
    }

    /// Inflates this `Area` so that it contains passed `area`.
    pub fn inflate(&mut self, area: &Area) {
        let old = self.clone();

        let mut diff: isize = old.pos.x - area.pos.x;
        if diff > 0 {
            self.size.width += diff as usize;
            self.pos.x = area.pos.x;
        }

        diff = old.pos.y - area.pos.y;
        if diff > 0 {
            self.size.height += diff as usize;
            self.pos.y = area.pos.y;
        }

        diff = area.size.width as isize - old.size.width as isize + area.pos.x - old.pos.x;
        if diff > 0 {
            self.size.width += diff as usize;
        }

        diff = area.size.height as isize - old.size.height as isize + area.pos.y - old.pos.y;
        if diff > 0 {
            self.size.height += diff as usize;
        }
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

    /// Constructs `Button` with current time.
    pub fn new_now(code: u16, value: i32) -> Self {
        Button {
            code: code,
            value: value,
            time: timing::Milliseconds::now(),
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

    pub fn new_now(discrete: Vector, continuous: Slide) -> Self {
        Axis {
            discrete: discrete,
            continuous: continuous,
            time: timing::Milliseconds::now(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Set of informations about output.
#[derive(Clone, Debug)]
pub struct OutputInfo {
    pub id: i32,
    pub area: Area,
    pub physical_size: Size,
    pub refresh_rate: usize,
    pub make: String,
    pub model: String,
}

// -------------------------------------------------------------------------------------------------

impl OutputInfo {
    /// Constructs new `OutputInfo`.
    pub fn new(id: i32,
               area: Area,
               physical_size: Size,
               refresh_rate: usize,
               make: String,
               model: String)
               -> Self {
        OutputInfo {
            id: id,
            area: area,
            physical_size: physical_size,
            refresh_rate: refresh_rate,
            make: make,
            model: model,
        }
    }
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
