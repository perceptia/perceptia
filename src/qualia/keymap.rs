// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to key maps.

// -------------------------------------------------------------------------------------------------

use std;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io::Write;
use xkbcommon::xkb;
use nix;
use nix::sys::mman;

use errors::Illusion;
use env;

// -------------------------------------------------------------------------------------------------

const DEFAULT_FORMAT: u32 = xkb::KEYMAP_FORMAT_TEXT_V1;

// -------------------------------------------------------------------------------------------------

/// Structure containing settings for key map.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Settings {
    pub format: u32,
    pub size: usize,
    pub fd: RawFd,
}

// -------------------------------------------------------------------------------------------------

/// Wrapper for `xkb` context and keymap.
pub struct XkbKeymap {
    pub context: xkb::Context,
    pub keymap: xkb::Keymap,
}

// -------------------------------------------------------------------------------------------------

impl XkbKeymap {
    /// Constructs new `XkbKeymap`.
    ///
    /// TODO: Keyboard layout should be customizable.
    pub fn default() -> Option<Self> {
        let rules = "evdev".to_owned();
        let model = "evdev".to_owned();
        let layout = "us".to_owned();
        let variant = "".to_owned();

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let k = xkb::Keymap::new_from_names(&context, &rules, &model, &layout, &variant, None, 0x0);
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
#[allow(dead_code)]
pub struct Keymap {
    settings: Settings,
    file: std::fs::File,
    xkb_keymap: XkbKeymap,
    memory: *mut nix::c_void,
}

// -------------------------------------------------------------------------------------------------

impl Keymap {
    /// `Keymap` constructor.
    pub fn new(env: &env::Env) -> Result<Self, Illusion> {
        let k = XkbKeymap::default();
        let xkb_keymap = if let Some(xkb_keymap) = k {
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

        // TODO: Unmap the memory.
        match mman::mmap(std::ptr::null_mut(),
                         keymap_str.len() + 1,
                         mman::PROT_READ | mman::PROT_WRITE,
                         mman::MAP_SHARED,
                         file.as_raw_fd(),
                         0) {
            Ok(memory) => {
                Ok(Keymap {
                    settings: Settings {
                        format: DEFAULT_FORMAT,
                        size: keymap_str.len() + 1,
                        fd: file.as_raw_fd(),
                    },
                    file: file,
                    xkb_keymap: xkb_keymap,
                    memory: memory,
                })
            }
            Err(_) => Err(Illusion::General(format!("mmap error"))),
        }
    }

    /// Return key map settings.
    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }
}

// -------------------------------------------------------------------------------------------------
