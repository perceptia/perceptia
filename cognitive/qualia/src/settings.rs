// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Various settings.

// -------------------------------------------------------------------------------------------------

use std::env;
use std::sync::{Arc, RwLock};
use std::os::unix::io::RawFd;
use std::path::PathBuf;

// -------------------------------------------------------------------------------------------------

/// Set of paths to XDG directories.
pub struct Directories {
    pub runtime: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
    pub user_config: Option<PathBuf>,
    pub system_config: Option<PathBuf>,
}

// -------------------------------------------------------------------------------------------------

/// Structure containing settings for key map.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct KeymapSettings {
    pub format: u32,
    pub size: usize,
    pub fd: RawFd,
}

// -------------------------------------------------------------------------------------------------

/// Global settings.
#[derive(Clone)]
pub struct Settings {
    keymap: Arc<RwLock<KeymapSettings>>,
    is_test_mode: bool,
}

// -------------------------------------------------------------------------------------------------

impl Settings {
    /// Constructs new `Settings`.
    pub fn new(keymap: KeymapSettings) -> Self {
        Settings {
            keymap: Arc::new(RwLock::new(keymap)),
            is_test_mode: env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok(),
        }
    }

    /// Returns key map related settings.
    pub fn get_keymap(&self) -> KeymapSettings {
        self.keymap.read().unwrap().clone()
    }

    /// Returns true if the application is in test mode.
    ///
    /// In test mode the application does not use any input nor output device. Whole user
    /// interaction with the application it provided by some remote desktop protocol.
    pub fn is_test_mode(&self) -> bool {
        self.is_test_mode
    }
}

// -------------------------------------------------------------------------------------------------
