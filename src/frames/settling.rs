// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra settling functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::SurfaceAccess;

use frame::Frame;
use searching::Searching;
use packing::Packing;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more settling functionality.
pub trait Settling {
    /// Settle self in buildable of target and relax it.
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess);
}

// -------------------------------------------------------------------------------------------------

impl Settling for Frame {
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess) {
        if let Some(ref mut buildable) = target.find_buildable() {
            buildable.append(self);
            buildable.relax(sa);
        }
    }
}

// -------------------------------------------------------------------------------------------------
