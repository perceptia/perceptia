// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common GBM-related tools.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;
use libgbm;

use qualia::{Illusion, Size};

// -------------------------------------------------------------------------------------------------

/// This structure collects GBM-related data.
pub struct GbmBucket {
    /// GBM device.
    pub device: libgbm::Device,

    /// GBM surface.
    pub surface: libgbm::Surface,
}

// -------------------------------------------------------------------------------------------------

impl GbmBucket {
    /// `GbmBucket` constructor.
    pub fn new(fd: io::RawFd, size: Size) -> Result<Self, Illusion> {
        // Create device
        let device = if let Some(device) = libgbm::Device::from_fd(fd) {
            device
        } else {
            return Err(Illusion::General(format!("Failed to create GBM device")));
        };

        // Create surface
        let surface = if let Some(surface) = libgbm::Surface::new(&device,
                                                                  size.width,
                                                                  size.height,
                                                                  libgbm::format::XRGB8888,
                                                                  libgbm::USE_SCANOUT |
                                                                  libgbm::USE_RENDERING) {
            surface
        } else {
            return Err(Illusion::General(format!("Failed to create GBM surface")));
        };


        Ok(GbmBucket {
            device: device,
            surface: surface,
        })
    }
}

// -------------------------------------------------------------------------------------------------
