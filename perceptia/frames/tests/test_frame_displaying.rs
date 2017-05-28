// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for displaying `Frame` functionality.

#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

extern crate frames;
extern crate cognitive_qualia as qualia;
extern crate testing;

mod common;

use qualia::{Position, SurfaceContext, SurfaceId};

use frames::Displaying;

use common::layouts;
use common::surface_listing_mock::SurfaceListingMock;

// -------------------------------------------------------------------------------------------------

/// Check if only one workspace will be visible on display.
#[test]
fn test_displaying_two_workspaces() {
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
