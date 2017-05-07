// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module gathers functionality related to surfaces.

// -------------------------------------------------------------------------------------------------

use qualia::{DmabufAttributes, EglAttributes, HwImage, MemoryView};
use qualia::{Position, Size, Vector};
use qualia::{DataSource, SurfaceContext, SurfaceId, SurfaceInfo, show_reason, surface_state};

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
    buffer: DataSource,

    /// Data to be used after commit.
    pending_buffer: DataSource,

    /// Flags describing logical state of surface
    state_flags: surface_state::SurfaceState,

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
            satellites: vec![*id],
            relative_position: Position::default(),
            buffer: DataSource::None,
            pending_buffer: DataSource::None,
            show_reasons: show_reason::NONE,
            state_flags: surface_state::REGULAR,
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

    /// Sets size desired by compositor.
    #[inline]
    pub fn set_desired_size(&mut self, size: Size) {
        self.desired_size = size
    }

    /// Sets parent SID.
    #[inline]
    pub fn set_parent_sid(&mut self, sid: SurfaceId) {
        self.parent_sid = sid
    }

    /// Adds satellite.
    #[inline]
    pub fn add_satellite(&mut self, sid: SurfaceId) {
        let mut contains = false;
        for satellite in self.satellites.iter() {
            if *satellite == sid {
                contains = true;
            }
        }
        if !contains {
            self.satellites.push(sid);
        }
    }

    /// Removes satellite.
    #[inline]
    pub fn remove_satellite(&mut self, sid: SurfaceId) {
        let mut index = None;
        for (i, satellite) in self.satellites.iter().enumerate() {
            if *satellite == sid {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            self.satellites.remove(index);
        }
    }

    /// Sets relative position.
    #[inline]
    pub fn set_relative_position(&mut self, position: Position) {
        self.relative_position = position
    }

    /// Sets state flags.
    #[inline]
    pub fn set_state_flags(&mut self, state_flags: surface_state::SurfaceState) {
        self.state_flags = state_flags
    }

    /// Adds given reason to show reasons. Returns updates set of reasons.
    ///
    /// Satelliting surface can not be shown in shell.
    #[inline]
    pub fn show(&mut self, reason: show_reason::ShowReason) -> show_reason::ShowReason {
        if !(reason == show_reason::IN_SHELL && self.parent_sid.is_valid()) {
            self.show_reasons.insert(reason);
        } else {
            log_warn2!("Requested to show satelliting surface {} in shell", self.id);
        }
        self.show_reasons
    }

    /// Subtracts given reason from show reasons. Returns updates set of reasons.
    #[inline]
    pub fn hide(&mut self, reason: show_reason::ShowReason) -> show_reason::ShowReason {
        self.show_reasons.remove(reason);
        self.show_reasons
    }

    /// Returns reasons to show the surface.
    #[inline]
    pub fn get_show_reason(&mut self) -> show_reason::ShowReason {
        self.show_reasons
    }

    /// Sets given buffer as pending.
    #[inline]
    pub fn attach_shm(&mut self, buffer: MemoryView) {
        self.pending_buffer = DataSource::Shm(buffer);
    }

    /// Sets given hardware image as pending.
    #[inline]
    pub fn attach_hw_image(&mut self, image: HwImage, attrs: EglAttributes) {
        self.pending_buffer = DataSource::HwImage(image, attrs);
    }

    /// Sets given dmabuf as pending.
    #[inline]
    pub fn attach_dmabuf(&mut self, image: HwImage, attrs: DmabufAttributes) {
        self.pending_buffer = DataSource::Dmabuf(image, attrs);
    }

    /// Sets pending buffer as current. If surface was committed for the first time and sizes are
    /// not set, assign size of buffer as requested size. Return `true` if surface was committed for
    /// the first time, `false` otherwise.
    pub fn commit(&mut self) -> bool {
        let is_first_time_committed = self.buffer.is_none();
        self.buffer = self.pending_buffer.clone();

        if let Some(ref image) = self.buffer.as_image() {
            // If surface was just created...
            if is_first_time_committed {
                // ... size was not yet requested by surface ...
                if (self.requested_size.width == 0) || (self.requested_size.height == 0) {
                    // ... use its image size as requested size ...
                    self.requested_size = image.get_size();
                }
                // ... and if it is subsurface ...
                if self.parent_sid.is_valid() {
                    // ... set its desired size.
                    self.desired_size = image.get_size();
                }
            }
        }

        is_first_time_committed
    }

    /// Returns information about surface.
    pub fn get_info(&self) -> SurfaceInfo {
        SurfaceInfo {
            id: self.id,
            offset: self.offset,
            parent_sid: self.parent_sid,
            desired_size: self.desired_size,
            requested_size: self.requested_size,
            state_flags: self.state_flags,
            data_source: self.buffer.clone(),
        }
    }

    /// Returns surfaces buffer.
    pub fn get_data_source(&self) -> DataSource {
        self.buffer.clone()
    }

    /// Returns surfaces rendering context.
    pub fn get_renderer_context(&self) -> SurfaceContext {
        SurfaceContext {
            id: self.id,
            pos: self.relative_position,
        }
    }

    /// Returns size desired by compositor.
    pub fn get_desired_size(&self) -> Size {
        self.desired_size
    }

    /// Returns flags describing state of the surface.
    pub fn get_state_flags(&self) -> surface_state::SurfaceState {
        self.state_flags
    }

    /// Returns ID of parent surface.
    pub fn get_parent_sid(&self) -> SurfaceId {
        self.parent_sid
    }

    /// Returns vector of IDs of satelliting surfaces (pop-ups, subsurfaces).
    pub fn get_satellites(&self) -> &Vec<SurfaceId> {
        &self.satellites
    }
}

// -------------------------------------------------------------------------------------------------
