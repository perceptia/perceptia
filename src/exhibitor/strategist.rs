// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides functionality related to compositor strategies.

// -------------------------------------------------------------------------------------------------

use qualia::{Area, Size, StrategistConfig, SurfaceInfo};
use frames::{self, Frame};

use strategies;

// -------------------------------------------------------------------------------------------------

/// Structure describing strategic decision about how to handle new surface.
pub struct TargetDecision {
    /// Target frame where new surface should be settled.
    pub target: Frame,

    /// Geometry of new frame.
    pub geometry: frames::Geometry,

    /// True if new frame should be selected. False otherwise.
    pub selection: bool,

    /// `Some` if frame should be floating. `None` otherwise.
    pub floating: Option<FloatingDecision>,
}

// -------------------------------------------------------------------------------------------------

/// Structure describing strategic decision about how to place floating frame.
pub struct FloatingDecision {
    /// Area of the frame.
    pub area: Area,
}

// -------------------------------------------------------------------------------------------------

type TargetDecider = fn(&Strategist, &Frame, &SurfaceInfo) -> TargetDecision;
type FloatingDecider = fn(&Strategist, Size, Option<Size>) -> FloatingDecision;

// -------------------------------------------------------------------------------------------------

/// Provides strategies used by `Compositor`.
///
/// Moving strategies outside `Compositor` simplifies its implementations and allows greater
/// customization or injecting functionalities.
pub struct Strategist {
    choose_target: TargetDecider,
    choose_floating: FloatingDecider,
}

// -------------------------------------------------------------------------------------------------

impl Strategist {
    /// Constructs new `Strategist`.
    pub fn new(choose_target: TargetDecider, choose_floating: FloatingDecider) -> Self {
        Strategist {
            choose_target: choose_target,
            choose_floating: choose_floating,
        }
    }

    /// Constructs new `Strategist`.
    pub fn new_from_config(config: StrategistConfig) -> Self {
        let mut strategist = Self::default();

        // NOTE: Using literals here is mediocre but otherwise `qualia` would have to define
        // strategies. Maybe that would be better?
        //
        // TODO: Provide macro for reading strategies from configuration.
        match config.choose_target.as_ref() {
            "always_floating" => {
                strategist.choose_target = strategies::choose_target_always_floating;
            }
            "anchored_but_popups" => {
                strategist.choose_target = strategies::choose_target_anchored_but_popups;
            }
            "" => {}
            _ => log_warn1!("Unknown 'choose_target' strategy: {}", config.choose_target),
        }

        match config.choose_floating.as_ref() {
            "always_centered" => {
                strategist.choose_floating = strategies::choose_floating_always_centered;
            }
            "random" => {
                strategist.choose_floating = strategies::choose_floating_random;
            }
            "" => {}
            _ => log_warn1!("Unknown 'choose_floating' strategy: {}", config.choose_floating),
        }

        strategist
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Strategist {
    fn default() -> Self {
        Strategist::new(strategies::choose_target_always_floating,
                        strategies::choose_floating_random)
    }
}

// -------------------------------------------------------------------------------------------------

// Strategy callers
impl Strategist {
    /// Decides how to handle new surface.
    pub fn choose_target(&self, frame: &Frame, surface: &SurfaceInfo) -> TargetDecision {
        (self.choose_target)(self, frame, surface)
    }

    /// Decides where to place floating surface.
    pub fn choose_floating(&self,
                           workspace_size: Size,
                           preferred_size: Option<Size>)
                           -> FloatingDecision {
        (self.choose_floating)(self, workspace_size, preferred_size)
    }
}

// -------------------------------------------------------------------------------------------------
