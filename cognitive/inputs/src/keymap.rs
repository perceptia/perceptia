// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to key maps.

// -------------------------------------------------------------------------------------------------

use std;
use std::os::unix::io::AsRawFd;
use std::io::Write;
use xkbcommon::xkb;

use qualia::{Illusion, KeyboardConfig, KeymapSettings, env};

// -------------------------------------------------------------------------------------------------

const DEFAULT_FORMAT: u32 = xkb::KEYMAP_FORMAT_TEXT_V1;

// -------------------------------------------------------------------------------------------------

/// Wrapper for `xkb` context and keymap.
pub struct XkbKeymap {
    pub context: xkb::Context,
    pub keymap: xkb::Keymap,
}

// -------------------------------------------------------------------------------------------------

impl XkbKeymap {
    /// Constructs new `XkbKeymap`.
    pub fn new(config: &KeyboardConfig) -> Option<Self> {
        let rules = "evdev".to_owned();
        let model = "evdev".to_owned();
        let layout = &config.layout;
        let variant = &config.variant;

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let k = xkb::Keymap::new_from_names(&context, &rules, &model, layout, variant, None, 0x0);
        if let Some(keymap) = k {
            Some(XkbKeymap {
                     context: context,
                     keymap: keymap,
                 })
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure handles creation of file used for communicating clients current keymap using
/// xkbcommon.
pub struct Keymap {
    settings: KeymapSettings,

    /// Keymap file. It is not referenced here but must be kept open because we will pass file
    /// descriptor to clients.
    _file: std::fs::File,
}

// -------------------------------------------------------------------------------------------------

impl Keymap {
    /// `Keymap` constructor.
    pub fn new(env: &env::Env, config: &KeyboardConfig) -> Result<Self, Illusion> {
        let xkb_keymap = if let Some(xkb_keymap) = XkbKeymap::new(config) {
            xkb_keymap
        } else {
            return Err(Illusion::General(format!("Failed to create key map")));
        };

        // Save keymap to file
        let file_name = "keymap".to_owned();
        let keymap_str = xkb_keymap.keymap.get_as_string(DEFAULT_FORMAT);
        let mut file = env.open_file(file_name, env::Directory::Runtime)?;
        file.write_all(keymap_str.as_bytes())?;
        file.write_all("\0".as_bytes())?;

        Ok(Keymap {
               settings: KeymapSettings {
                   format: DEFAULT_FORMAT,
                   size: keymap_str.len() + 1,
                   fd: file.as_raw_fd(),
               },
               _file: file,
           })
    }

    /// Return key map settings.
    pub fn get_settings(&self) -> KeymapSettings {
        self.settings.clone()
    }
}

// -------------------------------------------------------------------------------------------------
