// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module gathers functionality related to surfaces.

// -------------------------------------------------------------------------------------------------

use std::sync::Arc;

use buffer::Buffer;
use defs::{Position, Size, Vector};
pub use defs::{SurfaceId, SurfaceIdType};

// -------------------------------------------------------------------------------------------------

/// This structure defines how the surface should be drawn.
#[derive(Debug)]
pub struct SurfaceContext {
    pub id: SurfaceId,
    pub pos: Position,
}

// -------------------------------------------------------------------------------------------------

impl SurfaceContext {
    /// `SurfaceContext` constructor.
    pub fn new(id: SurfaceId, pos: Position) -> SurfaceContext {
        SurfaceContext { id: id, pos: pos }
    }
}

// -------------------------------------------------------------------------------------------------

/// These flags describe readiness of `Surface` to be displayed.
pub mod show_reason {
    pub type ShowReason = i32;
    pub const UNINITIALIZED: ShowReason = 0b0000;
    pub const DRAWABLE: ShowReason = 0b0001;
    pub const IN_SHELL: ShowReason = 0b0010;
    pub const READY: ShowReason = DRAWABLE | IN_SHELL;
}

// -------------------------------------------------------------------------------------------------

/// Structure containing public information about surface.
pub struct SurfaceInfo {
    pub id: SurfaceId,
    pub offset: Vector,
    pub parent_sid: SurfaceId,
    pub desired_size: Size,
    pub requested_size: Size,
}

// -------------------------------------------------------------------------------------------------

/// This structure represents surface.
pub struct Surface {
    /// ID of the surface.
    id: SurfaceId,

    /// Offset used to move coordinate system of surface.
    offset: Vector,

    /// Size desired by compositor.
    desired_size: Size,

    /// Size requested by client.
    requested_size: Size,

    /// ID of parent surface.
    parent_sid: SurfaceId,

    /// List of IDs of satelliting surfaces.
    satellites: Vec<SurfaceId>,

    /// Position requested by client relative to parent surface.
    /// For surfaces without parent this must be {0, 0}.
    relative_position: Position,

    /// Data required for draw.
    buffer: Arc<Buffer>,

    /// Data to be used after commit.
    pending_buffer: Buffer,

    /// Flags indicating if surface is ready to be shown.
    show_reasons: show_reason::ShowReason,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    /// `Surface` constructor.
    pub fn new(id: &SurfaceId) -> Self {
        Surface {
            id: *id,
            offset: Vector::default(),
            desired_size: Size::default(),
            requested_size: Size::default(),
            parent_sid: SurfaceId::invalid(),
            satellites: Vec::new(),
            relative_position: Position::default(),
            buffer: Arc::new(Buffer::empty()),
            pending_buffer: Buffer::empty(),
            show_reasons: show_reason::UNINITIALIZED,
        }
    }

    /// Sets position offset.
    pub fn set_offset(&mut self, offset: Vector) {
        self.offset.x = if offset.x > 0 { offset.x } else { 0 };
        self.offset.y = if offset.y > 0 { offset.y } else { 0 };
    }

    /// Sets size requested by client.
    #[inline]
    pub fn set_requested_size(&mut self, size: Size) {
        self.requested_size = size
    }

    /// Adds given reason to show reasons. Returns updates set of reasons.
    #[inline]
    pub fn show(&mut self, reason: show_reason::ShowReason) -> show_reason::ShowReason {
        self.show_reasons |= reason;
        self.show_reasons
    }

    /// Sets given buffer as pending.
    #[inline]
    pub fn attach(&mut self, buffer: Buffer) {
        self.pending_buffer.assign_from(&buffer);
    }

    /// Sets pending buffer as current. If surface was committed for the first time and sizes are
    /// not set, assign size of buffer as requested size. Return `true` if surface was committed for
    /// the first time, `false` otherwise.
    pub fn commit(&mut self) -> bool {
        let is_first_time_committed = self.buffer.is_empty();
        if let Some(buffer) = Arc::get_mut(&mut self.buffer) {
            buffer.assign_from(&self.pending_buffer);
        }

        // If surface was just created...
        if is_first_time_committed {
            // ... size was not yet requested by surface ...
            if !((self.requested_size.width == 0) || (self.requested_size.height == 0)) {
                // ... use its buffer size as requested size ...
                self.requested_size = self.buffer.get_size();
            }
            // ... and if it is subsurface ...
            if self.parent_sid.is_valid() {
                // ... set its desired size.
                self.desired_size = self.buffer.get_size();
            }
        }

        is_first_time_committed
    }

    /// Returns information about surface.
    pub fn get_info(&self) -> SurfaceInfo {
        SurfaceInfo {
            id: self.id,
            offset: self.offset.clone(),
            parent_sid: self.parent_sid,
            desired_size: self.desired_size.clone(),
            requested_size: self.requested_size.clone(),
        }
    }

    /// Returns surfaces buffer.
    pub fn get_buffer(&self) -> Arc<Buffer> {
        self.buffer.clone()
    }

    /// Returns surfaces rendering context.
    pub fn get_renderer_context(&self) -> SurfaceContext {
        SurfaceContext {
            id: self.id,
            pos: self.relative_position.clone(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait used for configuring and manipulating surfaces.
pub trait SurfaceAccess {
    fn configure(&mut self, sid: SurfaceId, i: i32);
}

// -------------------------------------------------------------------------------------------------
