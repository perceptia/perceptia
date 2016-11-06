// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

/// `DeviceMonitor` implements `dharma::Dispatcher`'s `EventHandler`. It is used to process
/// notifications from `udev` about adding and removing devices..

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;

use dharma::{EventHandler, EventKind};

// -------------------------------------------------------------------------------------------------

/// `udev` device event handled.
pub struct DeviceMonitor {
    monitor_fd: io::RawFd,
}

// -------------------------------------------------------------------------------------------------

impl DeviceMonitor {
    /// `DeviceMonitor` constructor.
    pub fn new(fd: io::RawFd) -> Self {
        DeviceMonitor { monitor_fd: fd }
    }
}

// -------------------------------------------------------------------------------------------------

/// This code executes in main dispatchers thread.
impl EventHandler for DeviceMonitor {
    fn get_fd(&self) -> io::RawFd {
        self.monitor_fd
    }

    fn process_event(&mut self, _: EventKind) {
        // FIXME: Implement handling of device adding and removing.
    }
}

// -------------------------------------------------------------------------------------------------
