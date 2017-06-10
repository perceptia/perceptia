// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Regression tests for single workspace cases.

#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate cognitive_qualia as qualia;
extern crate cognitive_outputs as outputs;
extern crate cognitive_frames as frames;
extern crate cognitive_exhibitor as exhibitor;

mod common;

use qualia::{OutputInfo, SurfaceId};
use qualia::{Area, Position, Size};
use qualia::coordinator_mock::CoordinatorMock;
use outputs::output_mock::OutputMock;
use frames::Geometry::{Horizontal, Stacked, Vertical};
use frames::Mobility::{Anchored, Floating};
use frames::Parameters;
use frames::representation::FrameRepresentation;
use exhibitor::{Exhibitor, Strategist};
use common::exhibitor_mixins::ExhibitorCommandShorthands;

// -------------------------------------------------------------------------------------------------

struct Environment {
    exhibitor: Exhibitor<CoordinatorMock>,
    output_info: OutputInfo,
}

// -------------------------------------------------------------------------------------------------

impl Environment {
    pub fn create(strategist: Strategist) -> Self {
        let output_info = OutputInfo::new(1,
                                          Area::new(Position::new(0, 0), Size::new(100, 100)),
                                          Size::new(100, 100),
                                          60,
                                          "test_make".to_owned(),
                                          "test_model".to_owned());

        let output = Box::new(OutputMock::new(output_info.clone()));
        let coordinator = CoordinatorMock::new();
        let mut exhibitor = Exhibitor::new(coordinator.clone(),
                                           std::time::Instant::now(),
                                           strategist,
                                           common::configurations::compositor());

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
    let mut e = Environment::create(Strategist::default());
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));

    // Exalt selected frame
    e.exhibitor.exalt();

    // Check structure was not changed
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new_leaf(2, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
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
    let mut e = Environment::create(Strategist::default());

    // Make three surfaces
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3));

    // Ramify selected frame
    e.exhibitor.ramify();

    // Check ramification
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![FrameRepresentation::new_leaf(3, Vertical).with_mobility(Anchored)],
            ),
            FrameRepresentation::new_leaf(2, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());

    // Destroy focused ramified frame
    e.exhibitor.on_surface_destroyed(SurfaceId::new(3));

    // Check selection and structure
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(2));

    let repr = FrameRepresentation::single_workspace(e.output_info.area, Stacked,
        vec![
            FrameRepresentation::new_leaf(2, Vertical).with_mobility(Floating),
            FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating),
        ]);

    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------

/// Test creating vertical layout containing two horizontal frames with two surfaces each. Then
/// dive one of them.
///
/// Buggy implementation placed ramified and dived surfaces on wrong positions.
#[test]
fn test_create_layout_of_four() {
    let mut config = common::configurations::strategist();
    config.choose_target = "anchored_but_popups".to_owned();
    let strategist = Strategist::new_from_config(config);
    let mut e = Environment::create(strategist);

    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_surface_ready(SurfaceId::new(2));
    e.exhibitor.on_surface_ready(SurfaceId::new(3));
    e.exhibitor.on_surface_ready(SurfaceId::new(4));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(4));

    // Create layout
    e.exhibitor.verticalize();
    e.exhibitor.ramify();
    e.exhibitor.focus_down();
    e.exhibitor.dive_up();
    e.exhibitor.horizontalize();
    e.exhibitor.focus_down();
    e.exhibitor.focus_down();
    e.exhibitor.dive_up();
    e.exhibitor.horizontalize();

    // Check structure
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Vertical,
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(3, Stacked).with_area( 0, 0, 50, 50),
                    FrameRepresentation::new_leaf(4, Stacked).with_area(50, 0, 50, 50),
                ],
            ).with_area(0, 0, 100, 50),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(1, Stacked).with_area( 0, 0, 50, 50),
                    FrameRepresentation::new_leaf(2, Stacked).with_area(50, 0, 50, 50),
                ],
            ).with_area(0, 50, 100, 50),
        ]);

    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));
    repr.assert_frames_spaced(&e.exhibitor.get_root());

    // Alternate layout
    e.exhibitor.focus_right();
    e.exhibitor.dive_up();

    // Check structure
    let repr = FrameRepresentation::single_workspace(e.output_info.area, Vertical,
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(3, Stacked).with_area(0, 0, 50, 50),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new_leaf(2, Stacked).with_area(0, 0, 50, 50),
                            FrameRepresentation::new_leaf(4, Stacked).with_area(0, 0, 50, 50),
                        ],
                    ).with_area(50, 0, 50, 50),
                ],
            ).with_area(0, 0, 100, 50),
            FrameRepresentation::new_leaf(1, Horizontal).with_area(0, 50, 100, 50),
        ]);

    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(2));
    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------

/// Tests if frames did not change size after creating new workspace.
///
/// In buggy implementation floating frames were resized during relaxation after creating new
/// workspace.
#[test]
fn test_frame_size_after_creating_new_workspace() {
    let mut config = common::configurations::strategist();
    config.choose_target = "always_floating".to_owned();
    let strategist = Strategist::new_from_config(config);
    let mut e = Environment::create(strategist);

    // Create one floating frame and remember its area.
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));
    let area_before = e.exhibitor.get_selection().get_area();

    // Create new workspace.
    e.exhibitor.focus_workspace("2");
    e.exhibitor.focus_workspace("1");

    // Check if the frame has preserved the area.
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));
    let area_after = e.exhibitor.get_selection().get_area();
    assert_eq!(area_before, area_after);
}

// -------------------------------------------------------------------------------------------------
