// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

/// `DeviceMonitor` implements `dharma::Dispatcher`'s `EventHandler`. It is used to process
/// notifications from `udev` about adding and removing devices..

// -------------------------------------------------------------------------------------------------

use std;
use std::ffi::{CString, OsStr};
use std::os::unix::io;
use std::os::unix::ffi::OsStrExt;

use libc;
use libudev_sys;

use dharma::{EventHandler, EventKind};
use qualia::{Illusion, StatePublishing};

use udev;

// -------------------------------------------------------------------------------------------------

/// `udev` device event handler.
///
/// `libudev-rs` unfortunately does not support sending monitor between threads so raw
/// `libudev-sys` pointers are used.
pub struct DeviceMonitor<P>
    where P: StatePublishing + Send
{
    context: *mut libudev_sys::udev,
    monitor: *mut libudev_sys::udev_monitor,
    state_publisher: P,
}

// -------------------------------------------------------------------------------------------------

/// `DeviceMonitor` contains only sendables and pointers to `libudev` objects and does not expose
/// to user anything constructed by `libudev` so it is safe for send it to other thread.
unsafe impl<P> Send for DeviceMonitor<P>
    where P: StatePublishing + Send
{
}

// -------------------------------------------------------------------------------------------------

impl<P> DeviceMonitor<P>
    where P: StatePublishing + Send
{
    /// Constructs new `DeviceMonitor`.
    ///
    /// Starts device monitoring and returns instance of `Dispatcher` `EventHandler` for processing
    /// device events.
    pub fn new(state_publisher: P) -> Result<Self, Illusion> {
        unsafe {
            let netlink_name = CString::new("udev").unwrap();
            let input_subsystem_name = CString::new("input").unwrap();
            let drm_subsystem_name = CString::new("drm").unwrap();

            let context = libudev_sys::udev_new();
            let monitor = libudev_sys::udev_monitor_new_from_netlink(context,
                                                                     netlink_name.as_ptr());
            libudev_sys::udev_monitor_filter_add_match_subsystem_devtype
                                         (monitor, input_subsystem_name.as_ptr(), std::ptr::null());
            libudev_sys::udev_monitor_filter_add_match_subsystem_devtype
                                           (monitor, drm_subsystem_name.as_ptr(), std::ptr::null());
            libudev_sys::udev_monitor_enable_receiving(monitor);

            Ok(DeviceMonitor {
                   context: context,
                   monitor: monitor,
                   state_publisher: state_publisher,
               })
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<P> Drop for DeviceMonitor<P>
    where P: StatePublishing + Send
{
    fn drop(&mut self) {
        unsafe {
            libudev_sys::udev_monitor_unref(self.monitor);
            libudev_sys::udev_unref(self.context);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This code executes in main dispatchers thread.
impl<P> EventHandler for DeviceMonitor<P>
    where P: StatePublishing + Send
{
    fn get_fd(&self) -> io::RawFd {
        unsafe { libudev_sys::udev_monitor_get_fd(self.monitor) }
    }

    fn process_event(&mut self, _: EventKind) {
        let device = unsafe { libudev_sys::udev_monitor_receive_device(self.monitor) };
        if !device.is_null() {
            let sysname = unsafe {
                let ptr = libudev_sys::udev_device_get_sysname(device);
                let slice = std::slice::from_raw_parts(ptr as *const u8,
                                                       libc::strlen(ptr) as usize);
                OsStr::from_bytes(slice)
            };

            if udev::is_input_device(sysname) {
                self.state_publisher.input_devices_changed();
            } else if udev::is_output_device(sysname) {
                self.state_publisher.output_devices_changed();
            }
        } else {
            log_warn2!("Received empty device monitor event");
        };
    }
}

// -------------------------------------------------------------------------------------------------
