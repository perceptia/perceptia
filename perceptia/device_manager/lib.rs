// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides device management functionality for Perceptia.

#[macro_use]
extern crate nix;
extern crate libudev;
extern crate uinput_sys;
extern crate libc;
extern crate egl;
extern crate gbm_rs as libgbm;
extern crate drm as libdrm;

extern crate dharma;
extern crate cognitive_graphics;
extern crate gears;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;

// TODO: Get rid of dependency from `coordination` and `dharma` in `device_manager`. See
// description of `coordination` crate. Provide unit tests.
extern crate coordination;

mod device_access;
mod input_gateway;
mod evdev;
mod drivers;
mod pageflip;
mod output_collector;
mod device_monitor;
mod virtual_terminal;

pub mod udev;

mod graphics_manager;
pub use graphics_manager::GraphicsManager;

pub mod device_manager;
pub use device_manager::DeviceManager;
