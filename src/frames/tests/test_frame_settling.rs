// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for settling `Frame` functionality.

// -------------------------------------------------------------------------------------------------

extern crate frames;
extern crate qualia;

mod common;

use frames::Parameters;
use frames::Geometry::{Horizontal, Stacked, Vertical};
use frames::settling::Settling;

use common::{assertions, layouts};
use common::frame_representation::FrameRepresentation;

// -------------------------------------------------------------------------------------------------

/// Test popping of directed frame.
///
/// Given frame should be popped as well as its parent.
/// Spatial order should be preserved.
#[test]
fn test_poping_directed() {
    let (mut r, _, _, _, _, _, _, _, mut h2, _, _, _, _)
      = layouts::make_simple_frames_appending();

    r.pop_recursively(&mut h2);

    let repr = FrameRepresentation {
        params: Parameters::new_root(),
        branches: vec![
            FrameRepresentation {
                params: Parameters::new_container(Horizontal),
                branches: vec![
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Vertical),
                branches: vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            },
        ]
    };

    assertions::assert_simple_frames_spaced(&r);
    repr.assert_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test popping of stacked frame.
///
/// Given frame should be popped as well as its parent.
/// Frames in stacked should also be popped in spatial order. 
#[test]
fn test_poping_stacked() {
    let (mut r, _, _, _, _, _, _, _, _, _, _, mut s2, _)
      = layouts::make_simple_frames_appending();

    r.pop_recursively(&mut s2);

    let spaced_repr = FrameRepresentation {
        params: Parameters::new_root(),
        branches: vec![
            FrameRepresentation {
                params: Parameters::new_container(Vertical),
                branches: vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Horizontal),
                branches: vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            },
        ]
    };

    let timed_repr = FrameRepresentation {
        params: Parameters::new_root(),
        branches: vec![
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Vertical),
                branches: vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Horizontal),
                branches: vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            },
        ]
    };

    spaced_repr.assert_frames_spaced(&r);
    timed_repr.assert_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
