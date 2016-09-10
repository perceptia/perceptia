// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

/// `Exhibitor` manages tasks related to drawing and compositing surfaces.

// -------------------------------------------------------------------------------------------------

extern crate qualia;

use qualia::SurfaceId;

// -------------------------------------------------------------------------------------------------

/// `Exhibitor` manages tasks related to drawing and compositing surfaces.
pub struct Exhibitor {
    i: i32,
}

// -------------------------------------------------------------------------------------------------

impl Exhibitor {
    /// `Exhibitor` constructor.
    pub fn new() -> Self {
        Exhibitor { i: 9 }
    }

    /// This method is called when new surface is ready to be managed.
    pub fn on_surface_ready(&mut self, sid: SurfaceId) {}
}

// -------------------------------------------------------------------------------------------------
