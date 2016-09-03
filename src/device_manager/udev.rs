// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Wrapper for `libudev`. Allows to find interesting devices.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use libudev;
use nix;

use qualia;

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
    /// `Udev` constructor.
    pub fn new() -> Self {
        Udev { context: libudev::Context::new().expect("Failed to create udev context") }
    }

    /// Iterate over connected input event devices and pass results to given handler.
    pub fn iterate_event_devices<F: FnMut(&Path)>(&self, mut f: F) {
        let mut enumerator = libudev::Enumerator::new(&self.context)
            .expect("Failed to create device enumerator");
        enumerator.match_subsystem("input").expect("Failed to apply filter for device enumerator");
        for device in enumerator.scan_devices().expect("Failed to scan devices") {
            match device.devnode() {
                Some(devnode) => {
                    match device.sysname().to_os_string().into_string() {
                        Ok(sysname) => {
                            if is_event_device(devnode, &sysname) {
                                let device_kind = determine_device_kind(&device);
                                if device_kind != qualia::enums::DeviceKind::Unknown {
                                    println!("{:?}", device.devpath());
                                    f(devnode);
                                }
                            }
                        }
                        Err(_) => (),
                    }
                }
                None => (), // Ignore devices without devnode
            };
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Checks if given device exists is event device.
fn is_event_device(devnode: &Path, sysname: &String) -> bool {
    match nix::sys::stat::stat(devnode) {
        Ok(_) => sysname.starts_with("event"),
        Err(_) => false,
    }
}

// -------------------------------------------------------------------------------------------------

/// Reads devices properties and determines device kind basing on them.
fn determine_device_kind(device: &libudev::Device) -> qualia::enums::DeviceKind {
    for property in device.properties() {
        println!("- {:?}", property.name());
        if property.name() == INPUT_MOUSE {
            return qualia::enums::DeviceKind::Mouse;
        } else if property.name() == INPUT_TOUCHPAD {
            return qualia::enums::DeviceKind::Touchpad;
        } else if property.name() == INPUT_KEYBOARD {
            return qualia::enums::DeviceKind::Keyboard;
        }
    }
    qualia::enums::DeviceKind::Unknown
}

// -------------------------------------------------------------------------------------------------
