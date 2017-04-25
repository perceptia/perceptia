// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related appearance of default cursor.

// -------------------------------------------------------------------------------------------------

use qualia::{Buffer, SurfaceId, AestheticsCoordinationTrait};

// -------------------------------------------------------------------------------------------------

const DEFAULT_CURSOR_SIZE: usize = 15;

// -------------------------------------------------------------------------------------------------

/// State of the cursor.
pub struct Cursor<C> where C: AestheticsCoordinationTrait {
    /// Surface ID of cursor surface.
    csid: SurfaceId,

    /// Default surface ID of cursor surface.
    default_csid: SurfaceId,

    /// Coordinator.
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<C> Cursor<C> where C: AestheticsCoordinationTrait {
    /// Constructs new `Cursor`.
    pub fn new(coordinator: C) -> Self {
        Cursor {
            csid: SurfaceId::invalid(),
            default_csid: SurfaceId::invalid(),
            coordinator: coordinator,
        }
    }

    /// Initializes default cursor buffer.
    ///
    /// Sets default cursor to be white semitransparent rectangle.
    pub fn initialize(&mut self) {
        let mut data = vec![200; 4 * DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE];
        for z in 0..(DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE) {
            data[4 * z + 3] = 100;
        }

        self.default_csid = self.coordinator.create_surface();
        let b =
            Buffer::new(DEFAULT_CURSOR_SIZE, DEFAULT_CURSOR_SIZE, 4 * DEFAULT_CURSOR_SIZE, data);
        let bid = self.coordinator.create_pool_from_buffer(b);
        if let Some(mvid) = self.coordinator.create_memory_view(bid,
                                                                0,
                                                                DEFAULT_CURSOR_SIZE,
                                                                DEFAULT_CURSOR_SIZE,
                                                                4 * DEFAULT_CURSOR_SIZE) {
            self.coordinator.attach_surface(mvid, self.default_csid);
            self.coordinator.commit_surface(self.default_csid);
            self.coordinator.set_surface_as_cursor(self.default_csid);
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<C> Cursor<C> where C: AestheticsCoordinationTrait {
    /// Handles cursor surface change notification.
    pub fn on_surface_change(&mut self, sid: SurfaceId) {
        self.csid = sid;
    }

    /// Handles pointer focus change. If no surface is focussed pointer cursor surface must be set
    /// to default.
    pub fn on_focus_changed(&mut self, _old_pfsid: SurfaceId, new_pfsid: SurfaceId) {
        if !new_pfsid.is_valid() {
            self.coordinator.set_surface_as_cursor(self.default_csid);
        }
    }

    /// Handles destruction of surface. If current cursor surface was destroyed it must be reset to
    /// default.
    pub fn on_surface_destroyed(&mut self, sid: SurfaceId) {
        if self.csid == sid {
            self.coordinator.set_surface_as_cursor(self.default_csid);
        }
    }

    /// Handles creation of display.
    ///
    /// Here we have sure `Exhibitor` is initialized so we can initialize cursor buffer.
    pub fn on_display_created(&mut self) {
        if !self.default_csid.is_valid() {
            self.initialize();
        }
    }
}

// -------------------------------------------------------------------------------------------------
