// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for basic `Frame` functionality like appending, prepending, joining, popping and removing.

// -------------------------------------------------------------------------------------------------

extern crate frames;

extern crate qualia;

mod common;

use frames::Parameters;
use frames::Geometry::{Horizontal, Stacked, Vertical};

use common::{assertions, layouts};
use common::frame_representation::FrameRepresentation;

// -------------------------------------------------------------------------------------------------

/// Check is simple frame layout is constructed correctly by appending all frames.
#[test]
fn test_append() {
    let r = layouts::make_simple_frames_appending().0;
    assertions::assert_simple_frames_timed(&r);
    assertions::assert_simple_frames_spaced(&r);
    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check is simple frame layout is constructed correctly by prepending all frames.
#[test]
fn test_prepend() {
    let r = layouts::make_simple_frames_prepending().0;
    assertions::assert_simple_frames_timed_reversed(&r);
    assertions::assert_simple_frames_spaced(&r);
    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check remove from begin, center and end works correctly.
#[test]
fn test_remove() {
    let (r, _, _, _, mut v1, _, _, _, mut h2, _, _, _, mut s3) =
        layouts::make_simple_frames_appending();

    // Remove chosen frames and destroy them.
    v1.remove();
    h2.remove();
    s3.remove();
    v1.destroy();
    h2.destroy();
    s3.destroy();

    // Prepare representation.
    let repr = FrameRepresentation {
        params: Parameters::new_root(),
        branches: vec![
            FrameRepresentation {
                params: Parameters::new_container(Vertical),
                branches: vec![
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Horizontal),
                branches: vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                ]
            },
        ]
    };

    repr.assert_frames_timed(&r);
    repr.assert_frames_spaced(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if popping surfaces works correctly. Test popping from the end and from inside. Spaced
/// order should not change.
#[test]
fn test_pop() {
    let (r, _, _, mut s, _, mut v2, _, _, _, _, _, _, mut s3) =
        layouts::make_simple_frames_appending();

    // Perform pop
    s.pop();
    s3.pop();
    v2.pop();

    // Check spaced layout.
    assertions::assert_simple_frames_spaced(&r);

    // Check timed layout.
    let time_repr = FrameRepresentation {
        params: Parameters::new_root(),
        branches: vec![
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![
                    FrameRepresentation::new_leaf(33, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                ]
            },
            FrameRepresentation {
                params: Parameters::new_container(Vertical),
                branches: vec![
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(11, Stacked),
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

    time_repr.assert_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if spaced order is correct when inserting frames at the begin, center and end.
#[test]
fn test_prejoin_adjoin() {
    let (r, _, _, _, mut v1, mut v2, _, mut h1, mut h2, _, mut s1, mut s2, _) =
        layouts::make_simple_frames_joining();

    // Pop some surfaces just to be able to use predefined timed representation
    v2.pop();
    v1.pop();
    h2.pop();
    h1.pop();
    s2.pop();
    s1.pop();

    // Assert layouts
    assertions::assert_simple_frames_spaced(&r);
    assertions::assert_simple_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test forward iteration in time.
#[test]
fn test_iteration_forward_in_time() {
    let (r, v, _, _, v1, v2, v3, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let mut iter = v.time_iter();
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v1);
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v2);
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v3);
    assert!(iter.next().is_none());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test backward iteration in time.
#[test]
fn test_iteration_backward_in_time() {
    let (r, v, _, _, v1, v2, v3, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let mut iter = v.time_rev_iter();
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v3);
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v2);
    assertions::assert_frame_equal_exact(&iter.next().unwrap(), &v1);
    assert!(iter.next().is_none());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
