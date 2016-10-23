// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra settling functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::SurfaceAccess;

use frame::{Frame, Geometry};
use searching::Searching;
use packing::Packing;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more settling functionality.
pub trait Settling {
    /// Settle self in buildable of target and relax it.
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Pop the surface `pop` and its parents inside surface `self`.
    /// After calling this function `pop` will be most recently used frame inside `self`.
    fn pop_recursively(&mut self, pop: &mut Frame);

    /// Changes frames geometry and resizes all subframe accordingly.
    fn change_geometry(&mut self, geometry: Geometry, sa: &mut SurfaceAccess);
}

// -------------------------------------------------------------------------------------------------

impl Settling for Frame {
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess) {
        if let Some(ref mut buildable) = target.find_buildable() {
            buildable.append(self);
            buildable.relax(sa);
        }
    }

    fn pop_recursively(&mut self, pop: &mut Frame) {
        // If we reached `self` we can finish
        if self.equals_exact(pop) {
            return;
        }

        // If there's nothing above we can finish
        if let Some(ref mut parent) = pop.get_parent() {
            // If it is `stacked` frame we have to pop it also spatially
            if parent.get_geometry() == Geometry::Stacked {
                pop.remove();
                parent.prepend(pop);
            }

            // Pop in temporal order
            pop.pop();

            // Do the same recursively on trunk
            self.pop_recursively(parent);
        }
    }

    fn change_geometry(&mut self, geometry: Geometry, sa: &mut SurfaceAccess) {
        self.set_plumbing_geometry(geometry);
        self.homogenize(sa);
    }
}

// -------------------------------------------------------------------------------------------------
