// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains mock for `SurfaceListing`.

// -------------------------------------------------------------------------------------------------

use qualia::{Position, SurfaceContext, SurfaceId, SurfaceListing};

// -------------------------------------------------------------------------------------------------

/// Mock of `SurfaceListing`.
pub struct SurfaceListingMock {}

// -------------------------------------------------------------------------------------------------

impl SurfaceListingMock {
    pub fn new() -> Self {
        SurfaceListingMock {}
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceListing for SurfaceListingMock {
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        Some(vec![SurfaceContext::new(sid, Position::default())])
    }
}

// -------------------------------------------------------------------------------------------------
