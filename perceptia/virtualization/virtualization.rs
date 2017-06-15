// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Virtualization manager.

use std::os::unix::io::RawFd;

use dharma;

use qualia::{InputForwarding, InputHandling, Perceptron};

use remote_desktop;

// -------------------------------------------------------------------------------------------------

/// `Virtualization` gathers together all functionality needed to allow to to run the application
/// intest mode without any devices.
pub struct Virtualization {
    vnc: Option<remote_desktop::Vnc>,
}

// -------------------------------------------------------------------------------------------------

impl Virtualization {
    /// Constructs new `Virtualization`.
    pub fn new(input_handler: Box<InputHandling>,
               input_forwarder: Box<InputForwarding>,
               dispatcher: dharma::DispatcherController,
               signaler: dharma::Signaler<Perceptron>)
               -> Self {
        let vnc = remote_desktop::Vnc::new(input_handler, input_forwarder, dispatcher, signaler);
        Virtualization {
            vnc: vnc.ok(),
        }
    }

    /// This method is called when new remote desktop client was connected.
    pub fn on_client_connected(&mut self, fd: RawFd) {
        if let Some(ref mut vnc) = self.vnc {
            vnc.handle_client_connection(fd);
        }
    }

    /// This method is called when remote desktop client was disconnected.
    pub fn on_client_disconnected(&mut self, id: u64) {
        if let Some(ref mut vnc) = self.vnc {
            vnc.handle_client_disconnection(id);
        }
    }
}

// -------------------------------------------------------------------------------------------------
