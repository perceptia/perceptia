// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit test of simple Exhibitor scenarios.

#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate cognitive_qualia as qualia;
extern crate frames;
extern crate exhibitor;
extern crate testing;

use qualia::{OutputInfo, SurfaceId};
use qualia::{Area, Position, Size};
use qualia::{Action, Command, Direction};
use frames::Geometry::{Stacked, Vertical};
use frames::Mobility::{Anchored, Floating};
use frames::Parameters;
use exhibitor::{Exhibitor, Strategist};
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
        let mut exhibitor = Exhibitor::new(coordinator.clone(),
                                           Strategist::default(),
                                           testing::configurations::compositor());

        exhibitor.on_output_found(output);

        Environment {
            exhibitor: exhibitor,
            output_info: output_info,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Check if couple surfaces are managed correctly.
#[test]
fn test_creation_of_three_surfaces() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));

    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new_leaf(3, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(2, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3))
}

// -------------------------------------------------------------------------------------------------

/// Check if changing geometry via command works.
#[test]
fn test_configuring_geometry() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));

    e.exhibitor.on_command(Command {
        action: Action::Configure,
        direction: Direction::North,
        magnitude: 0,
        string: String::default(),
    });

    let repr = FrameRepresentation::single_workspace(e.output_info.area, Vertical,
        vec![
            FrameRepresentation::new_leaf(3, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(2, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3))
}

// -------------------------------------------------------------------------------------------------

/// Check if dive command works.
#[test]
fn test_diving() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));

    e.exhibitor.on_command(Command {
        action: Action::Configure,
        direction: Direction::North,
        magnitude: 0,
        string: String::default(),
    });

    e.exhibitor.on_command(Command {
        action: Action::Dive,
        direction: Direction::South,
        magnitude: 1, // TODO: Check case with magnitude = 0.
        string: String::default(),
    });

    let repr = FrameRepresentation::single_workspace(e.output_info.area, Vertical,
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(3, Vertical).with_mobility(Anchored),
                    FrameRepresentation::new_leaf(2, Vertical).with_mobility(Anchored),
                ]
            ),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3))
}

// -------------------------------------------------------------------------------------------------
