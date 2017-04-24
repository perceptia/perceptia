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
    ///
    /// Sets default cursor to be white semitransparent rectangle.
    pub fn new(mut coordinator: C) -> Self {
        let mut data = vec![200; 4 * DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE];
        for z in 0..(DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE) {
            data[4 * z + 3] = 100;
        }

        let default_csid = coordinator.create_surface();
        let b =
            Buffer::new(DEFAULT_CURSOR_SIZE, DEFAULT_CURSOR_SIZE, 4 * DEFAULT_CURSOR_SIZE, data);
        let bid = coordinator.create_pool_from_buffer(b);
        if let Some(mvid) = coordinator.create_memory_view(bid,
                                                           0,
                                                           DEFAULT_CURSOR_SIZE,
                                                           DEFAULT_CURSOR_SIZE,
                                                           DEFAULT_CURSOR_SIZE) {
            coordinator.attach_surface(mvid, default_csid);
            coordinator.commit_surface(default_csid);
            coordinator.set_surface_as_cursor(default_csid);
        }

        Cursor {
            csid: SurfaceId::invalid(),
            default_csid: default_csid,
            coordinator: coordinator,
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
}

// -------------------------------------------------------------------------------------------------
