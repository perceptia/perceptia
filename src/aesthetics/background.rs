// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to background image.

// -------------------------------------------------------------------------------------------------

use std::path::PathBuf;

use image;

use qualia::{AestheticsConfig, Buffer, PixelFormat, SurfaceId, AestheticsCoordinationTrait};

// -------------------------------------------------------------------------------------------------

/// State of the background.
pub struct Background<C> where C: AestheticsCoordinationTrait {
    /// Surface ID of the background.
    background_sid: SurfaceId,

    /// Path to background image from configuration.
    background_path: Option<PathBuf>,

    /// Coordinator.
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<C> Background<C> where C: AestheticsCoordinationTrait {
    /// Constructs new `Background`.
    pub fn new(coordinator: C, config: AestheticsConfig) -> Self {
        Background {
            background_sid: SurfaceId::invalid(),
            background_path: config.background_path,
            coordinator: coordinator,
        }
    }

    /// Reads in background image file and creates surface to be displayed as background.
    ///
    /// TODO: Background currently is placed at top left corner. Make it configurable to be
    /// centred, stretched, etc...
    ///
    /// NOTE: `image::open` spawns four threads when opening JPEG images and does not close them.
    pub fn set_background(&mut self) {
        if let Some(ref path) = self.background_path {
            match image::open(&path) {
                Ok(img) => {
                    let rgba = img.to_rgba();
                    let f = PixelFormat::ABGR8888;
                    let w = rgba.width() as usize;
                    let h = rgba.height() as usize;
                    let s = w * f.get_size();
                    let d = rgba.into_raw();

                    let background_sid = self.coordinator.create_surface();
                    let buffer = Buffer::new(f, w, h, s, d);
                    let bid = self.coordinator.create_pool_from_buffer(buffer);

                    if let Some(mvid) = self.coordinator.create_memory_view(bid, f, 0, w, h, s) {
                        self.coordinator.attach_shm(mvid, background_sid);
                        self.coordinator.commit_surface(background_sid);
                        self.coordinator.set_surface_as_background(background_sid);
                    } else {
                        log_warn1!("Failed to create memory view for background");
                    }
                }
                Err(err) => log_warn1!("Failed to open background file: {:?}", err),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<C> Background<C> where C: AestheticsCoordinationTrait {
    /// Handles background surface change request.
    pub fn on_surface_change(&mut self, sid: SurfaceId) {
        self.background_sid = sid;
    }

    /// Handles creation of display.
    pub fn on_display_created(&mut self) {
        if !self.background_sid.is_valid() {
            self.set_background();
        }
    }
}

// -------------------------------------------------------------------------------------------------
