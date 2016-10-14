// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Configuration for perceptia.

// -------------------------------------------------------------------------------------------------

use std::default::Default;
use std::sync::{Arc, Mutex};

// -------------------------------------------------------------------------------------------------

/// Configuration of input devices.
#[derive(Clone, Copy)]
pub struct InputConfig {
    pub touchpad_scale: f32,
    pub touchpad_pressure_treshold: i32,
    pub mouse_scale: f32,
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for global configuration.
#[derive(Clone)]
struct InnerConfig {
    /// Scale for touchpad event position values.
    /// In future will be replaced by non-linear scale per dimension.
    touchpad_scale: f32,

    /// Threshold value for touchpad pressure below which move events will be ignored.
    touchpad_pressure_treshold: i32,

    /// Scale for mouse event motion values.
    /// In future will be replaced by non-linear scale per dimension.
    mouse_scale: f32,
}

// -------------------------------------------------------------------------------------------------

/// Global configuration.
#[derive(Clone)]
pub struct Config {
    inner: Arc<Mutex<InnerConfig>>,
}

// -------------------------------------------------------------------------------------------------

impl Config {
    /// Return configuration for input devices.
    pub fn get_input_config(&self) -> InputConfig {
        let mut mine = self.inner.lock().unwrap();
        InputConfig {
            touchpad_scale: mine.touchpad_scale,
            touchpad_pressure_treshold: mine.touchpad_pressure_treshold,
            mouse_scale: mine.mouse_scale,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        Config {
            inner: Arc::new(Mutex::new(InnerConfig {
                touchpad_scale: 1.0,
                touchpad_pressure_treshold: 0,
                mouse_scale: 1.0,
            })),
        }
    }
}

// -------------------------------------------------------------------------------------------------
