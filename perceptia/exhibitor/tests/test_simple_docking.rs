// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit test of simple docking scenarios.

#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate cognitive_qualia as qualia;
extern crate frames;
extern crate exhibitor;
extern crate testing;

use qualia::{OutputInfo, SurfaceId};
use qualia::{Area, Position, Size};
use frames::Geometry::{Stacked, Vertical};
use frames::Mobility::{Docked, Floating};
use frames::Parameters;
use exhibitor::{Exhibitor, Strategist};
use testing::frame_representation::FrameRepresentation;
use testing::output_mock::OutputMock;
use testing::coordinator_mock::CoordinatorMock;
use testing::exhibitor_mixins::ExhibitorCommandShorthands;

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

/// Creates single workspace and then new dock. Workspace should be in new ramified container with
/// correct area.
#[test]
fn test_adding_dock() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_dock_surface(SurfaceId::new(2),
                                Size::new(e.output_info.area.size.width, 10),
                                e.output_info.id);

    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(e.output_info.area, e.output_info.make.clone()),
                vec![
                    FrameRepresentation::new_leaf(2, Stacked)
                                        .with_mobility(Docked)
                                        .with_area(0, 0, e.output_info.area.size.width, 10),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new(
                                Parameters::new_workspace("1".to_owned(), Stacked),
                                vec![FrameRepresentation::new_leaf(1, Vertical)
                                                         .with_mobility(Floating)]
                            ).with_area(0, 0, 100, 90),
                        ]
                    ).with_area(0, 10, 100, 90),
                ]
            ).with_geometry(Vertical),
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1))
}

// -------------------------------------------------------------------------------------------------

/// Creates single workspace, new dock and then new workspace. Both workspaces should be in new
/// ramified container with correct area.
#[test]
fn test_adding_dock_and_workspace() {
    let mut e = Environment::create();
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_dock_surface(SurfaceId::new(2),
                                Size::new(e.output_info.area.size.width, 10),
                                e.output_info.id);
    e.exhibitor.focus_workspace("2");
    e.exhibitor.on_surface_ready(SurfaceId::new(3));

    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(e.output_info.area, e.output_info.make.clone()),
                vec![
                    FrameRepresentation::new_leaf(2, Stacked)
                                        .with_mobility(Docked)
                                        .with_area(0, 0, e.output_info.area.size.width, 10),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new(
                                Parameters::new_workspace("2".to_owned(), Stacked),
                                vec![FrameRepresentation::new_leaf(3, Vertical)
                                                         .with_mobility(Floating)]
                            ).with_area(0, 0, 100, 90),
                            FrameRepresentation::new(
                                Parameters::new_workspace("1".to_owned(), Stacked),
                                vec![FrameRepresentation::new_leaf(1, Vertical)
                                                         .with_mobility(Floating)]
                            ).with_area(0, 0, 100, 90),
                        ]
                    ).with_area(0, 10, 100, 90),
                ]
            ).with_geometry(Vertical),
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(3))
}

// -------------------------------------------------------------------------------------------------

/// Creates single workspace, new dock and then new display.
#[test]
fn test_adding_dock_and_display() {
    let mut e = Environment::create();
    let output2_info = OutputInfo::new(2,
                                       Area::new(Position::new(100, 0), Size::new(200, 200)),
                                       Size::new(200, 200),
                                       60,
                                       "test_make_2".to_owned(),
                                       "test_model_2".to_owned());

    let output2 = Box::new(OutputMock::new(output2_info.clone()));

    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    e.exhibitor.on_dock_surface(SurfaceId::new(2),
                                Size::new(e.output_info.area.size.width, 10),
                                e.output_info.id);
    e.exhibitor.on_output_found(output2);
    e.exhibitor.on_dock_surface(SurfaceId::new(3),
                                Size::new(output2_info.area.size.width, 10),
                                output2_info.id);
    e.exhibitor.on_surface_ready(SurfaceId::new(4));

    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(output2_info.area, output2_info.make.clone()),
                vec![
                    FrameRepresentation::new_leaf(3, Stacked)
                                        .with_mobility(Docked)
                                        .with_area(0, 0, output2_info.area.size.width, 10),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new(
                                Parameters::new_workspace("2".to_owned(), Stacked),
                                vec![FrameRepresentation::new_leaf(4, Vertical)
                                                         .with_mobility(Floating)]
                            ).with_area(0, 0, 200, 190),
                        ]
                    ).with_area(0, 10, 200, 190),
                ]
            ).with_geometry(Vertical),
            FrameRepresentation::new(
                Parameters::new_display(e.output_info.area, e.output_info.make.clone()),
                vec![
                    FrameRepresentation::new_leaf(2, Stacked)
                                        .with_mobility(Docked)
                                        .with_area(0, 0, e.output_info.area.size.width, 10),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new(
                                Parameters::new_workspace("1".to_owned(), Stacked),
                                vec![FrameRepresentation::new_leaf(1, Vertical)
                                                         .with_mobility(Floating)]
                            ).with_area(0, 0, 100, 90),
                        ]
                    ).with_area(0, 10, 100, 90),
                ]
            ).with_geometry(Vertical),
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(4))
}

// -------------------------------------------------------------------------------------------------
