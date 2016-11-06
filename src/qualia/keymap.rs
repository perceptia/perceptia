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

/// Structure containing settings for key map.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Settings {
    pub format: u32,
    pub size: usize,
    pub fd: RawFd,
}

// -------------------------------------------------------------------------------------------------

/// This structure handles creation of file used for communicating clients current keymap using
/// xkbcommon.
#[allow(dead_code)]
pub struct Keymap {
    settings: Settings,
    file: std::fs::File,
    context: xkb::Context,
    keymap: xkb::Keymap,
    memory: *mut nix::c_void,
}

// -------------------------------------------------------------------------------------------------

impl Keymap {
    /// `Keymap` constructor.
    pub fn new(env: &env::Env) -> Result<Self, Illusion> {
        let format = xkb::KEYMAP_FORMAT_TEXT_V1;
        let rules = "evdev".to_owned();
        let model = "evdev".to_owned();
        let layout = "us".to_owned();
        let variant = "".to_owned();
        let file_name = "keymap".to_owned();

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let k = xkb::Keymap::new_from_names(&context, &rules, &model, &layout, &variant, None, 0x0);
        let keymap = if let Some(keymap) = k {
            keymap
        } else {
            return Err(Illusion::General(format!("Failed to create key map")));
        };

        // Save keymap to file
        let keymap_str = keymap.get_as_string(format);
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
                        format: format,
                        size: keymap_str.len() + 1,
                        fd: file.as_raw_fd(),
                    },
                    file: file,
                    context: context,
                    keymap: keymap,
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
