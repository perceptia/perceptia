// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains mock for `SurfaceAccess`.

// -------------------------------------------------------------------------------------------------

use qualia::{SurfaceAccess, SurfaceId, Size, surface_state};

// -------------------------------------------------------------------------------------------------

/// Mock of `SurfaceAccess`.
///
/// FIXME: Currently it is only stub. Test should be extended to also check `SurfaceAccess`
/// functionality.
pub struct SurfaceAccessMock {}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccessMock {
    pub fn new() -> Self {
        SurfaceAccessMock {}
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccess for SurfaceAccessMock {
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState) {
    }
}

// -------------------------------------------------------------------------------------------------
