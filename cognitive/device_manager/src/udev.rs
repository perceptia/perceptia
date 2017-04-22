// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Wrapper for `libudev`. Allows to find interesting devices.

// -------------------------------------------------------------------------------------------------

use std::ffi::OsStr;
use std::path::Path;

use libudev;
use nix;

use qualia::DeviceKind;

// -------------------------------------------------------------------------------------------------

const INPUT_MOUSE: &'static str = "ID_INPUT_MOUSE";
const INPUT_TOUCHPAD: &'static str = "ID_INPUT_TOUCHPAD";
const INPUT_KEYBOARD: &'static str = "ID_INPUT_KEYBOARD";

// -------------------------------------------------------------------------------------------------

/// Wrapper for `libudev`'s context.
pub struct Udev {
    context: libudev::Context,
}

// -------------------------------------------------------------------------------------------------

impl Udev {
    /// Constructs new "Udev".
    pub fn new() -> Self {
        Udev { context: libudev::Context::new().expect("Failed to create udev context") }
    }

    /// Iterates over connected input event devices and pass results to given handler.
    /// Panics if something goes wrong.
    pub fn iterate_input_devices<F>(&self, mut f: F)
        where F: FnMut(&Path, DeviceKind, &libudev::Device)
    {
        let mut enumerator =
            libudev::Enumerator::new(&self.context).expect("Failed to create device enumerator");

        enumerator.match_subsystem("input").expect("Failed to apply filter for device enumerator");
        for device in enumerator.scan_devices().expect("Failed to scan devices") {
            let device_kind = determine_device_kind(&device);
            if device_kind != DeviceKind::Unknown && is_input_device(device.sysname()) {
                if let Some(devnode) = device.devnode() {
                    if exists_in_filesystem(&devnode) {
                        f(devnode, device_kind, &device);
                    }
                }
            }
        }
    }

    /// Iterates over connected output DRM devices and pass results to given handler.
    /// Panics if something goes wrong.
    pub fn iterate_output_devices<F: FnMut(&Path, &libudev::Device)>(&self, mut f: F) {
        let mut enumerator =
            libudev::Enumerator::new(&self.context).expect("Failed to create device enumerator");

        enumerator.match_subsystem("drm").expect("Failed to apply filter for device enumerator");
        for device in enumerator.scan_devices().expect("Failed to scan devices") {
            if is_output_device(device.sysname()) {
                if let Some(devnode) = device.devnode() {
                    if exists_in_filesystem(&devnode) {
                        log_info1!("Found output device: {:?}", devnode);
                        f(devnode, &device);
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Checks if given device exists is event device.
pub fn exists_in_filesystem(devnode: &Path) -> bool {
    nix::sys::stat::stat(devnode).is_ok()
}

// -------------------------------------------------------------------------------------------------

/// Checks if given sysname is for input device.
pub fn is_input_device(sysname: &OsStr) -> bool {
    match sysname.to_os_string().into_string() {
        Ok(sysname) => sysname.starts_with("event"),
        Err(_) => false,
    }
}

// -------------------------------------------------------------------------------------------------

/// Checks if given sysname is for output device.
pub fn is_output_device(sysname: &OsStr) -> bool {
    match sysname.to_os_string().into_string() {
        Ok(sysname) => sysname.starts_with("card"),
        Err(_) => false,
    }
}

// -------------------------------------------------------------------------------------------------

/// Reads devices properties and determines device kind basing on them.
pub fn determine_device_kind(device: &libudev::Device) -> DeviceKind {
    for property in device.properties() {
        if property.name() == INPUT_MOUSE {
            return DeviceKind::Mouse;
        } else if property.name() == INPUT_TOUCHPAD {
            return DeviceKind::Touchpad;
        } else if property.name() == INPUT_KEYBOARD {
            return DeviceKind::Keyboard;
        }
    }
    DeviceKind::Unknown
}

// -------------------------------------------------------------------------------------------------

