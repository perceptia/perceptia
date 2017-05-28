// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Various settings.

// -------------------------------------------------------------------------------------------------

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
}

// -------------------------------------------------------------------------------------------------

impl Settings {
    /// `Settings` constructor.
    pub fn new(keymap: KeymapSettings) -> Self {
        Settings { keymap: Arc::new(RwLock::new(keymap)) }
    }

    /// Get key map related settings.
    pub fn get_keymap(&self) -> KeymapSettings {
        self.keymap.read().unwrap().clone()
    }
}

// -------------------------------------------------------------------------------------------------
