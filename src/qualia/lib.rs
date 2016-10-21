// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `qualia` is crate containing enumerations and simple tools common to all the crates of
//! `perceptia`.

extern crate dbus;
extern crate libc;
extern crate libudev; // for implementation of `From` in `errors`.
extern crate nix;
extern crate time;
extern crate xkbcommon;

#[macro_use(timber)]
extern crate timber;
extern crate dharma;

pub mod enums;
pub use enums::{DeviceKind, KeyState};

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod errors;
pub use errors::Illusion;

pub mod defs;
pub use defs::{Area, Point, Button, Key, Position, OptionalPosition, SurfacePosition, Size, Vector, DrmBundle};

pub mod config;
pub use config::{Config, InputConfig};

pub mod buffer;
pub use buffer::Buffer;

#[macro_use]
pub mod log;
pub use log::level;

pub mod env;
pub use env::Env;

pub mod keymap;
pub use keymap::{Keymap, Settings as KeymapSettings};

pub mod settings;
pub use settings::Settings;

pub mod surface;
pub use surface::{SurfaceAccess, SurfaceContext, SurfaceId, SurfaceIdType, SurfaceInfo, show_reason, surface_state};

pub mod coordinator;
pub use coordinator::Coordinator;

pub mod context;
pub use context::Context;

pub mod ipc;
pub use ipc::Ipc;
