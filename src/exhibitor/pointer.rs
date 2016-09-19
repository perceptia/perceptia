// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to pointer like tracking position or setting surface.

// -------------------------------------------------------------------------------------------------

use qualia::{Buffer, Coordinator, Position, SurfaceId};

// -------------------------------------------------------------------------------------------------

const DEFAULT_CURSOR_SIZE: usize = 15;

// -------------------------------------------------------------------------------------------------

/// State of the pointer.
pub struct Pointer {
    /// Position in global coordinates.
    position: Position,

    /// Surface ID of cursor surface.
    csid: SurfaceId,

    /// Default surface ID of cursor surface.
    default_csid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl Pointer {
    /// `Pointer` constructor.
    pub fn new(coordinator: &mut Coordinator) -> Self {
        let mut data = vec![200; 4 * DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE];
        for z in 0..(DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE) {
            data[4 * z + 3] = 100;
        }

        let default_csid = coordinator.create_surface();
        let b = Buffer::new(DEFAULT_CURSOR_SIZE,
                            DEFAULT_CURSOR_SIZE,
                            4 * DEFAULT_CURSOR_SIZE,
                            data);
        coordinator.attach(default_csid, b);
        coordinator.commit_surface(default_csid);

        Pointer {
            position: Position::default(),
            csid: default_csid,
            default_csid: default_csid,
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Getters
impl Pointer {
    /// Get position in global coordinates.
    pub fn get_global_position(&self) -> Position {
        self.position.clone()
    }

    /// Get ID of the cursor surface.
    pub fn get_sid(&self) -> SurfaceId {
        self.csid.clone()
    }
}

// -------------------------------------------------------------------------------------------------
