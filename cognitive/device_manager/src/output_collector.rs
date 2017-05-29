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

use dharma::event_kind;
use qualia::{DrmBundle, Illusion, EventHandling, HwGraphics, StatePublishing};

use graphics_manager::GraphicsManager;
use pageflip::PageFlipEventHandler;

// -------------------------------------------------------------------------------------------------

/// Output Collector manages output devices. When output if found or lost Collector notifies the
/// rest of application about this event.
pub struct OutputCollector<C>
    where C: EventHandling + StatePublishing + HwGraphics + Clone
{
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<C> OutputCollector<C>
    where C: 'static + EventHandling + StatePublishing + HwGraphics + Send + Clone
{
    /// Constructs new `OutputCollector`.
    pub fn new(coordinator: C) -> Self {
        OutputCollector { coordinator: coordinator }
    }

    /// Scan DRM devices to find outputs. When the output is found emits `OutputFound` signal.
    ///
    /// TODO: Add unit tests.
    pub fn scan_device(&mut self, path: &Path) -> Result<(), Illusion> {
        // Open device
        log_info1!("OutputCollector: scan device '{:?}'", path);
        let fd = match fcntl::open(path, fcntl::O_RDWR | fcntl::O_CLOEXEC, stat::Mode::empty()) {
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
                    self.process_connector(fd, path, &connector);
                } else {
                    log_warn1!("Failed to get connector info!");
                }
            }
        } else {
            log_warn1!("No resources for device {:?}", path);
        }

        // Register for pageflip events
        let pageflip_handler = Box::new(PageFlipEventHandler::new(fd, self.coordinator.clone()));
        self.coordinator.add_event_handler(pageflip_handler, event_kind::READ);

        // Create graphics manager
        // FIXME: How to handle many graphic cards?
        match GraphicsManager::new(fd) {
            Ok(graphics_manager) => {
                self.coordinator.set_graphics_manager(Box::new(graphics_manager));
            }
            Err(err) => {
                log_warn1!("Failed to initialize graphics manager: {}", err);
            }
        }

        Ok(())
    }

    /// Helper method for `scan_devices`.
    fn process_connector(&mut self, fd: io::RawFd, path: &Path, connector: &drm_mode::Connector) {
        log_info1!("{:?}", connector);

        if connector.get_connection() == drm_mode::Connection::Connected {
            if let Some(encoder) = drm_mode::get_encoder(fd, connector.get_encoder_id()) {
                let bundle = DrmBundle {
                    path: path.to_owned(),
                    fd: fd,
                    connector_id: connector.get_connector_id(),
                    crtc_id: encoder.get_crtc_id(),
                };
                self.coordinator.publish_output(bundle);
            } else {
                log_warn1!("No encoder for connector '{:?}'", connector.get_connector_id());
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
