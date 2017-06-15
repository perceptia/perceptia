// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionlity needed to run the application in test mode without any
//! devices e.g. under other compositor or as remote desktop.

#![warn(missing_docs)]

extern crate vnc;

extern crate dharma;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;
extern crate cognitive_inputs as inputs;

pub mod remote_desktop;

pub mod virtualization;
pub use virtualization::Virtualization;
