// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate implements Wayland functionality.
//!
//! Currently most of it is writtent in C and links staticaly to application. It should be whole
//! rewritten un Rust in future.

extern crate libc;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;

pub mod wayland_frontend;
pub use wayland_frontend::WaylandFrontend;
