// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Regression tests for double display cases.

#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate qualia;
extern crate frames;
extern crate exhibitor;
extern crate testing;

use qualia::{OutputInfo, SurfaceId};
use qualia::{Area, Position, Size};
use qualia::CompositorConfig;
use frames::Geometry::{Stacked, Vertical};
use frames::Parameters;
use exhibitor::{Exhibitor, Strategist};
use testing::frame_representation::FrameRepresentation;
use testing::output_mock::OutputMock;
use testing::coordinator_mock::CoordinatorMock;
use testing::exhibitor_mixins::ExhibitorCommandShorthands;

// -------------------------------------------------------------------------------------------------

struct Environment {
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

        let coordinator = CoordinatorMock::new();
        let mut exhibitor = Exhibitor::new(coordinator.clone(),
                                           strategist,
                                           CompositorConfig::default());

        exhibitor.on_output_found(output1);
        exhibitor.on_output_found(output2);

        Environment {
            exhibitor: exhibitor,
            output1_info: output1_info,
            output2_info: output2_info,
        }
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
                Parameters::new_display(e.output2_info.area, e.output2_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("2".to_owned(), Stacked),
                        vec![FrameRepresentation::new_leaf(1, Vertical)]
                    )
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_display(e.output1_info.area, e.output1_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("1".to_owned(), Stacked),
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
                Parameters::new_display(e.output2_info.area, e.output2_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("2".to_owned(), Stacked),
                        Vec::new()
                    )
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_display(e.output1_info.area, e.output1_info.make.clone()),
                vec![
                    FrameRepresentation::new(
                        Parameters::new_workspace("1".to_owned(), Stacked),
                        vec![FrameRepresentation::new_leaf(1, Vertical)]
                    )
                ]
            )
        ]
    );

    repr.assert_frames_spaced(&e.exhibitor.get_root());
}

// -------------------------------------------------------------------------------------------------
