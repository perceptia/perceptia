// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Configuration structures for `perceptia`.

// -------------------------------------------------------------------------------------------------

use std;
use std::ascii::AsciiExt;
use std::path::PathBuf;
use uinput_sys;
use yaml_rust;
use serde_yaml;
use serde::ser::{Serialize, Serializer, SerializeMap};

use defs::modifier;
use input_manager::Binding;
use binding_functions;

// -------------------------------------------------------------------------------------------------

macro_rules! load_config {
    ( $config:expr; $section:expr; $( $key:ident: $typ:ident ),* ) => {
        $(
            load_config!(_entry_ $config; $section; $key: $typ);
        )*
    };
    ( _entry_ $config:expr; $section:expr; $key:ident: i32 ) => {
        if let Some(value) = $section[stringify!($key)].as_i64() {
            $config.$key = value as i32;
        }
    };
    ( _entry_ $config:expr; $section:expr; $key:ident: u32 ) => {
        if let Some(value) = $section[stringify!($key)].as_i64() {
            $config.$key = value as u32;
        }
    };
    ( _entry_ $config:expr; $section:expr; $key:ident: f32 ) => {
        if let Some(value) = $section[stringify!($key)].as_f64() {
            $config.$key = value as f32;
        }
    };
    ( _entry_ $config:expr; $section:expr; $key:ident: String ) => {
        if let Some(value) = $section[stringify!($key)].as_str() {
            $config.$key = value.to_owned();
        }
    };
    ( _entry_ $config:expr; $section:expr; $key:ident: PathBuf ) => {
        if let Some(ref value) = $section[stringify!($key)].as_str() {
            $config.$key = Some(PathBuf::from(value));
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure used to keep configuration entry for one key binding.
pub struct BindingEntry {
    pub binding: Binding,
    pub executor: Box<binding_functions::Executor>,
}

/// Manually implement `Clone` for `Binding` as `executor` does not provide standard way to copy.
impl Clone for BindingEntry {
    fn clone(&self) -> Self {
        BindingEntry {
            binding: self.binding.clone(),
            executor: self.executor.duplicate(),
        }
    }
}

impl std::fmt::Debug for BindingEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.binding)
    }
}

impl BindingEntry {
    /// `BindingEntry` constructor.
    pub fn new(code: i32,
               modifiers: modifier::ModifierType,
               executor: Box<binding_functions::Executor>)
               -> Self {
        BindingEntry {
            binding: Binding::new(code, modifiers),
            executor: executor,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Configuration of aesthetics.
#[derive(Clone, Debug, Serialize)]
pub struct AestheticsConfig {
    /// Path to background image.
    pub background_path: Option<PathBuf>,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of compositor.
#[derive(Clone, Debug, Serialize)]
pub struct CompositorConfig {
    /// Distance in pixels by which frames are moved by `move` command.
    pub move_step: u32,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of exhibitor.
#[derive(Clone, Debug, Serialize)]
pub struct ExhibitorConfig {
    /// Configuration of compositor.
    pub compositor: CompositorConfig,

    /// Configuration of strategist.
    pub strategist: StrategistConfig,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of input devices.
#[derive(Clone, Debug, Serialize)]
pub struct InputConfig {
    /// Scale for touchpad event position values.
    /// In future will be replaced by non-linear scale per dimension.
    pub touchpad_scale: f32,

    /// Threshold value for touchpad pressure below which move events will be ignored.
    pub touchpad_pressure_threshold: i32,

    /// Scale for mouse event motion values.
    /// In future will be replaced by non-linear scale per dimension.
    pub mouse_scale: f32,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of keyboard.
#[derive(Clone, Debug)]
pub struct KeybindingsConfig {
    /// Bindings for `common` mode.
    pub common: Vec<BindingEntry>,

    /// Bindings for `insert` mode.
    pub insert: Vec<BindingEntry>,

    /// Bindings for `normal` mode.
    pub normal: Vec<BindingEntry>,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of keyboard.
#[derive(Clone, Debug, Serialize)]
pub struct KeyboardConfig {
    pub layout: String,
    pub variant: String,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of strategist.
#[derive(Clone, Debug, Serialize)]
pub struct StrategistConfig {
    /// Strategy used to decide where and how new surface should be placed.
    pub choose_target: String,

    /// Strategy used to decide position and size of floating surface (new or deanchorized).
    pub choose_floating: String,
}

// -------------------------------------------------------------------------------------------------

/// Global configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Config for aesthetics.
    aesthetics: AestheticsConfig,

    /// Config for exhibitor.
    exhibitor: ExhibitorConfig,

    /// Config for input devices.
    input: InputConfig,

    /// Config for keyboard.
    keyboard: KeyboardConfig,

    /// Set of key bindings.
    keybindings: KeybindingsConfig,
}

// -------------------------------------------------------------------------------------------------

impl Config {
    /// Constructs new `Config`.
    pub fn new(aesthetics: AestheticsConfig,
               exhibitor: ExhibitorConfig,
               input: InputConfig,
               keyboard: KeyboardConfig,
               keybindings: KeybindingsConfig)
               -> Self {
        Config {
            aesthetics: aesthetics,
            exhibitor: exhibitor,
            input: input,
            keyboard: keyboard,
            keybindings: keybindings,
        }
    }

    /// Override current setting with setting found in given YAML documents.
    ///
    /// TODO: Implement better warnings about errors in configuration.
    pub fn load(&mut self, yamls: &Vec<yaml_rust::Yaml>) {
        for yaml in yamls.iter() {
            load_config!{self.aesthetics; yaml["aesthetics"];
                background_path: PathBuf
            }

            load_config!{self.exhibitor.compositor; yaml["exhibitor"]["compositor"];
                move_step: u32
            }

            load_config!{self.exhibitor.strategist; yaml["exhibitor"]["strategist"];
                choose_target: String,
                choose_floating: String
            }

            load_config!{self.input; yaml["input"];
                touchpad_scale: f32,
                touchpad_pressure_threshold: i32,
                mouse_scale: f32
            }

            load_config!{self.keyboard; yaml["keyboard"];
                layout: String,
                variant: String
            }

            if let yaml_rust::yaml::Yaml::Array(ref array) = yaml["keybindings"]["insert"] {
                for e in array.iter() {
                    let code = {
                        if let Some(value) = e["key"].as_str() {
                            Self::string_to_key_code(value)
                        } else {
                            break;
                        }
                    };

                    let mods = {
                        if let yaml_rust::yaml::Yaml::Array(ref mods) = e["mods"] {
                            let mut modifiers = modifier::NONE;
                            for m in mods.iter() {
                                if let Some(value) = m.as_str() {
                                    modifiers |= Self::string_to_key_mod(value);
                                } else {
                                    break;
                                }
                            }
                            modifiers
                        } else {
                            break;
                        }
                    };

                    let executor = {
                        if let Some(value) = e["action"].as_str() {
                            Self::string_to_key_action(value)
                        } else if let yaml_rust::yaml::Yaml::Array(ref args) = e["execute"] {
                            let mut command = Vec::new();
                            for a in args.iter() {
                                if let Some(value) = a.as_str() {
                                    command.push(value.to_string());
                                } else {
                                    break;
                                }
                            }
                            Self::vec_to_key_command(command)
                        } else {
                            break;
                        }
                    };

                    self.keybindings.insert.push(BindingEntry::new(code, mods, executor));
                }
            }
        }
    }

    /// Serialize configuration to YAML.
    ///
    /// TODO: Implement serialization for key bindings.
    pub fn serialize(&self) -> String {
        serde_yaml::to_string(self).unwrap_or(String::new())
    }
}

// -------------------------------------------------------------------------------------------------

// Helper methods for parsing keybindings configuration
impl Config {
    /// Translates string to key code.
    fn string_to_key_code(value: &str) -> i32 {
        match value.to_ascii_lowercase().as_ref() {
            "1" => uinput_sys::KEY_1,
            "2" => uinput_sys::KEY_2,
            "3" => uinput_sys::KEY_3,
            "4" => uinput_sys::KEY_4,
            "5" => uinput_sys::KEY_5,
            "6" => uinput_sys::KEY_6,
            "7" => uinput_sys::KEY_7,
            "8" => uinput_sys::KEY_8,
            "9" => uinput_sys::KEY_9,
            "0" => uinput_sys::KEY_10,
            "q" => uinput_sys::KEY_Q,
            "w" => uinput_sys::KEY_W,
            "e" => uinput_sys::KEY_E,
            "r" => uinput_sys::KEY_R,
            "t" => uinput_sys::KEY_T,
            "y" => uinput_sys::KEY_Y,
            "u" => uinput_sys::KEY_U,
            "i" => uinput_sys::KEY_I,
            "o" => uinput_sys::KEY_O,
            "p" => uinput_sys::KEY_P,
            "a" => uinput_sys::KEY_A,
            "s" => uinput_sys::KEY_S,
            "d" => uinput_sys::KEY_D,
            "f" => uinput_sys::KEY_F,
            "g" => uinput_sys::KEY_G,
            "h" => uinput_sys::KEY_H,
            "j" => uinput_sys::KEY_J,
            "k" => uinput_sys::KEY_K,
            "l" => uinput_sys::KEY_L,
            "z" => uinput_sys::KEY_Z,
            "x" => uinput_sys::KEY_X,
            "c" => uinput_sys::KEY_C,
            "v" => uinput_sys::KEY_V,
            "b" => uinput_sys::KEY_B,
            "n" => uinput_sys::KEY_N,
            "m" => uinput_sys::KEY_M,
            "space" => uinput_sys::KEY_SPACE,
            _ => uinput_sys::KEY_SPACE,
        }
    }

    /// Translates string to key modifier.
    fn string_to_key_mod(value: &str) -> modifier::ModifierType {
        match value.to_ascii_lowercase().as_ref() {
            "lctl" => modifier::LCTL,
            "rctl" => modifier::RCTL,
            "lshift" => modifier::LSHF,
            "rshift" => modifier::RSHF,
            "lalt" => modifier::LALT,
            "ralt" => modifier::RALT,
            "lmeta" => modifier::LMTA,
            "rmeta" => modifier::RMTA,
            _ => modifier::RMTA,
        }
    }

    /// Constructs new executor to given action.
    fn string_to_key_action(value: &str) -> Box<binding_functions::Executor> {
        match value.to_ascii_lowercase().as_ref() {
            "nop" => binding_functions::Nop::new(),
            "clean_command" => binding_functions::CleanCommand::new(),
            "quit" => binding_functions::Quit::new(),
            "put_focus" => binding_functions::PutFocus::new(),
            "put_swap" => binding_functions::PutSwap::new(),
            "put_jump" => binding_functions::PutJump::new(),
            "put_dive" => binding_functions::PutDive::new(),
            "put_move" => binding_functions::PutMove::new(),
            "put_north" => binding_functions::PutNorth::new(),
            "put_east" => binding_functions::PutEast::new(),
            "put_south" => binding_functions::PutSouth::new(),
            "put_west" => binding_functions::PutWest::new(),
            "put_forward" => binding_functions::PutForward::new(),
            "put_backward" => binding_functions::PutBackward::new(),
            "put_begin" => binding_functions::PutBegin::new(),
            "put_end" => binding_functions::PutEnd::new(),
            "put_magnitude" => binding_functions::PutMagnitude::new(),
            "horizontalize" => binding_functions::Horizontalize::new(),
            "verticalize" => binding_functions::Verticalize::new(),
            "stackize" => binding_functions::Stackize::new(),
            "toggle_anchorization" => binding_functions::ToggleAnchorization::new(),
            "cicle_history_forward" => binding_functions::CicleHistoryForward::new(),
            "cicle_history_backward" => binding_functions::CicleHistoryBackward::new(),
            "focus_right" => binding_functions::FocusRight::new(),
            "focus_down" => binding_functions::FocusDown::new(),
            "focus_left" => binding_functions::FocusLeft::new(),
            "focus_up" => binding_functions::FocusUp::new(),
            "jump_right" => binding_functions::JumpRight::new(),
            "jump_down" => binding_functions::JumpDown::new(),
            "jump_left" => binding_functions::JumpLeft::new(),
            "jump_ip" => binding_functions::JumpUp::new(),
            "exalt" => binding_functions::Exalt::new(),
            "ramify" => binding_functions::Ramify::new(),
            "dive_right" => binding_functions::DiveRight::new(),
            "dive_down" => binding_functions::DiveDown::new(),
            "dive_left" => binding_functions::DiveLeft::new(),
            "dive_up" => binding_functions::DiveUp::new(),
            "jump_to_workspace" => binding_functions::JumpToWorkspace::new(),
            "dive_to_workspace" => binding_functions::DiveToWorkspace::new(),
            "focus_workspace" => binding_functions::FocusWorkspace::new(),
            "swap_mode_normal_to_insert" => binding_functions::SwapModeNormalToInsert::new(),
            "swap_mode_insert_to_normal" => binding_functions::SwapModeInsertToNormal::new(),
            _ => binding_functions::Nop::new(),
        }
    }

    /// Constructs new executor to given command.
    fn vec_to_key_command(command: Vec<String>) -> Box<binding_functions::Executor> {
        binding_functions::SpawnProcess::new_from_vec(command)
    }
}

// -------------------------------------------------------------------------------------------------

// Return immutable sections.
impl Config {
    /// Returns config for aesthetics.
    pub fn get_aesthetics_config(&self) -> &AestheticsConfig {
        &self.aesthetics
    }

    /// Returns config for exhibitor.
    pub fn get_exhibitor_config(&self) -> &ExhibitorConfig {
        &self.exhibitor
    }

    /// Returns configuration for input devices.
    pub fn get_input_config(&self) -> &InputConfig {
        &self.input
    }

    /// Returns configuration for keyboard.
    pub fn get_keyboard_config(&self) -> &KeyboardConfig {
        &self.keyboard
    }

    /// Returns configuration for key binding.
    pub fn get_keybindings_config(&self) -> &KeybindingsConfig {
        &self.keybindings
    }
}

// -------------------------------------------------------------------------------------------------

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("aesthetics", &self.aesthetics)?;
        map.serialize_entry("exhibitor", &self.exhibitor)?;
        map.serialize_entry("input", &self.input)?;
        map.serialize_entry("keyboard", &self.keyboard)?;
        // TODO: Serialize key bindings.
        // map.serialize_entry("keybindings", &self.keybindings)?;
        map.end()
    }
}

// -------------------------------------------------------------------------------------------------
