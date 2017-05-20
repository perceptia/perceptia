// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common GBM-related tools.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::RawFd;

use egl;
use libgbm;

use qualia::{DmabufAttributes, EglAttributes, GraphicsManagement, Illusion, HwImage};

use gbm_tools;
use egl_tools;

// -------------------------------------------------------------------------------------------------

/// Graphics manager provides basic acces to GPU for non-rendering components.
pub struct GraphicsManager {
    /// GBM device.
    _device: libgbm::Device,

    /// EGL display.
    display: egl::EGLDisplay,
}

/// `GraphicsManager` contains only pointers. It is `Send` but not `Sync`.
unsafe impl Send for GraphicsManager {}

// -------------------------------------------------------------------------------------------------

impl GraphicsManager {
    /// Constructs new `GraphicsManager`.
    pub fn new(fd: RawFd) -> Result<Self, Illusion> {
        // Create device and display
        let device = gbm_tools::get_device(fd)?;
        let display = egl_tools::get_gbm_display(device.c_struct() as egl::EGLNativeDisplayType)?;

        // Initialize EGL
        let mut major = 0;
        let mut minor = 0;
        if !egl::initialize(display, &mut major, &mut minor) {
            return Err(Illusion::General(format!("Failed to initialize EGL")));
        };

        if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
            return Err(Illusion::General(format!("Failed to bind EGL API")));
        };

        // Check for image base extension and related functions
        if egl_tools::has_extension(display, egl_tools::ext::IMAGE_BASE_EXT) {
            if egl_tools::get_proc_address_of_create_image_khr().is_none() {
                return Err(Illusion::General(format!("Failed to get function address")));
            }
        } else {
            return Err(Illusion::General(format!("No {} extension",
                                                 egl_tools::ext::IMAGE_BASE_EXT)));
        }

        Ok(GraphicsManager {
               _device: device,
               display: display,
           })
    }
}

// -------------------------------------------------------------------------------------------------

impl GraphicsManagement for GraphicsManager {
    /// Creates EGL image from given parameters.
    fn create_egl_image(&mut self, attrs: &EglAttributes) -> Option<HwImage> {
        egl_tools::create_image(self.display, attrs)
    }

    /// Imports dmabuf as EGL image.
    fn import_dmabuf(&mut self, attrs: &DmabufAttributes) -> Option<HwImage> {
        egl_tools::import_dmabuf(self.display, attrs)
    }

    /// Destroys given hardware image.
    fn destroy_hw_image(&mut self, image: HwImage) -> Result<(), ()> {
        egl_tools::destroy_image(self.display, image)
    }
}

// -------------------------------------------------------------------------------------------------
