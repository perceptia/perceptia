// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Bucket for general tools.

extern crate libc;
extern crate uinput_sys;

extern crate yaml_rust;
extern crate serde;
extern crate serde_yaml;

extern crate dharma;
#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;

mod binding_functions;
mod config_defaults;

pub mod functions;

pub mod config;
pub use config::{Config, KeybindingsConfig};

pub mod input_manager;
pub use input_manager::{InputForwarder, InputManager};
