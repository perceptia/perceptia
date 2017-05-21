// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains assertions for checking validity of frame layouts contained in module
//! `layouts`.

#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

use frames::{Frame, Parameters};
use frames::Geometry::{Horizontal, Stacked, Vertical};

use testing::frame_representation::FrameRepresentation;

use qualia::{Position, Size};

// -------------------------------------------------------------------------------------------------

/// Assert if given two frames are exactly the same by comparing their internals.
pub fn assert_frame_equal_exact(frame1: &Frame, frame2: &Frame) {
    assert!(frame1.equals_exact(frame2),
            "Frames are not exactly equal:\n\t{:?}\n\t{:?}",
            frame1,
            frame2);
}

// -------------------------------------------------------------------------------------------------

/// Validate timed part if `simple` layout.
pub fn assert_simple_frames_timed(frame: &Frame) {
    let repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            ),
        ]
    );

    repr.assert_frames_timed(frame);
}

// -------------------------------------------------------------------------------------------------

/// Validate timed part if `reversed simple` layout.
pub fn assert_simple_frames_timed_reversed(frame: &Frame) {
    let repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(33, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(23, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(21, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(13, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(11, Stacked),
                ]
            ),
        ]
    );

    repr.assert_frames_timed(frame);
}

// -------------------------------------------------------------------------------------------------

/// Validate spaced part if `simple` layout.
pub fn assert_simple_frames_spaced(frame: &Frame) {
    let repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            ),
        ]
    );

    repr.assert_frames_spaced(frame);
}

// -------------------------------------------------------------------------------------------------

/// Assert frame area by comparing frames area with expected area.
///
/// TODO: Extend this assertion to also check area set to `SurfaceAccess`.
pub fn assert_area(frame: &Frame, pos: Position, size: Size) {
    assert_eq!(pos, frame.get_position(), "");
    assert_eq!(size, frame.get_size(), "");
}

// -------------------------------------------------------------------------------------------------
