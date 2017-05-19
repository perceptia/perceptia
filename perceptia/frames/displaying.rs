// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra displaying functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use frame::Frame;

use qualia::{Position, SurfaceListing, SurfaceContext};

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more displaying functionality.
pub trait Displaying {
    fn to_array(&self,
                relative_position: Position,
                listing: &SurfaceListing)
                -> Vec<SurfaceContext>;
}

// -------------------------------------------------------------------------------------------------

impl Displaying for Frame {
    // TODO: Maybe make generic over `SurfaceListing`?
    // TODO: Do not allocate so much. Make benchmarks?
    fn to_array(&self,
                relative_position: Position,
                listing: &SurfaceListing)
                -> Vec<SurfaceContext> {
        let mut skip_workspace = false;
        to_array(self, relative_position, listing, &mut skip_workspace)
    }
}

// -------------------------------------------------------------------------------------------------

fn to_array(main_frame: &Frame,
            relative_position: Position,
            listing: &SurfaceListing,
            skip_workspace: &mut bool)
            -> Vec<SurfaceContext> {
    let mut result = Vec::new();
    for frame in main_frame.time_iter() {
        if frame.get_mode().is_workspace() {
            if *skip_workspace {
                continue;
            } else {
                *skip_workspace = true;
            }
        }

        let pos = relative_position + frame.get_position();
        if frame.get_sid().is_valid() {
            if let Some(ref mut array) = listing.get_renderer_context(frame.get_sid()) {
                for ref mut c in array.iter().rev() {
                    result.push(c.moved(pos));
                }
            }
        } else {
            result.append(&mut to_array(&frame, pos, listing, skip_workspace));
        }
    }
    result
}

// -------------------------------------------------------------------------------------------------
