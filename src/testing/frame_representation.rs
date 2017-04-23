// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Defines `FrameRepresentation` structure.

// -------------------------------------------------------------------------------------------------

use frames::{Frame, Parameters, Geometry};
use qualia::{Area, SurfaceId};

// -------------------------------------------------------------------------------------------------

/// This frame is simplified representation of `Frame` used in tests to define how frame tree
/// should look like and check if it is valid.
pub struct FrameRepresentation {
    pub params: Parameters,
    pub branches: Vec<FrameRepresentation>,
}

// -------------------------------------------------------------------------------------------------

// Constructing
impl FrameRepresentation {
    /// Creates representation of leaf `Frame`.
    pub fn new_leaf(sid: u64, geometry: Geometry) -> Self {
        FrameRepresentation {
            params: Parameters::new_leaf(SurfaceId::new(sid), geometry),
            branches: Vec::new(),
        }
    }

    /// Creates representation of whole frame tree with display of given area and with given
    /// workspaces.
    pub fn single_display(area: Area, workspaces: Vec<FrameRepresentation>) -> Self {
        FrameRepresentation {
            params: Parameters::new_root(),
            branches: vec![
                FrameRepresentation {
                    params: Parameters::new_display(area, String::default()),
                    branches: workspaces,
                },
            ]
        }
    }

    /// Creates representation of whole frame tree with single display of given area and single
    /// workspaces with given geometry and branches.
    pub fn single_workspace(area: Area,
                            geometry: Geometry,
                            branches: Vec<FrameRepresentation>) -> Self {
        FrameRepresentation {
            params: Parameters::new_root(),
            branches: vec![
                FrameRepresentation {
                    params: Parameters::new_display(area, String::default()),
                    branches: vec![
                        FrameRepresentation {
                            params: Parameters::new_workspace("1".to_owned(), geometry),
                            branches: branches,
                        }
                    ],
                },
            ]
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Validating
impl FrameRepresentation {
    /// Validates `Frame`s parameters by comparing with its representation.
    pub fn assert_frame(&self, frame: &Frame) {
        assert_eq!(frame.get_sid(), self.params.sid, "wrong sid");
        assert_eq!(frame.get_mode(), self.params.mode, "wrong mode");
        assert_eq!(frame.get_geometry(), self.params.geometry, "wrong geometry");
    }

    /// Validates whole spaced part of frame tree by comparing with its representation.
    pub fn assert_frames_spaced(&self, frame: &Frame) {
        self.assert_frame(frame);

        let mut frame_iter = frame.space_iter();
        let mut repr_iter = self.branches.iter();
        loop {
            let frame_item = frame_iter.next();
            let repr_item = repr_iter.next();

            if frame_item.is_some() && repr_item.is_some() {
                let next_frame = frame_item.unwrap();
                repr_item.unwrap().assert_frames_spaced(&next_frame);
                if let Some(ref parent) = next_frame.get_parent() {
                    self.assert_frame(parent);
                } else {
                    panic!("Parent not found");
                }
            } else if frame_item.is_none() && repr_item.is_none() {
                break;
            } else {
                panic!("Frame has unexpected length");
            }
        }
    }

    /// Validates whole timed part of frame tree by comparing with its representation.
    pub fn assert_frames_timed(&self, frame: &Frame) {
        self.assert_frame(frame);

        let mut frame_iter = frame.time_iter();
        let mut repr_iter = self.branches.iter();
        loop {
            let frame_item = frame_iter.next();
            let repr_item = repr_iter.next();

            if frame_item.is_some() && repr_item.is_some() {
                let next_frame = frame_item.unwrap();
                repr_item.unwrap().assert_frames_timed(&next_frame);
                if let Some(ref parent) = next_frame.get_parent() {
                    self.assert_frame(parent);
                } else {
                    panic!("Parent not found");
                }
            } else if frame_item.is_none() && repr_item.is_none() {
                break;
            } else {
                panic!("Frame has unexpected length");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
