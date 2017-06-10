// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Regression tests for double display cases.

#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate cognitive_qualia as qualia;
extern crate cognitive_outputs as outputs;
extern crate cognitive_frames as frames;
extern crate cognitive_exhibitor as exhibitor;

mod common;

use qualia::{InteractionMode, OutputInfo, SurfaceId};
use qualia::{Area, Position, Size, Vector};
use qualia::coordinator_mock::CoordinatorMock;
use outputs::output_mock::OutputMock;
use frames::Geometry::{Stacked, Vertical};
use frames::Mobility::Floating;
use frames::Parameters;
use frames::representation::FrameRepresentation;
use exhibitor::{Exhibitor, Strategist};
use common::exhibitor_mixins::ExhibitorCommandShorthands;

// -------------------------------------------------------------------------------------------------

struct Environment {
    coordinator_mock: CoordinatorMock,
    exhibitor: Exhibitor<CoordinatorMock>,
    output1_info: OutputInfo,
    output2_info: OutputInfo,
}

// -------------------------------------------------------------------------------------------------

impl Environment {
    pub fn create(strategist: Strategist) -> Self {
        let output1_info = OutputInfo::new(1,
                                           Area::new(Position::new(0, 0), Size::new(100, 100)),
                                           Size::new(100, 100),
                                           60,
                                           "test_make_1".to_owned(),
                                           "test_model_1".to_owned());

        let output2_info = OutputInfo::new(2,
                                           Area::new(Position::new(100, 0), Size::new(200, 200)),
                                           Size::new(200, 200),
                                           60,
                                           "test_make_2".to_owned(),
                                           "test_model_2".to_owned());

        let output1 = Box::new(OutputMock::new(output1_info.clone()));
        let output2 = Box::new(OutputMock::new(output2_info.clone()));

        let coordinator_mock = CoordinatorMock::new();
        let mut exhibitor = Exhibitor::new(coordinator_mock.clone(),
                                           std::time::Instant::now(),
                                           strategist,
                                           common::configurations::compositor());

        exhibitor.on_output_found(output1);
        exhibitor.on_output_found(output2);

        Environment {
            coordinator_mock: coordinator_mock,
            exhibitor: exhibitor,
            output1_info: output1_info,
            output2_info: output2_info,
        }
    }

    pub fn redraw(&mut self) {
        self.exhibitor.on_notify();
        self.exhibitor.on_pageflip(1);
        self.exhibitor.on_pageflip(2);
    }

    pub fn create_surface(&mut self, id: u64) {
        let sid = SurfaceId::new(id);
        self.coordinator_mock.add_surface(sid);
        self.exhibitor.on_surface_ready(sid);
    }
}

// -------------------------------------------------------------------------------------------------

/// Test moving one and only surface to empty workspace on different display.
#[test]
fn test_moving_surface_between_displays() {
    let mut e = Environment::create(Strategist::default());

    // Make one surface
    e.exhibitor.on_surface_ready(SurfaceId::new(1));
    assert!(e.exhibitor.get_selection().get_sid() == SurfaceId::new(1));

    // Check structure
    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(2, e.output2_info.area, e.output2_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("2".to_owned(), Stacked, true),
                        vec![FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating)]
                    )
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_display(1, e.output1_info.area, e.output1_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("1".to_owned(), Stacked, true),
                        Vec::new()
                    )
                ]
            )
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());

    // Jump the surface
    e.exhibitor.jump_to_workspace("1");

    // Check structure
    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(2, e.output2_info.area, e.output2_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("2".to_owned(), Stacked, true),
                        Vec::new()
                    )
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_display(1, e.output1_info.area, e.output1_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("1".to_owned(), Stacked, true),
                        vec![FrameRepresentation::new_leaf(1, Vertical).with_mobility(Floating)]
                    )
                ]
            )
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------

/// Test dragging surface by switching visual mode on and moving pointer. If visual mode is off the
/// surface should not be dragged.
#[test]
fn test_dragging_surface_in_visual_mode() {
    let mut config = common::configurations::strategist();
    config.choose_target = "always_floating".to_owned();
    config.choose_floating = "always_centered".to_owned();
    let strategist = Strategist::new_from_config(config);
    let mut e = Environment::create(strategist);

    let vector = Vector::new(10, 20);

    // Make one surface and redraw to update hover state
    e.exhibitor.focus_workspace("1");
    e.create_surface(1);
    e.redraw();
    let selection = e.exhibitor.get_selection();
    let mut area = selection.get_area();

    // Switch visual mode on and move cursor
    e.exhibitor.on_mode_switched(true, InteractionMode::Visual);
    e.exhibitor.on_motion(vector);
    area.pos = area.pos + vector;
    assert_eq!(e.exhibitor.get_selection().get_area().pos, area.pos);

    // After switching visual mode off nothing should be moved
    e.exhibitor.on_mode_switched(false, InteractionMode::Visual);
    e.exhibitor.on_motion(vector);
    assert_eq!(e.exhibitor.get_selection().get_area().pos, area.pos);
}

// -------------------------------------------------------------------------------------------------

/// Test dragging surface by switching visual mode on and moving pointer to another display. The
/// surface should then be resettled to corresponding workspace. If visual mode is off the surface
/// should not be dragged.
#[test]
fn test_dragging_surface_in_visual_mode_to_different_display() {
    let mut config = common::configurations::strategist();
    config.choose_target = "always_floating".to_owned();
    config.choose_floating = "always_centered".to_owned();
    let strategist = Strategist::new_from_config(config);
    let mut e = Environment::create(strategist);

    // Make one surface and redraw to update hover state
    e.exhibitor.focus_workspace("1");
    e.create_surface(1);
    e.redraw();
    let selection = e.exhibitor.get_selection();
    let area = selection.get_area();

    // Switch visual mode on and move cursor to different display
    let vector = Vector::new(110, 20);
    e.exhibitor.on_mode_switched(true, InteractionMode::Visual);
    e.exhibitor.on_motion(vector);

    let new_pos = Position::new(35, 45);

    // Check structure
    let repr = FrameRepresentation::new(
        Parameters::new_root(),
        vec![
            FrameRepresentation::new(
                Parameters::new_display(2, e.output2_info.area, e.output2_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("2".to_owned(), Stacked, true),
                        vec![FrameRepresentation::new_leaf(1, Vertical)
                            .with_mobility(Floating)
                            .with_area(new_pos.x, new_pos.y, area.size.width, area.size.height)
                        ]
                    )
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_display(1, e.output1_info.area, e.output1_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("1".to_owned(), Stacked, true),
                        Vec::new()
                    )
                ]
            ),
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());

    // After switching visual mode off nothing should be moved
    let vector = Vector::new(10, 20);
    e.exhibitor.on_mode_switched(false, InteractionMode::Visual);
    e.exhibitor.on_motion(vector);
    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------
