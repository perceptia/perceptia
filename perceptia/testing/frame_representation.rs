// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Defines `FrameRepresentation` structure.

// -------------------------------------------------------------------------------------------------

use frames::{Frame, Parameters, Geometry, Mobility, Mode};
use qualia::{Area, Position, Size, SurfaceId};

// -------------------------------------------------------------------------------------------------

/// This frame is simplified representation of `Frame` used in tests to define how frame tree
/// should look like and check if it is valid.
pub struct FrameRepresentation {
    pub params: Parameters,
    pub branches: Vec<FrameRepresentation>,
    pub has_area: bool,
}

// -------------------------------------------------------------------------------------------------

// Constructing
impl FrameRepresentation {
    /// Creates representation of leaf `Frame`.
    pub fn new(params: Parameters, branches: Vec<FrameRepresentation>) -> Self {
        FrameRepresentation {
            has_area: (params.mode == Mode::Display),
            params: params,
            branches: branches,
        }
    }

    /// Creates representation of leaf `Frame`.
    pub fn new_leaf(sid: u64, geometry: Geometry) -> Self {
        FrameRepresentation {
            params: Parameters::new_leaf(SurfaceId::new(sid), geometry),
            branches: Vec::new(),
            has_area: false,
        }
    }

    /// Sets additional conditions to check frame area.
    pub fn with_area(mut self, x: isize, y: isize, width: usize, height: usize) -> Self {
        self.params.pos = Position::new(x, y);
        self.params.size = Size::new(width, height);
        self.has_area = true;
        self
    }

    /// Modifies conditions for geometry.
    pub fn with_geometry(mut self, geometry: Geometry) -> Self {
        self.params.geometry = geometry;
        self
    }

    /// Modifies conditions for mobility.
    pub fn with_mobility(mut self, mobility: Mobility) -> Self {
        self.params.mobility = mobility;
        self
    }

    /// Creates representation of whole frame tree with display of given area and with given
    /// workspaces.
    pub fn single_display(area: Area, workspaces: Vec<FrameRepresentation>) -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        FrameRepresentation::new(
            Parameters::new_root(),
            vec![
                FrameRepresentation::new(
                    Parameters::new_display(area, String::default()),
                    workspaces,
                )
            ]
        )
    }

    /// Creates representation of whole frame tree with single display of given area and single
    /// workspaces with given geometry and branches.
    pub fn single_workspace(area: Area,
                            geometry: Geometry,
                            branches: Vec<FrameRepresentation>)
                            -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        FrameRepresentation::new(
            Parameters::new_root(),
            vec![
                FrameRepresentation::new(
                    Parameters::new_display(area, String::default()),
                    vec![
                        FrameRepresentation::new(
                            Parameters::new_workspace("1".to_owned(), geometry),
                            branches
                        ).with_area(area.pos.x, area.pos.y, area.size.width, area.size.height)
                    ]
                )
            ]
        )
    }
}

// -------------------------------------------------------------------------------------------------

// Validating
impl FrameRepresentation {
    /// Validates `Frame`s parameters by comparing with its representation.
    pub fn assert_frame(&self, frame: &Frame) {
        assert_eq!(frame.get_sid(), self.params.sid, "wrong sid");
        assert_eq!(frame.get_geometry(),
                   self.params.geometry,
                   "wrong geometry in {:?}",
                   frame.get_sid());
        assert_eq!(frame.get_mobility(),
                   self.params.mobility,
                   "wrong mobility in {:?}",
                   frame.get_sid());
        assert_eq!(frame.get_mode(),
                   self.params.mode,
                   "wrong mode in {:?}",
                   frame.get_sid());

        if self.params.mode == Mode::Workspace {
            assert_eq!(frame.get_title(),
                       self.params.title,
                       "wrong title in {:?}",
                       frame.get_sid());
        }

        if self.has_area {
            assert_eq!(frame.get_position(),
                       self.params.pos,
                       "wrong position in {:?}",
                       frame.get_sid());
            assert_eq!(frame.get_size(),
                       self.params.size,
                       "wrong size {:?}",
                       frame.get_sid());
        }
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
