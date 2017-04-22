// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Provides device management functionality.

#[macro_use]
extern crate nix;
extern crate libc;
extern crate libudev_sys;
extern crate libudev;
extern crate egl;
extern crate gbm_rs as libgbm;
extern crate drm as libdrm;
extern crate dbus;

extern crate dharma;
extern crate cognitive_graphics;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;
extern crate cognitive_inputs as inputs;

mod ipc;
mod device_access;
mod input_gateway;
mod drivers;
mod evdev_driver;
mod pageflip;
mod device_monitor;

pub mod udev;
pub use udev::Udev;

mod input_collector;
pub use input_collector::InputCollector;

mod output_collector;
pub use output_collector::OutputCollector;

mod virtual_terminal;
pub use virtual_terminal::VirtualTerminal;

mod graphics_manager;
pub use graphics_manager::GraphicsManager;

pub mod device_manager;
pub use device_manager::DeviceManager;
