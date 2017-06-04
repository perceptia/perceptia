// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Default configurations for tests.

use qualia::{CompositorConfig, StrategistConfig};

// -------------------------------------------------------------------------------------------------

pub fn compositor() -> CompositorConfig {
    CompositorConfig {
        move_step: 10,
        resize_step: 10,
    }
}

// -------------------------------------------------------------------------------------------------

pub fn strategist() -> StrategistConfig {
    StrategistConfig {
        choose_target: "always_floating".to_owned(),
        choose_floating: "random".to_owned(),
    }
}

// -------------------------------------------------------------------------------------------------
