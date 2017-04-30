// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Default configuration for `perceptia`.

// -------------------------------------------------------------------------------------------------

use std::default::Default;
use uinput_sys;

use defs::{modifier, mode_name};
use binding_functions;
use config::{BindingEntry, Config};
use config::{AestheticsConfig, KeyboardConfig, InputConfig};
use config::{CompositorConfig, ExhibitorConfig, StrategistConfig};

// -------------------------------------------------------------------------------------------------

impl Default for AestheticsConfig {
    fn default() -> Self {
        AestheticsConfig {
            background_path: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for CompositorConfig {
    fn default() -> Self {
        CompositorConfig {
            move_step: 10,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for ExhibitorConfig {
    fn default() -> Self {
        ExhibitorConfig {
            compositor: CompositorConfig::default(),
            strategist: StrategistConfig::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for InputConfig {
    fn default() -> Self {
        InputConfig {
            touchpad_scale: 1.0,
            touchpad_pressure_threshold: 50,
            mouse_scale: 1.0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for StrategistConfig {
    fn default() -> Self {
        StrategistConfig {
            choose_target: "always_floating".to_owned(),
            choose_floating: "random".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            layout: "us".to_owned(),
            variant: "".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        let bindings = vec![
            // common
            BindingEntry::new(mode_name::COMMON,
                              uinput_sys::KEY_ESC,
                              modifier::LCTL | modifier::LMTA,
                              binding_functions::quit),

            // normal
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_ESC,
                              modifier::NONE,
                              binding_functions::clean_command),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_H,
                              modifier::NONE,
                              binding_functions::horizontalize),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_V,
                              modifier::NONE,
                              binding_functions::verticalize),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_S,
                              modifier::NONE,
                              binding_functions::stackize),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_I,
                              modifier::NONE,
                              binding_functions::swap_mode_normal_to_insert),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_SPACE,
                              modifier::NONE,
                              binding_functions::swap_mode_normal_to_insert),
            // actions
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_F,
                              modifier::NONE,
                              binding_functions::put_focus),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_F,
                              modifier::LSHF,
                              binding_functions::put_swap),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_J,
                              modifier::NONE,
                              binding_functions::put_jump),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_D,
                              modifier::NONE,
                              binding_functions::put_dive),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_M,
                              modifier::NONE,
                              binding_functions::put_move),
            // directions
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_RIGHT,
                              modifier::NONE,
                              binding_functions::put_east),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_LEFT,
                              modifier::NONE,
                              binding_functions::put_west),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_UP,
                              modifier::NONE,
                              binding_functions::put_north),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_DOWN,
                              modifier::NONE,
                              binding_functions::put_south),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_PAGEUP,
                              modifier::NONE,
                              binding_functions::put_forward),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_PAGEDOWN,
                              modifier::NONE,
                              binding_functions::put_backward),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_HOME,
                              modifier::NONE,
                              binding_functions::put_begin),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_END,
                              modifier::NONE,
                              binding_functions::put_end),
            // magnitude
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_1,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_2,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_3,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_4,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_5,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_6,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_7,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_8,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_9,
                              modifier::NONE,
                              binding_functions::put_magnitude),
            BindingEntry::new(mode_name::NORMAL,
                              uinput_sys::KEY_10,
                              modifier::NONE,
                              binding_functions::put_magnitude),

            // insert
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_ESC,
                              modifier::LMTA,
                              binding_functions::swap_mode_insert_to_normal),
            // focus frame
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_RIGHT,
                              modifier::LMTA,
                              binding_functions::focus_right),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_DOWN,
                              modifier::LMTA,
                              binding_functions::focus_down),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_LEFT,
                              modifier::LMTA,
                              binding_functions::focus_left),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_UP,
                              modifier::LMTA,
                              binding_functions::focus_up),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_TAB,
                              modifier::LMTA,
                              binding_functions::cicle_history_forward),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_TAB,
                              modifier::LMTA | modifier::LSHF,
                              binding_functions::cicle_history_backward),
            // focus workspace
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_1,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_2,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_3,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_4,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_5,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_6,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_7,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_8,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_9,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_10,
                              modifier::LMTA,
                              binding_functions::focus_workspace),
            // jumping
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_RIGHT,
                              modifier::LMTA | modifier::LSHF,
                              binding_functions::jump_right),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_DOWN,
                              modifier::LMTA | modifier::LSHF,
                              binding_functions::jump_down),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_LEFT,
                              modifier::LMTA | modifier::LSHF,
                              binding_functions::jump_left),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_UP,
                              modifier::LMTA | modifier::LSHF,
                              binding_functions::jump_up),
            // jumping to workspace
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_1,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_2,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_3,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_4,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_5,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_6,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_7,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_8,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_9,
                              modifier::LMTA | modifier::LCTL,
                              binding_functions::jump_to_workspace),
            // diving
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_RIGHT,
                              modifier::LMTA | modifier::LALT,
                              binding_functions::dive_right),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_DOWN,
                              modifier::LMTA | modifier::LALT,
                              binding_functions::dive_down),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_LEFT,
                              modifier::LMTA | modifier::LALT,
                              binding_functions::dive_left),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_UP,
                              modifier::LMTA | modifier::LALT,
                              binding_functions::dive_up),
            // diving to workspace
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_1,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_2,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_3,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_4,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_5,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_6,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_7,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_8,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_9,
                              modifier::LMTA | modifier::LCTL | modifier::LSHF,
                              binding_functions::dive_to_workspace),
            // other commands
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_HOME,
                              modifier::LMTA,
                              binding_functions::exalt),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_END,
                              modifier::LMTA,
                              binding_functions::ramify),
            BindingEntry::new(mode_name::INSERT,
                              uinput_sys::KEY_SPACE,
                              modifier::LALT,
                              binding_functions::toggle_anchorization),
        ];

        Config::new(AestheticsConfig::default(),
                    ExhibitorConfig::default(),
                    InputConfig::default(),
                    KeyboardConfig::default(),
                    bindings)
    }
}

// -------------------------------------------------------------------------------------------------
