// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! TODO

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;

use dharma::EventHandler;

// -------------------------------------------------------------------------------------------------

/// TODO
pub struct DeviceMonitor {
    monitor_fd: io::RawFd,
}

// -------------------------------------------------------------------------------------------------

impl DeviceMonitor {
    /// TODO
    pub fn new(fd: io::RawFd) -> Self {
        DeviceMonitor {
            monitor_fd: fd,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This code executes in main dispatchers thread.
impl EventHandler for DeviceMonitor {
    fn get_fd(&self) -> io::RawFd {
        self.monitor_fd
    }

    fn process_event(&mut self) {
    }
}

// -------------------------------------------------------------------------------------------------
