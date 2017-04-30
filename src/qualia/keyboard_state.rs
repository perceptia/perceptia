// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality allowing to keep track of state of keyboard.

// -------------------------------------------------------------------------------------------------

use xkbcommon::xkb;

use config::KeyboardConfig;
use defs::{KeyCode, KeyValue};
use errors::Illusion;
use keymap::XkbKeymap;

// -------------------------------------------------------------------------------------------------

/// This struct represents state of keyboard modifiers (shift, ctrl, etc...).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyMods {
    pub depressed: u32,
    pub latched: u32,
    pub locked: u32,
    pub effective: u32,
}

// -------------------------------------------------------------------------------------------------

impl KeyMods {
    /// Constructs default `KeyMods`.
    pub fn default() -> Self {
        KeyMods {
            depressed: 0,
            latched: 0,
            locked: 0,
            effective: 0,
        }
    }

    /// Constructs `KeyMods` from given modifiers.
    pub fn new(depressed: u32, latched: u32, locked: u32, effective: u32) -> Self {
        KeyMods {
            depressed: depressed,
            latched: latched,
            locked: locked,
            effective: effective,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents state of keyboard.
pub struct KeyboardState {
    xkb_state: xkb::State,
    mods: KeyMods,
}

// -------------------------------------------------------------------------------------------------

impl KeyboardState {
    /// Constructs new `KeyboardState`.
    pub fn new(config: &KeyboardConfig) -> Result<Self, Illusion> {
        let xkb_keymap = if let Some(xkb_keymap) = XkbKeymap::new(config) {
            xkb_keymap
        } else {
            return Err(Illusion::General(format!("Failed to create key map")));
        };

        Ok(KeyboardState {
                xkb_state: xkb::State::new(&xkb_keymap.keymap),
                mods: KeyMods::default(),
            })
    }

    /// Updates state with given key. Returns `true` when modifiers changed, false otherwise.
    pub fn update(&mut self, code: KeyCode, value: KeyValue) -> bool {
        let direction = if value == 0 {
            xkb::KeyDirection::Up
        } else {
            xkb::KeyDirection::Down
        };

        // Offset the key code by 8, as the evdev XKB rules reflect X's
        // broken key code system, which starts at 8.
        self.xkb_state.update_key(code as u32 + 8, direction);
        let mods = KeyMods::new(self.xkb_state.serialize_mods(xkb::STATE_MODS_DEPRESSED),
                                self.xkb_state.serialize_mods(xkb::STATE_MODS_LATCHED),
                                self.xkb_state.serialize_mods(xkb::STATE_MODS_LOCKED),
                                self.xkb_state.serialize_mods(xkb::STATE_MODS_EFFECTIVE));

        if mods != self.mods {
            self.mods = mods;
            true
        } else {
            false
        }
    }

    /// Returns state of modifiers.
    pub fn get_mods(&self) -> KeyMods {
        self.mods
    }
}

// -------------------------------------------------------------------------------------------------
