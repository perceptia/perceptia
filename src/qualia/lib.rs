// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `qualia` is crate containing enumerations and simple tools common to all the crates of
//! `perceptia`.

extern crate dbus;
extern crate nix;

extern crate dharma;

pub mod enums;

pub mod perceptron;
pub use perceptron::Perceptron;

pub mod errors;
pub use errors::Error;


pub mod context;
pub use context::Context;

pub mod ipc;
pub use ipc::Ipc;
