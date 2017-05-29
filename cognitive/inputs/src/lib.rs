// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tools for handling events from input devices.

extern crate xkbcommon;
extern crate cognitive_qualia as qualia;

pub mod codes;

pub mod keyboard_state;
pub use keyboard_state::{KeyboardState, KeyMods};

pub mod keymap;
pub use keymap::{Keymap};
