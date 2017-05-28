// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common GBM-related tools.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::RawFd;
use libgbm;

use errors::GraphicsError;

// -------------------------------------------------------------------------------------------------

/// This structure collects GBM-related data.
pub struct GbmBucket {
    /// GBM device.
    pub device: libgbm::Device,

    /// GBM surface.
    pub surface: libgbm::Surface,
}

// -------------------------------------------------------------------------------------------------

/// Helper function for getting device.
pub fn get_device(fd: RawFd) -> Result<libgbm::Device, GraphicsError> {
    if let Some(device) = libgbm::Device::from_fd(fd) {
        Ok(device)
    } else {
        Err(GraphicsError::new(format!("Failed to create GBM device")))
    }
}
// -------------------------------------------------------------------------------------------------

impl GbmBucket {
    /// `GbmBucket` constructor.
    pub fn new(fd: RawFd, width: u32, height: u32) -> Result<Self, GraphicsError> {
        // Create device
        let device = self::get_device(fd)?;

        // Create surface
        let surface = if let Some(surface) = libgbm::Surface::new(&device,
                                                                  width,
                                                                  height,
                                                                  libgbm::format::XRGB8888,
                                                                  libgbm::USE_SCANOUT |
                                                                  libgbm::USE_RENDERING) {
            surface
        } else {
            return Err(GraphicsError::new(format!("Failed to create GBM surface")));
        };


        Ok(GbmBucket {
               device: device,
               surface: surface,
           })
    }
}

// -------------------------------------------------------------------------------------------------
