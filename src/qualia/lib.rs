// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `qualia` is crate containing enumerations, macros and definitions common to all the crates of
//! `perceptia` and traits used to decouple `perceptia`'s creates one from another (mainly for unit
//! tests).
//!
//! Unfortunately it is also home for small tools not important enough to have their own crate.
//! TODO: Identify and move to separate crate tools not fitting to purpose of this crate.

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

pub mod errors;
pub use errors::Illusion;

#[macro_use]
pub mod macros;

pub mod timing;
pub use timing::Milliseconds;

pub mod defs;
pub use defs::{Area, Point, Position, OptionalPosition, Size, Slide, Vector};
pub use defs::{Axis, Button, Command, DrmBundle, modifier, Key, KeyCode, KeyValue, OutputInfo};
pub use defs::{MemoryPoolId, MemoryViewId, SignalId};

pub mod config;
pub use config::{Config, AestheticsConfig, InputConfig};

pub mod memory;
pub use memory::{Buffer, Pixmap, MappedMemory, MemoryPool, MemoryView};

pub mod surface;
pub use surface::{SurfaceContext, SurfaceId, SurfaceIdType, SurfaceInfo};
pub use surface::{SurfaceManagement, SurfaceControl, SurfaceViewer};
pub use surface::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
pub use surface::{show_reason, surface_state};

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod traits;
pub use traits::{AppearanceManagement, Emiter, Screenshooting, MemoryManagement};
pub use traits::{AestheticsCoordinationTrait, ExhibitorCoordinationTrait};

#[macro_use]
pub mod log;
pub use log::level;

pub mod functions;

pub mod env;
pub use env::Env;

pub mod keyboard_state;
pub use keyboard_state::{KeyboardState, KeyMods};

pub mod keymap;
pub use keymap::{Keymap, Settings as KeymapSettings, XkbKeymap};

pub mod settings;
pub use settings::Settings;

mod binding_functions;
pub mod input_manager;
pub use input_manager::{InputManager, KeyCatchResult};

pub mod ipc;
pub use ipc::Ipc;
