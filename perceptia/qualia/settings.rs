// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Settings for perceptia.

// -------------------------------------------------------------------------------------------------

use std::sync::{Arc, Mutex};

use keymap;

// -------------------------------------------------------------------------------------------------

/// Helper structure for global settings.
#[derive(Clone)]
struct InnerSettings {
    pub keymap: keymap::Settings,
}

// -------------------------------------------------------------------------------------------------

/// Global settings.
#[derive(Clone)]
pub struct Settings {
    inner: Arc<Mutex<InnerSettings>>,
}

// -------------------------------------------------------------------------------------------------

impl Settings {
    /// `Settings` constructor.
    pub fn new(keymap: keymap::Settings) -> Self {
        Settings { inner: Arc::new(Mutex::new(InnerSettings { keymap: keymap })) }
    }

    /// Get key map related settings.
    pub fn get_keymap(&self) -> keymap::Settings {
        let mine = self.inner.lock().unwrap();
        mine.keymap.clone()
    }
}

// -------------------------------------------------------------------------------------------------
