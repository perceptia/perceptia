// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Default configuration for `perceptia`.

// -------------------------------------------------------------------------------------------------

use uinput_sys;

use qualia::modifier;
pub use qualia::{AestheticsConfig, CompositorConfig, KeyboardConfig};
pub use qualia::{ExhibitorConfig, InputConfig, StrategistConfig};

use config::{BindingEntry, Config, KeybindingsConfig};
use binding_functions;

// -------------------------------------------------------------------------------------------------

/// Trait for creating default configuration.
pub trait DefaultConfig {
    fn default() -> Self;
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for Config {
    fn default() -> Self {
        Config::new(AestheticsConfig::default(),
                    ExhibitorConfig::default(),
                    InputConfig::default(),
                    KeyboardConfig::default(),
                    KeybindingsConfig::default())
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for AestheticsConfig {
    fn default() -> Self {
        AestheticsConfig { background_path: None }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for CompositorConfig {
    fn default() -> Self {
        CompositorConfig {
            move_step: 10,
            resize_step: 10,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for ExhibitorConfig {
    fn default() -> Self {
        ExhibitorConfig {
            compositor: CompositorConfig::default(),
            strategist: StrategistConfig::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for InputConfig {
    fn default() -> Self {
        InputConfig {
            touchpad_scale: 1.0,
            touchpad_pressure_threshold: 50,
            mouse_scale: 1.0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for KeybindingsConfig {
    fn default() -> Self {
        KeybindingsConfig {
            common: {
                vec![BindingEntry::new(uinput_sys::KEY_ESC,
                                       modifier::LCTL | modifier::LMTA,
                                       binding_functions::Quit::new())]
            },
            normal: {
                vec![BindingEntry::new(uinput_sys::KEY_ESC,
                                       modifier::NONE,
                                       binding_functions::CleanCommand::new()),
                     BindingEntry::new(uinput_sys::KEY_H,
                                       modifier::NONE,
                                       binding_functions::Horizontalize::new()),
                     BindingEntry::new(uinput_sys::KEY_V,
                                       modifier::NONE,
                                       binding_functions::Verticalize::new()),
                     BindingEntry::new(uinput_sys::KEY_S,
                                       modifier::NONE,
                                       binding_functions::Stackize::new()),
                     BindingEntry::new(uinput_sys::KEY_I,
                                       modifier::NONE,
                                       binding_functions::SwapModeNormalToInsert::new()),
                     BindingEntry::new(uinput_sys::KEY_SPACE,
                                       modifier::NONE,
                                       binding_functions::SwapModeNormalToInsert::new()),
                     // actions
                     BindingEntry::new(uinput_sys::KEY_F,
                                       modifier::NONE,
                                       binding_functions::PutFocus::new()),
                     BindingEntry::new(uinput_sys::KEY_F,
                                       modifier::LSHF,
                                       binding_functions::PutSwap::new()),
                     BindingEntry::new(uinput_sys::KEY_J,
                                       modifier::NONE,
                                       binding_functions::PutJump::new()),
                     BindingEntry::new(uinput_sys::KEY_D,
                                       modifier::NONE,
                                       binding_functions::PutDive::new()),
                     BindingEntry::new(uinput_sys::KEY_M,
                                       modifier::NONE,
                                       binding_functions::PutMove::new()),
                     BindingEntry::new(uinput_sys::KEY_R,
                                       modifier::NONE,
                                       binding_functions::PutResize::new()),
                     // directions
                     BindingEntry::new(uinput_sys::KEY_RIGHT,
                                       modifier::NONE,
                                       binding_functions::PutEast::new()),
                     BindingEntry::new(uinput_sys::KEY_LEFT,
                                       modifier::NONE,
                                       binding_functions::PutWest::new()),
                     BindingEntry::new(uinput_sys::KEY_UP,
                                       modifier::NONE,
                                       binding_functions::PutNorth::new()),
                     BindingEntry::new(uinput_sys::KEY_DOWN,
                                       modifier::NONE,
                                       binding_functions::PutSouth::new()),
                     BindingEntry::new(uinput_sys::KEY_PAGEUP,
                                       modifier::NONE,
                                       binding_functions::PutForward::new()),
                     BindingEntry::new(uinput_sys::KEY_PAGEDOWN,
                                       modifier::NONE,
                                       binding_functions::PutBackward::new()),
                     BindingEntry::new(uinput_sys::KEY_HOME,
                                       modifier::NONE,
                                       binding_functions::PutBegin::new()),
                     BindingEntry::new(uinput_sys::KEY_END,
                                       modifier::NONE,
                                       binding_functions::PutEnd::new()),
                     // magnitude
                     BindingEntry::new(uinput_sys::KEY_1,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_2,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_3,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_4,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_5,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_6,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_7,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_8,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_9,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_10,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_MINUS,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_KPMINUS,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new()),
                     BindingEntry::new(uinput_sys::KEY_KPPLUS,
                                       modifier::NONE,
                                       binding_functions::PutMagnitude::new())]
            },
            insert: {
                vec![// insert
                     BindingEntry::new(uinput_sys::KEY_ESC,
                                       modifier::LMTA,
                                       binding_functions::SwapModeInsertToNormal::new()),
                     // focus frame
                     BindingEntry::new(uinput_sys::KEY_RIGHT,
                                       modifier::LMTA,
                                       binding_functions::FocusRight::new()),
                     BindingEntry::new(uinput_sys::KEY_DOWN,
                                       modifier::LMTA,
                                       binding_functions::FocusDown::new()),
                     BindingEntry::new(uinput_sys::KEY_LEFT,
                                       modifier::LMTA,
                                       binding_functions::FocusLeft::new()),
                     BindingEntry::new(uinput_sys::KEY_UP,
                                       modifier::LMTA,
                                       binding_functions::FocusUp::new()),
                     BindingEntry::new(uinput_sys::KEY_TAB,
                                       modifier::LMTA,
                                       binding_functions::CicleHistoryForward::new()),
                     BindingEntry::new(uinput_sys::KEY_TAB,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::CicleHistoryBackward::new()),
                     // focus workspace
                     BindingEntry::new(uinput_sys::KEY_1,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_2,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_3,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_4,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_5,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_6,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_7,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_8,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_9,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_10,
                                       modifier::LMTA,
                                       binding_functions::FocusWorkspace::new()),
                     // jumping
                     BindingEntry::new(uinput_sys::KEY_RIGHT,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpRight::new()),
                     BindingEntry::new(uinput_sys::KEY_DOWN,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpDown::new()),
                     BindingEntry::new(uinput_sys::KEY_LEFT,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpLeft::new()),
                     BindingEntry::new(uinput_sys::KEY_UP,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpUp::new()),
                     // jumping to workspace
                     BindingEntry::new(uinput_sys::KEY_1,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_2,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_3,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_4,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_5,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_6,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_7,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_8,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_9,
                                       modifier::LMTA | modifier::LSHF,
                                       binding_functions::JumpToWorkspace::new()),
                     // diving
                     BindingEntry::new(uinput_sys::KEY_RIGHT,
                                       modifier::LMTA | modifier::LALT,
                                       binding_functions::DiveRight::new()),
                     BindingEntry::new(uinput_sys::KEY_DOWN,
                                       modifier::LMTA | modifier::LALT,
                                       binding_functions::DiveDown::new()),
                     BindingEntry::new(uinput_sys::KEY_LEFT,
                                       modifier::LMTA | modifier::LALT,
                                       binding_functions::DiveLeft::new()),
                     BindingEntry::new(uinput_sys::KEY_UP,
                                       modifier::LMTA | modifier::LALT,
                                       binding_functions::DiveUp::new()),
                     // diving to workspace
                     BindingEntry::new(uinput_sys::KEY_1,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_2,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_3,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_4,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_5,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_6,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_7,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_8,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     BindingEntry::new(uinput_sys::KEY_9,
                                       modifier::LMTA | modifier::LCTL | modifier::LSHF,
                                       binding_functions::DiveToWorkspace::new()),
                     // other commands
                     BindingEntry::new(uinput_sys::KEY_HOME,
                                       modifier::LMTA,
                                       binding_functions::Exalt::new()),
                     BindingEntry::new(uinput_sys::KEY_END,
                                       modifier::LMTA,
                                       binding_functions::Ramify::new()),
                     BindingEntry::new(uinput_sys::KEY_SPACE,
                                       modifier::LMTA,
                                       binding_functions::ToggleAnchorization::new()),
                     // spawning processes
                     BindingEntry::new(uinput_sys::KEY_T,
                                       modifier::LCTL | modifier::LMTA,
                                       binding_functions::SpawnProcess::new(&["weston-terminal"]))]
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            layout: "us".to_owned(),
            variant: "".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DefaultConfig for StrategistConfig {
    fn default() -> Self {
        StrategistConfig {
            choose_target: "always_floating".to_owned(),
            choose_floating: "random".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
