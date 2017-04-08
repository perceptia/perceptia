// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Output Collector.
//!
//! TODO: Implement adding and removing outputs.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use std::os::unix::io;
use nix::fcntl;
use nix::sys::stat;
use libdrm::drm_mode;

use dharma::{DispatcherController, Signaler, event_kind};
use qualia::{DrmBundle, Illusion, perceptron, Perceptron};

use pageflip::PageFlipEventHandler;

// -------------------------------------------------------------------------------------------------

/// Output Collector manages output devices. When output if found or lost Collector notifies the
/// rest of application about this event.
pub struct OutputCollector {
    dispatcher: DispatcherController,
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl OutputCollector {
    /// `OutputCollector` constructor.
    pub fn new(dispatcher: DispatcherController, signaler: Signaler<Perceptron>) -> Self {
        OutputCollector {
            dispatcher: dispatcher,
            signaler: signaler,
        }
    }

    /// Scan DRM devices to find outputs. When the output is found emits `OutputFound` signal.
    pub fn scan_device(&mut self, path: &Path) -> Result<(), Illusion> {
        // Open device
        log_info1!("OutputCollector: scan device '{:?}'", path);
        let fd = match fcntl::open(path, fcntl::O_RDWR, stat::Mode::empty()) {
            Ok(fd) => fd,
            Err(err) => {
                let text = format!("Could open output device {:?}: {}", path, err);
                return Err(Illusion::General(text));
            }
        };

        // Scan for connected outputs
        if let Some(resources) = drm_mode::get_resources(fd) {
            for id in resources.get_connectors() {
                if let Some(connector) = drm_mode::get_connector(fd, id) {
                    self.process_connector(fd, &connector);
                } else {
                    log_warn1!("Failed to get connector info!");
                }
            }
        } else {
            log_warn1!("No resources for device {:?}", path);
        }

        // Register for pageflip events
        let pageflip_event_handler = Box::new(PageFlipEventHandler::new(fd, self.signaler.clone()));
        self.dispatcher.add_source(pageflip_event_handler, event_kind::READ);

        Ok(())
    }

    /// Helper method for `scan_devices`.
    fn process_connector(&mut self, fd: io::RawFd, connector: &drm_mode::Connector) {
        log_info1!("{:?}", connector);

        if connector.get_connection() == drm_mode::Connection::Connected {
            if let Some(encoder) = drm_mode::get_encoder(fd, connector.get_encoder_id()) {
                let bundle = DrmBundle {
                    fd: fd,
                    connector_id: connector.get_connector_id(),
                    crtc_id: encoder.get_crtc_id(),
                };
                self.signaler.emit(perceptron::OUTPUT_FOUND, Perceptron::OutputFound(bundle));
            } else {
                log_warn1!("No encoder for connector '{:?}'", connector.get_connector_id());
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
