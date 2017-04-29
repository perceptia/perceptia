// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Regression tests for single workspace cases.

extern crate qualia;
extern crate frames;
extern crate exhibitor;
extern crate testing;

use qualia::{OutputInfo, SurfaceId};
use qualia::{Area, Position, Size};
use qualia::{Action, Command, Direction};
use frames::Geometry::{Stacked, Vertical};
use frames::Parameters;
use exhibitor::Exhibitor;
use testing::frame_representation::FrameRepresentation;
use testing::output_mock::OutputMock;
use testing::coordinator_mock::CoordinatorMock;

// -------------------------------------------------------------------------------------------------

struct Environment {
    exhibitor: Exhibitor<CoordinatorMock>,
    output_info: OutputInfo,
}

// -------------------------------------------------------------------------------------------------

impl Environment {
    pub fn create() -> Self {
        let output_info = OutputInfo::new(1,
                                          Area::new(Position::new(0, 0), Size::new(100, 100)),
                                          Size::new(100, 100),
                                          60,
                                          "test_make".to_owned(),
                                          "test_model".to_owned());

        let output = Box::new(OutputMock::new(output_info.clone()));
        let coordinator = CoordinatorMock::new();
        let mut exhibitor = Exhibitor::new(coordinator.clone());

        exhibitor.on_output_found(output);

        Environment {
            exhibitor: exhibitor,
            output_info: output_info,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Check exaltation of frame which can not be exalted more.
///
/// In buggy implementation there was no limit for exaltation to most exalted frame was landing
/// among workspaces.
#[test]
fn test_exaltation_of_the_most_exalted() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));

    // Exalt selected frame
    e.exhibitor.on_command(Command {
        action: Action::Jump,
        direction: Direction::Begin,
        magnitude: 0,
        string: String::default(),
    });

    // Check structure was not changed
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new_leaf(2, Vertical),
            FrameRepresentation::new_leaf(1, Vertical),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------

/// Check selection after unmanaging selected ramified frame.
///
/// Buggy implementation always selected buildable parent but in case of removing frame with no
/// siblings parent was also removed so selection was lost.
#[test]
fn test_selection_after_unmanaging_ramified() {
    let mut e = Environment::create();

    // Make three surfaces
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3));

    // Ramify selected frame
    e.exhibitor.on_command(Command {
        action: Action::Jump,
        direction: Direction::End,
        magnitude: 0,
        string: String::default(),
    });

    // Check ramification
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation {
                params: Parameters::new_container(Stacked),
                branches: vec![FrameRepresentation::new_leaf(3, Vertical)],
            },
            FrameRepresentation::new_leaf(2, Vertical),
            FrameRepresentation::new_leaf(1, Vertical),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());

    // Destroy focused ramified frame
    e.exhibitor.on_surface_destroyed(SurfaceId::new(3));

    // Check selection and structure
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(2));

    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new_leaf(2, Vertical),
            FrameRepresentation::new_leaf(1, Vertical),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------
