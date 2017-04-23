// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra displaying functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use frame::Frame;

use qualia::{SurfaceListing, SurfaceContext};

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more displaying functionality.
pub trait Displaying {
    fn to_array(&self, listing: &SurfaceListing) -> Vec<SurfaceContext>;
}

// -------------------------------------------------------------------------------------------------

impl Displaying for Frame {
    // TODO: Add unit tests.
    // TODO: Maybe make generic over `SurfaceListing`?
    fn to_array(&self, listing: &SurfaceListing) -> Vec<SurfaceContext> {
        // FIXME: Do not allocate here.
        let mut result = Vec::new();
        for frame in self.space_rev_iter() {
            if frame.get_sid().is_valid() {
                if let Some(ref mut array) = listing.get_renderer_context(frame.get_sid()) {
                    for ref mut c in array.iter() {
                        result.push(c.moved(frame.get_position()));
                    }
                }
            } else {
                result.append(&mut frame.to_array(listing));
            }
        }
        result
    }
}

// -------------------------------------------------------------------------------------------------
