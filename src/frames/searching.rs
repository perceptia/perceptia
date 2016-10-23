// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra searching functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::SurfaceId;
use frame::Frame;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more search functionality.
pub trait Searching {
    /// Finds first frame suitable for building.
    /// Returns `self` if `self` has no surface ID set, its parent otherwise.
    fn find_buildable(&self) -> Option<Frame>;

    /// Find first trunk which is `Special`.
    /// For normal frame this should be workspace.
    fn find_top(&self) -> Option<Frame>;

    /// Find frame with given surface ID.
    fn find_with_sid(&self, sid: SurfaceId) -> Option<Frame>;
}

// -------------------------------------------------------------------------------------------------

impl Searching for Frame {
    fn find_buildable(&self) -> Option<Frame> {
        if self.get_sid().is_valid() {
            self.get_parent()
        } else {
            Some(self.clone())
        }
    }

    fn find_top(&self) -> Option<Frame> {
        let mut current = Some(self.clone());
        loop {
            current = if let Some(ref frame) = current {
                if frame.get_mode().is_top() {
                    return current.clone();
                }
                frame.get_parent()
            } else {
                return None;
            }
        }
    }

    fn find_with_sid(&self, sid: SurfaceId) -> Option<Frame> {
        if self.get_sid() == sid {
            Some(self.clone())
        } else {
            for subsurface in self.time_iter() {
                let result = subsurface.find_with_sid(sid);
                if result.is_some() {
                    return result;
                }
            }
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------
