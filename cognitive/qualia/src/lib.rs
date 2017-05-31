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
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;

#[macro_use(timber)]
extern crate timber;
extern crate dharma;
extern crate cognitive_graphics as graphics;

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
pub use defs::{Command, OutputInfo, DrmBundle};
pub use defs::{DmabufId, EglImageId, MemoryPoolId, MemoryViewId, SignalId};

pub mod input;
pub use input::{Axis, Button, Binding, Key, KeyCatchResult, KeyCode, KeyValue, modifier};
pub use input::{InputForwarding, InputHandling};

pub mod image;
pub use image::{Image, Pixmap, PixelFormat};

pub mod memory;
pub use memory::{Buffer, MappedMemory, MemoryPool, MemoryView};

pub mod configuration;
pub use configuration::{AestheticsConfig, CompositorConfig, ExhibitorConfig};
pub use configuration::{KeyboardConfig, InputConfig, StrategistConfig};

pub mod surface;
pub use surface::{SurfaceContext, SurfaceId, SurfaceIdType, SurfaceInfo, DataSource};
pub use surface::{SurfaceManagement, SurfaceControl, SurfaceViewer};
pub use surface::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
pub use surface::{show_reason, surface_state};

pub mod transfer;
pub use transfer::Transfer;

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod traits;
pub use traits::{AppearanceManagement, DataTransferring, EventHandling, StatePublishing};
pub use traits::{Screenshooting, MemoryManagement, HwGraphics};
pub use traits::GraphicsManagement;
pub use traits::{AestheticsCoordinationTrait, ExhibitorCoordinationTrait};
pub use traits::FrontendsCoordinationTrait;

#[macro_use]
pub mod log;
pub use log::level;

pub mod env;
pub use env::{Env, LogDestination};

pub mod settings;
pub use settings::{Settings, Directories, KeymapSettings};

pub mod ipc;
pub use ipc::Ipc;

#[cfg(feature = "testing")]
pub mod coordinator_mock;
