// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Exhibitor` manages tasks related to drawing and compositing surfaces.

// -------------------------------------------------------------------------------------------------

#![feature(deque_extras)]

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate frames;

mod surface_history;
mod compositor;

use qualia::{Coordinator, SurfaceId};

use compositor::Compositor;

// -------------------------------------------------------------------------------------------------

/// `Exhibitor` manages tasks related to drawing and compositing surfaces.
pub struct Exhibitor {
    compositor: Compositor,
    coordinator: Coordinator,
}

// -------------------------------------------------------------------------------------------------

impl Exhibitor {
    /// `Exhibitor` constructor.
    pub fn new(coordinator: Coordinator) -> Self {
        Exhibitor {
            compositor: Compositor::new(coordinator.clone()),
            coordinator: coordinator,
        }
    }

    /// This method is called when new surface is ready to be managed.
    pub fn on_surface_ready(&mut self, sid: SurfaceId) {
        self.compositor.manage_surface(sid);
    }
}

// -------------------------------------------------------------------------------------------------
