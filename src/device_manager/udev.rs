// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Wrapper for `libudev`. Allows to find interesting devices.

// -------------------------------------------------------------------------------------------------

use libudev;
use nix;
use std::os::unix::io::AsRawFd;
use std::path::Path;

use qualia;

use device_monitor::DeviceMonitor;

// -------------------------------------------------------------------------------------------------

const INPUT_MOUSE: &'static str = "ID_INPUT_MOUSE";
const INPUT_TOUCHPAD: &'static str = "ID_INPUT_TOUCHPAD";
const INPUT_KEYBOARD: &'static str = "ID_INPUT_KEYBOARD";

// -------------------------------------------------------------------------------------------------

/// Wrapper for `libudev`'s context.
pub struct Udev<'a> {
    context: libudev::Context,
    monitor_socket: Option<libudev::MonitorSocket<'a>>,
}

// -------------------------------------------------------------------------------------------------

impl<'a> Udev<'a> {
    /// `Udev` constructor.
    pub fn new() -> Self {
        Udev {
            context: libudev::Context::new().expect("Failed to create udev context"),
            monitor_socket: None,
        }
    }

    /// Iterate over connected input event devices and pass results to given handler.
    /// Panic if something goes wrong - this is crucial for perceptia to have input.
    pub fn iterate_event_devices<F: FnMut(&Path, &libudev::Device)>(&self, mut f: F) {
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
                                    log_info2!("Found {:?} {:?}", device_kind, devnode);
                                    f(devnode, &device);
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

    /// Start device monitoring and return instance of `Dispatcher` `EventHandler` for processing
    /// device events.
    ///
    /// Returned `DeviceMonitor` contains file descriptor from `udev` monitor. `DeviceMonitor` will
    /// handle situations when the file descriptor becomes invalid.
    pub fn start_device_monitor(&mut self) -> Result<DeviceMonitor, qualia::Error> {
        if self.monitor_socket.is_none() {
            let mut monitor = try!(libudev::Monitor::new(&self.context));
            ensure!(monitor.match_subsystem("input"));
            ensure!(monitor.match_subsystem("drm"));
            // self.monitor_socket = Some(try!(monitor.listen()));
        }

        match self.monitor_socket {
            Some(ref monitor_socket) => Ok(DeviceMonitor::new(monitor_socket.as_raw_fd())),
            None => Err(qualia::Error::General("Failed to create device monitor".to_owned())),
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
pub fn determine_device_kind(device: &libudev::Device) -> qualia::enums::DeviceKind {
    for property in device.properties() {
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
