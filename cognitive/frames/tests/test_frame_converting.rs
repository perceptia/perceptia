// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for displaying `Frame` functionality.

#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

extern crate cognitive_qualia as qualia;
extern crate cognitive_frames as frames;

mod common;

use qualia::{Position, SurfaceContext, SurfaceId, WorkspaceInfo, WorkspaceState};
use frames::Converting;
use common::layouts;
use common::surface_listing_mock::SurfaceListingMock;

// -------------------------------------------------------------------------------------------------

/// Checks if only one workspace will be visible on display.
#[test]
fn test_converting_two_workspaces_to_array() {
    let (r, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) =
        layouts::make_positioned_for_displaying();

    let surface_listing = SurfaceListingMock::new();

    let array = r.to_array(Position::new(1, 2), &surface_listing);
    let expected = vec![
        SurfaceContext::new(SurfaceId::new(101), Position::new(1, 2)),
        SurfaceContext::new(SurfaceId::new(102), Position::new(1, 102)),
        SurfaceContext::new(SurfaceId::new(1), Position::new(101, 102)),
        SurfaceContext::new(SurfaceId::new(2), Position::new(101, 102)),
        SurfaceContext::new(SurfaceId::new(3), Position::new(101, 102)),
        SurfaceContext::new(SurfaceId::new(4), Position::new(101, 102)),
        SurfaceContext::new(SurfaceId::new(5), Position::new(101, 202)),
        SurfaceContext::new(SurfaceId::new(6), Position::new(101, 302)),
    ];

    assert_eq!(array.len(), expected.len());

    for (context, expected_context) in array.iter().zip(expected) {
        assert_eq!(*context, expected_context);
    }

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Checks if frames are correctly converted to `WorkspaceState` structure.
///
/// - workspaces which are not direct children on display should be included
/// - workspaces under other workspaces should be ignored
/// - activeness should be preserved
#[test]
fn test_converting_to_workspaces() {
    let (r, _, _, _, _, _, _, _, _) = layouts::make_simple_with_workspaces();

    let mut expected = WorkspaceState::empty();
    expected.workspaces.insert(1, vec![WorkspaceInfo::new("11".to_string(), true),
                                       WorkspaceInfo::new("12".to_string(), false)]);
    expected.workspaces.insert(2, vec![WorkspaceInfo::new("21".to_string(), true),
                                       WorkspaceInfo::new("22".to_string(), false)]);

    assert_eq!(expected, r.to_workspace_state());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
