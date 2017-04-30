// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Configuration structures for `perceptia`.

// -------------------------------------------------------------------------------------------------

use std;
use std::path::PathBuf;
use yaml_rust;
use serde_yaml;
use serde::ser::{Serialize, Serializer, SerializeMap};

use defs::modifier;
use input_manager::Binding;
use binding_functions;

// -------------------------------------------------------------------------------------------------

macro_rules! load_config {
    ( $config:expr; $section:expr; $( $key:ident: $type:ident ),* ) => {
        $(
            load_config!(_entry_ $config; $section; $key: $type);
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
    pub mode_name: &'static str,
    pub binding: Binding,
    pub executor: binding_functions::Executor,
}

// -------------------------------------------------------------------------------------------------

/// Manually implement `Clone` for `Binding` as there is bug in compiler:
/// https://github.com/rust-lang/rust/issues/24000
///
/// TODO: Keep checking if bug was resolved.
impl Clone for BindingEntry {
    fn clone(&self) -> Self {
        BindingEntry {
            mode_name: self.mode_name,
            binding: self.binding.clone(),
            executor: self.executor,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Debug for BindingEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.binding)
    }
}

// -------------------------------------------------------------------------------------------------

impl BindingEntry {
    /// `BindingEntry` constructor.
    pub fn new(mode_name: &'static str,
               code: i32,
               modifiers: modifier::ModifierType,
               executor: binding_functions::Executor)
               -> Self {
        BindingEntry {
            mode_name: mode_name,
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
    bindings: Vec<BindingEntry>,
}

// -------------------------------------------------------------------------------------------------

impl Config {
    /// Constructs new `Config`.
    pub fn new(aesthetics: AestheticsConfig,
               exhibitor: ExhibitorConfig,
               input: InputConfig,
               keyboard: KeyboardConfig,
               bindings: Vec<BindingEntry>)
               -> Self {
        Config {
            aesthetics: aesthetics,
            exhibitor: exhibitor,
            input: input,
            keyboard: keyboard,
            bindings: bindings,
        }
    }

    /// Override current setting with setting found in given YAML documents.
    ///
    /// TODO: Implement loading key bindings from configuration file.
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
    pub fn get_key_binding_config(&self) -> &Vec<BindingEntry> {
        &self.bindings
    }
}

// -------------------------------------------------------------------------------------------------

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_map(Some(4))?;
        seq.serialize_entry("aesthetics", &self.aesthetics)?;
        seq.serialize_entry("exhibitor", &self.exhibitor)?;
        seq.serialize_entry("input", &self.input)?;
        seq.serialize_entry("keyboard", &self.keyboard)?;
        seq.end()
    }
}

// -------------------------------------------------------------------------------------------------
