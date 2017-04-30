// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides strategies to be used by `Strategist`.

// TODO: Add strategy to place floating frame in random corner.

#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------

use rand;
use rand::distributions::{IndependentSample, Range};

use qualia::{Area, Position, Size, SurfaceInfo};

use frames::{self, Frame};
use frames::searching::Searching;

use strategist::{TargetDecision, FloatingDecision, Strategist};

// -------------------------------------------------------------------------------------------------

/// Decides how to handle new surface.
///
/// The surface will always be floating. `choose_floating` strategy is used to choose position.
pub fn choose_target_always_floating(strategist: &Strategist,
                                     selection: &Frame,
                                     surface: &SurfaceInfo)
                                     -> TargetDecision {
    let preferred_size = if !surface.requested_size.is_zero() {
        Some(surface.requested_size)
    } else {
        None
    };

    let workspace = selection.find_top().expect("searching workspace for floating");
    let floating = strategist.choose_floating(workspace.get_size(), preferred_size);
    TargetDecision {
        target: workspace,
        geometry: frames::Geometry::Vertical,
        selection: true,
        floating: Some(floating),
    }
}

// -------------------------------------------------------------------------------------------------

/// Decides how to handle new surface.
///
/// Toplevel surfaces will be anchored and popup will be floating. If anchored new frame will
/// become sibling of selection. If floating new frame the `always_floating` strategy will be used.
pub fn choose_target_anchored_but_popups(strategist: &Strategist,
                                         selection: &Frame,
                                         surface: &SurfaceInfo)
                                         -> TargetDecision {
    if !surface.parent_sid.is_valid() {
        let target = selection.find_buildable().expect("searching buildable for anchorization");
        TargetDecision {
            target: target,
            geometry: frames::Geometry::Stacked,
            selection: true,
            floating: None,
        }
    } else {
        choose_target_always_floating(strategist, selection, surface)
    }
}

// -------------------------------------------------------------------------------------------------

/// Decides where to place floating surface.
///
/// The frame will always be centered. If preferred size was not provided the frame will have 1/2
/// of width and height of workspace.
pub fn choose_floating_always_centered(_strategist: &Strategist,
                                       workspace_size: Size,
                                       preferred_size: Option<Size>)
                                       -> FloatingDecision {
    let size = if let Some(preferred_size) = preferred_size {
        preferred_size
    } else {
        workspace_size.scaled(0.5)
    };
    let pos = Position::new((workspace_size.width / 4) as isize,
                            (workspace_size.height / 4) as isize);
    FloatingDecision {
        area: Area::new(pos, size),
    }
}

// -------------------------------------------------------------------------------------------------

/// Decides where to place floating surface.
///
/// The frame will be placed in random position in workspace. If preferred size was not provided
/// the frame will have 1/2 of width and height of workspace.
pub fn choose_floating_random(_strategist: &Strategist,
                              workspace_size: Size,
                              preferred_size: Option<Size>)
                              -> FloatingDecision {
    let mut rng = rand::thread_rng();

    let size = if let Some(preferred_size) = preferred_size {
        preferred_size
    } else {
        workspace_size.scaled(0.5)
    };

    let x_range = Range::new(0, workspace_size.width - size.width);
    let y_range = Range::new(0, workspace_size.height - size.height);
    let pos = Position::new(x_range.ind_sample(&mut rng) as isize,
                            y_range.ind_sample(&mut rng) as isize);
    FloatingDecision {
        area: Area::new(pos, size),
    }
}

// -------------------------------------------------------------------------------------------------
