// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides structures and functionality related to surfaces.

// -------------------------------------------------------------------------------------------------

use defs::MemoryViewId;
use memory::MemoryView;
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
    pub fn new(id: SurfaceId, pos: Position) -> Self {
        SurfaceContext { id: id, pos: pos }
    }

    /// Creates new context with position moved by given vector.
    pub fn moved(&self, vector: Vector) -> Self {
        SurfaceContext::new(self.id, self.pos + vector)
    }
}

// -------------------------------------------------------------------------------------------------

/// These flags describe readiness of `Surface` to be displayed.
pub mod show_reason {
    bitflags!(
        pub flags ShowReason: u32 {
            const NONE = 0b0000,
            const DRAWABLE = 0b0001,
            const IN_SHELL = 0b0010,
            const READY = DRAWABLE.bits | IN_SHELL.bits,
        }
    );
}

// -------------------------------------------------------------------------------------------------

/// These constants describe state of `Surface`.
pub mod surface_state {
    bitflags!(
        pub flags SurfaceState: u32 {
            const REGULAR = 0x0000,
            const MAXIMIZED = 0b0001,
            const FULLSCREEN = 0x0010,
            const RESIZING = 0x0100,
        }
    );
}

// -------------------------------------------------------------------------------------------------

/// Structure containing public information about surface.
pub struct SurfaceInfo {
    pub id: SurfaceId,
    pub offset: Vector,
    pub parent_sid: SurfaceId,
    pub desired_size: Size,
    pub requested_size: Size,
    pub state_flags: surface_state::SurfaceState,
    pub buffer: Option<MemoryView>,
}

// -------------------------------------------------------------------------------------------------

/// Managing surface content.
pub trait SurfaceManagement {
    /// Creates new surface with newly generated unique ID.
    fn create_surface(&mut self) -> SurfaceId;

    /// Sets given buffer as pending for given surface.
    fn attach_surface(&self, mvid: MemoryViewId, sid: SurfaceId);

    /// Informs other parts of application the surface is now not visible.
    fn detach_surface(&self, sid: SurfaceId);

    /// Commits the surface.
    fn commit_surface(&self, sid: SurfaceId);

    /// Detaches and forgets given surface.
    fn destroy_surface(&self, sid: SurfaceId);
}

// -------------------------------------------------------------------------------------------------

/// Controlling surfaces parameters like size, position and relationships.
pub trait SurfaceControl {
    /// Adds given show reason flag to set of surfaces show reason.
    fn show_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason);

    /// Subtracts given show reason flag from set of surfaces show reason.
    fn hide_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason);

    /// Sets position offset given surface.
    fn set_surface_offset(&self, sid: SurfaceId, offset: Vector);

    /// Sets requested size for given surface.
    fn set_surface_requested_size(&self, sid: SurfaceId, size: Size);

    /// Sets satellite surface position relative to its parent.
    fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector);

    /// Relates two surfaces.
    fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId);

    /// Unrelates two surfaces.
    fn unrelate_surface(&self, sid: SurfaceId);
}

// -------------------------------------------------------------------------------------------------

/// Viewing information about surface.
pub trait SurfaceViewer {
    /// Returns information about surface.
    fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo>;
}

// -------------------------------------------------------------------------------------------------

/// Trait used for configuring and manipulating surfaces.
pub trait SurfaceAccess {
    /// Reconfigure surface and send notification about this event.
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState);
}

// -------------------------------------------------------------------------------------------------

/// Listing related surfaces.
pub trait SurfaceListing {
    /// Returns surface context.
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>>;
}

// -------------------------------------------------------------------------------------------------

/// Focusing and obtaining information about keyboard and pointer focus.
pub trait SurfaceFocusing {
    /// Returns ID of currently keyboard-focussed surface.
    fn get_keyboard_focused_sid(&self) -> SurfaceId;

    /// Informs rest of the application exhibitor set keyboard focus to given surface.
    fn set_keyboard_focus(&mut self, sid: SurfaceId);

    /// Returns ID of currently pointer-focussed surface.
    fn get_pointer_focused_sid(&self) -> SurfaceId;

    /// Informs rest of the application exhibitor set pointer focus to given surface.
    fn set_pointer_focus(&mut self, sid: SurfaceId, position: Position);
}

// -------------------------------------------------------------------------------------------------
