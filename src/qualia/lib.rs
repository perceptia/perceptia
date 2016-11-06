// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `qualia` is crate containing enumerations and simple tools common to all the crates of
//! `perceptia`.

#![feature(unique)]

extern crate backtrace;
extern crate dbus;
extern crate libc;
extern crate libudev; // for implementation of `From` in `errors`.
extern crate nix;
extern crate time;
extern crate xkbcommon;
extern crate uinput_sys;
#[macro_use]
extern crate bitflags;

#[macro_use(timber)]
extern crate timber;
extern crate dharma;

pub mod enums;
pub use enums::{DeviceKind, KeyState, Action, Direction};

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod errors;
pub use errors::Illusion;

#[macro_use]
pub mod macros;

pub mod defs;
pub use defs::{Area, Point, Position, OptionalPosition, SurfacePosition, Size, Vector};
pub use defs::{Button, Command, DrmBundle, modifier, Key, KeyCode, KeyValue};
pub use defs::{MemoryPoolId, MemoryViewId};

pub mod config;
pub use config::{Config, InputConfig};

pub mod timing;
pub use timing::Milliseconds;

pub mod memory;
pub use memory::{Buffer, Pixmap, MappedMemory, MemoryPool, MemoryView};

#[macro_use]
pub mod log;
pub use log::level;

pub mod functions;

pub mod env;
pub use env::Env;

pub mod keymap;
pub use keymap::{Keymap, Settings as KeymapSettings};

pub mod settings;
pub use settings::Settings;

pub mod surface;
pub use surface::{SurfaceAccess, SurfaceContext, SurfaceId, SurfaceIdType, SurfaceInfo};
pub use surface::{show_reason, surface_state};

pub mod coordinator;
pub use coordinator::Coordinator;

mod binding_functions;
pub mod input_manager;
pub use input_manager::{InputManager, KeyCatchResult};

pub mod context;
pub use context::Context;

pub mod ipc;
pub use ipc::Ipc;
