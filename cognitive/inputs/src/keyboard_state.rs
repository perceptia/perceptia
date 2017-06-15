// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality allowing to keep track of state of keyboard.

// -------------------------------------------------------------------------------------------------

use xkbcommon::xkb;

use qualia::{modifier, CatchResult, Illusion, KeyState, KeyboardConfig, InputCode, InputValue};

use keymap::XkbKeymap;
use codes;

// -------------------------------------------------------------------------------------------------

/// This structure holds current state of modifier keys.
#[derive(Debug)]
pub struct ModState {
    /// Current state of modifiers.
    modifiers: modifier::ModifierType,

    /// List of key codes used as modifiers.
    modifier_keys: Vec<(InputCode, modifier::ModifierType)>,
}

// -------------------------------------------------------------------------------------------------

impl ModState {
    /// Constructs new `ModState`.
    pub fn new() -> Self {
        ModState {
            modifiers: modifier::NONE,
            modifier_keys: vec![(codes::KEY_LEFTCTRL as InputCode, modifier::LCTL),
                                (codes::KEY_RIGHTCTRL as InputCode, modifier::RCTL),
                                (codes::KEY_LEFTSHIFT as InputCode, modifier::LSHF),
                                (codes::KEY_RIGHTSHIFT as InputCode, modifier::RSHF),
                                (codes::KEY_LEFTALT as InputCode, modifier::LALT),
                                (codes::KEY_RIGHTALT as InputCode, modifier::RALT),
                                (codes::KEY_LEFTMETA as InputCode, modifier::LMTA),
                                (codes::KEY_RIGHTMETA as InputCode, modifier::RMTA)],
        }
    }

    /// Returns current state of modifier keys.
    #[inline]
    pub fn get(&self) -> modifier::ModifierType {
        self.modifiers
    }

    /// Updating state of the modifiers.
    pub fn update(&mut self, code: InputCode, value: InputValue) -> CatchResult {
        let mut result = CatchResult::Passed;
        for &(modifier_code, modifier_flag) in self.modifier_keys.iter() {
            if code == modifier_code {
                if value == KeyState::Pressed as InputValue {
                    if (self.modifiers & modifier_flag) != 0x0 {
                        result = CatchResult::Caught;
                    } else {
                        self.modifiers |= modifier_flag;
                    }
                } else {
                    self.modifiers &= !modifier_flag;
                }
                break;
            }
        }
        result
    }
}

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
    pub fn update(&mut self, code: InputCode, value: InputValue) -> bool {
        let direction = {
            if value == 0 {
                xkb::KeyDirection::Up
            } else {
                xkb::KeyDirection::Down
            }
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
