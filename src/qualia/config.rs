// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Configuration for perceptia.

// -------------------------------------------------------------------------------------------------

use std::default::Default;
use std::sync::{Arc, Mutex};
use uinput_sys;

use defs::{modifier, mode_name};
use input_manager::Binding;
use binding_functions;

// -------------------------------------------------------------------------------------------------

/// Helper structure used to keep configuration entry for one key binding.
pub struct BindingEntry {
    pub mode_name: &'static str,
    pub binding: Binding,
    pub executor: binding_functions::Executor,
}

// -------------------------------------------------------------------------------------------------

/// Manually implement `Clone` for `Binding` as there is bug in compiler:
/// https://github.com/rust-lang/rust/issues/24000
///
/// TODO: Keep checking if bug was resolved.
impl Clone for BindingEntry {
    fn clone(&self) -> Self {
        BindingEntry {
            mode_name: self.mode_name,
            binding: self.binding.clone(),
            executor: self.executor,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl BindingEntry {
    /// `BindingEntry` constructor.
    pub fn new(mode_name: &'static str,
               code: i32,
               modifiers: modifier::ModifierType,
               executor: binding_functions::Executor)
               -> Self {
        BindingEntry {
            mode_name: mode_name,
            binding: Binding::new(code, modifiers),
            executor: executor,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Configuration of input devices.
#[derive(Clone, Copy)]
pub struct InputConfig {
    pub touchpad_scale: f32,
    pub touchpad_pressure_threshold: i32,
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
    touchpad_pressure_threshold: i32,

    /// Scale for mouse event motion values.
    /// In future will be replaced by non-linear scale per dimension.
    mouse_scale: f32,

    /// Set of key bindings.
    bindings: Vec<BindingEntry>,
}

// -------------------------------------------------------------------------------------------------

/// Global configuration.
#[derive(Clone)]
pub struct Config {
    inner: Arc<Mutex<InnerConfig>>,
}

// -------------------------------------------------------------------------------------------------

impl Config {
    /// Returns configuration for input devices.
    pub fn get_input_config(&self) -> InputConfig {
        let mine = self.inner.lock().unwrap();
        InputConfig {
            touchpad_scale: mine.touchpad_scale,
            touchpad_pressure_threshold: mine.touchpad_pressure_threshold,
            mouse_scale: mine.mouse_scale,
        }
    }

    /// Returns configuration for key binding.
    pub fn get_key_binding_config(&self) -> Vec<BindingEntry> {
        let mine = self.inner.lock().unwrap();
        mine.bindings.clone()
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        Config {
            inner: Arc::new(Mutex::new(InnerConfig {
                touchpad_scale: 1.0,
                touchpad_pressure_threshold: 0,
                mouse_scale: 1.0,
                bindings: vec![
                    // common
                    BindingEntry::new(mode_name::COMMON,
                                      uinput_sys::KEY_ESC,
                                      modifier::LCTL | modifier::LMTA,
                                      binding_functions::quit),
                    // normal
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_ESC,
                                      modifier::NONE,
                                      binding_functions::clean_command),
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_H,
                                      modifier::NONE,
                                      binding_functions::horizontalize),
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_V,
                                      modifier::NONE,
                                      binding_functions::verticalize),
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_S,
                                      modifier::NONE,
                                      binding_functions::stackize),
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_I,
                                      modifier::NONE,
                                      binding_functions::swap_mode_normal_to_insert),
                    BindingEntry::new(mode_name::NORMAL,
                                      uinput_sys::KEY_SPACE,
                                      modifier::NONE,
                                      binding_functions::swap_mode_normal_to_insert),
                    // insert
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_ESC,
                                      modifier::NONE,
                                      binding_functions::swap_mode_insert_to_normal),
                    // focus
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_RIGHT,
                                      modifier::LMTA,
                                      binding_functions::focus_right),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_DOWN,
                                      modifier::LMTA,
                                      binding_functions::focus_down),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_LEFT,
                                      modifier::LMTA,
                                      binding_functions::focus_left),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_UP,
                                      modifier::LMTA,
                                      binding_functions::focus_up),
                    // jumping
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_RIGHT,
                                      modifier::LMTA | modifier::LSHF,
                                      binding_functions::jump_right),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_DOWN,
                                      modifier::LMTA | modifier::LSHF,
                                      binding_functions::jump_down),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_LEFT,
                                      modifier::LMTA | modifier::LSHF,
                                      binding_functions::jump_left),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_UP,
                                      modifier::LMTA | modifier::LSHF,
                                      binding_functions::jump_up),
                    // diving
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_RIGHT,
                                      modifier::LMTA | modifier::LALT,
                                      binding_functions::dive_right),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_DOWN,
                                      modifier::LMTA | modifier::LALT,
                                      binding_functions::dive_down),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_LEFT,
                                      modifier::LMTA | modifier::LALT,
                                      binding_functions::dive_left),
                    BindingEntry::new(mode_name::INSERT,
                                      uinput_sys::KEY_UP,
                                      modifier::LMTA | modifier::LALT,
                                      binding_functions::dive_up),
                ],
            })),
        }
    }
}

// -------------------------------------------------------------------------------------------------
