// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `qualia` is crate containing enumerations and simple tools common to all the crates of
//! `perceptia`.

extern crate dbus;
extern crate libudev; // for implementation of `From` in `errors`.
extern crate nix;

#[macro_use(timber)]
extern crate timber;
extern crate dharma;

pub mod enums;

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod errors;
pub use errors::Error;

pub mod defs;
pub use defs::{Area, Point, Position, Size, Vector};

pub mod buffer;
pub use buffer::Buffer;

#[macro_use]
pub mod log;
pub use log::level;

pub mod coordinator;
pub use coordinator::{Coordinator, ShowReason, SurfaceId};

pub mod context;
pub use context::Context;

pub mod ipc;
pub use ipc::Ipc;
