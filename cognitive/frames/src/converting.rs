// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra functionality for converting `frames::Frame` to different
//! collections.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use frame::{Frame, Mode};

use qualia::{Position, SurfaceListing, SurfaceContext, WorkspaceInfo, WorkspaceState};

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more displaying functionality.
pub trait Converting {
    /// Converts frame three to list of `SurfaceContext` suitable for drawing by renderer.
    fn to_array(&self,
                relative_position: Position,
                listing: &SurfaceListing)
                -> Vec<SurfaceContext>;

    /// Converts frame tree to structure describing state of workspaces.
    fn to_workspace_state(&self) -> WorkspaceState;
}

// -------------------------------------------------------------------------------------------------

impl Converting for Frame {
    // TODO: Maybe make generic over `SurfaceListing`?
    // TODO: Do not allocate so much. Make benchmarks?
    fn to_array(&self,
                relative_position: Position,
                listing: &SurfaceListing)
                -> Vec<SurfaceContext> {
        let mut result = Vec::new();
        for frame in self.time_iter() {
            if let Mode::Workspace{is_active} = frame.get_mode() {
                if !is_active {
                    continue;
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
                result.append(&mut frame.to_array(pos, listing));
            }
        }
        result
    }

    fn to_workspace_state(&self) -> WorkspaceState {
        let mut state = WorkspaceState::empty();
        root_to_displays(self, &mut state.workspaces);
        state
    }
}

// -------------------------------------------------------------------------------------------------

fn root_to_displays(frame: &Frame, displays: &mut HashMap<i32, Vec<WorkspaceInfo>>) {
    if let Mode::Display{id} = frame.get_mode() {
        let mut workspaces = Vec::new();
        display_to_workspaces(frame, &mut workspaces);
        workspaces.sort();
        displays.insert(id, workspaces);
    } else {
        for subframe in frame.space_iter() {
            root_to_displays(&subframe, displays);
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn display_to_workspaces(frame: &Frame, workspaces: &mut Vec<WorkspaceInfo>) {
    if let Mode::Workspace{is_active} = frame.get_mode() {
        workspaces.push(WorkspaceInfo::new(frame.get_title(), is_active));
    } else {
        for subframe in frame.space_iter() {
            display_to_workspaces(&subframe, workspaces);
        }
    }
}

// -------------------------------------------------------------------------------------------------
