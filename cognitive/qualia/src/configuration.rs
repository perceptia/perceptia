// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Default configuration for `cognitive` entities.

use std::path::PathBuf;

// -------------------------------------------------------------------------------------------------

/// Configuration of aesthetics.
#[derive(Clone, Debug, Serialize)]
pub struct AestheticsConfig {
    /// Path to background image.
    pub background_path: Option<PathBuf>,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of compositor.
#[derive(Clone, Debug, Serialize)]
pub struct CompositorConfig {
    /// Distance in pixels by which frames are moved by `move` command.
    pub move_step: u32,

    /// Distance in pixels by which frames are resized by `resize` command.
    pub resize_step: u32,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of exhibitor.
#[derive(Clone, Debug, Serialize)]
pub struct ExhibitorConfig {
    /// Configuration of compositor.
    pub compositor: CompositorConfig,

    /// Configuration of strategist.
    pub strategist: StrategistConfig,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of input devices.
#[derive(Clone, Debug, Serialize)]
pub struct InputConfig {
    /// Scale for touchpad event position values.
    /// In future will be replaced by non-linear scale per dimension.
    pub touchpad_scale: f32,

    /// Threshold value for touchpad pressure below which move events will be ignored.
    pub touchpad_pressure_threshold: i32,

    /// Scale for mouse event motion values.
    /// In future will be replaced by non-linear scale per dimension.
    pub mouse_scale: f32,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of keyboard.
#[derive(Clone, Debug, Serialize)]
pub struct KeyboardConfig {
    pub layout: String,
    pub variant: String,
}

// -------------------------------------------------------------------------------------------------

/// Configuration of strategist.
#[derive(Clone, Debug, Serialize)]
pub struct StrategistConfig {
    /// Strategy used to decide where and how new surface should be placed.
    pub choose_target: String,

    /// Strategy used to decide position and size of floating surface (new or deanchorized).
    pub choose_floating: String,
}

// -------------------------------------------------------------------------------------------------
