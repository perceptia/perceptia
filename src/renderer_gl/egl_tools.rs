// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common EGL-related tools.

// -------------------------------------------------------------------------------------------------

use egl;

use qualia::Error;

// -------------------------------------------------------------------------------------------------

/// List of attributes for create of configuration.
static CONFIG_ATTRIB_LIST: [egl::EGLint; 13] = [
        egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
        egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
        egl::EGL_RED_SIZE,        1,
        egl::EGL_GREEN_SIZE,      1,
        egl::EGL_BLUE_SIZE,       1,
        egl::EGL_ALPHA_SIZE,      1,
        egl::EGL_NONE
    ];

// -------------------------------------------------------------------------------------------------

/// List of attributes for create of context.
static CONTEXT_ATTRIB_LIST: [egl::EGLint; 3] = [
        egl::EGL_CONTEXT_CLIENT_VERSION, 2,
        egl::EGL_NONE
    ];

// -------------------------------------------------------------------------------------------------

/// List of attributes for create of surface. 
static SURFACE_ATTRIB_LIST: [egl::EGLint; 0] = [];

// -------------------------------------------------------------------------------------------------

/// Log EGL error.
pub fn log_status()
{
    log_info1!("Status - EGL: 0x{:x}", egl::get_error());
}

// -------------------------------------------------------------------------------------------------

/// This structure collects EGL-related data.
#[derive(Clone, Copy)]
pub struct EglBucket {
    pub display: egl::EGLDisplay,
    pub config: egl::EGLConfig,
    pub context: egl::EGLContext,
    pub surface: egl::EGLSurface,
}

// -------------------------------------------------------------------------------------------------

/// This structure is returned by `EglBucket::make_current` and is used to automatically release
/// EGL context when this structure goes out of the scope.
pub struct EglContext {
    egl: EglBucket,
}

// -------------------------------------------------------------------------------------------------

impl EglBucket {
    /// `EglBucket` constructor.
    pub fn new(display_type: egl::EGLNativeDisplayType, window_type: egl::EGLNativeWindowType)
    -> Result<Self, Error> {
        // Get display
        let display = if let Some(display) = egl::get_display(display_type) {
            display
        } else {
            return Err(Error::General(format!("Failed to get EGL display")))
        };

        // Initialize EGL
        let mut major = 0;
        let mut minor = 0;
        if !egl::initialize(display, &mut major, &mut minor) {
            return Err(Error::General(format!("Failed to initialize EGL")))
        };

        if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
            return Err(Error::General(format!("Failed to bind EGL API")))
        };

        // Choose config
        let config = if let Some(config) = egl::choose_config(display, &CONFIG_ATTRIB_LIST, 1) {
            config
        } else {
            return Err(Error::General(format!("Failed to choose EGL config")))
        };

        // Create context
        let context = if let Some(context)
                 = egl::create_context(display, config, egl::EGL_NO_CONTEXT, &CONTEXT_ATTRIB_LIST) {
            context
        } else {
            return Err(Error::General(format!("Failed to create EGL context")))
        };

        // Create window surface
        let surface = if let Some(surface)
                  = egl::create_window_surface(display, config, window_type, &SURFACE_ATTRIB_LIST) {
            surface
        } else {
            return Err(Error::General(format!("Failed to create EGL window surface")))
        };

        // Return bundle
        Ok(EglBucket {
            display: display,
            config: config,
            context: context,
            surface: surface,
        })
    }

    /// Make EGL context current.
    /// This method returns `EglContext` structure which will release context when goes out of the
    /// scope.
    pub fn make_current(&self) -> Result<EglContext, Error> {
        if !egl::make_current(self.display, self.surface, self.surface, self.context) {
            Err(Error::General(format!("Failed to make EGL context current")))
        } else {
            Ok(EglContext::new(*self))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl EglContext {
    /// `EglContext` constructor.
    fn new(egl: EglBucket) -> Self {
        EglContext {
            egl: egl,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl EglContext {
    /// Release EGL context.
    fn release(&self) -> Result<(), Error> {
        if !egl::make_current(self.egl.display,
                              egl::EGL_NO_SURFACE,
                              egl::EGL_NO_SURFACE,
                              egl::EGL_NO_CONTEXT) {
            Err(Error::General(format!("Failed to release EGL context")))
        } else {
            Ok(())
        }
    }

    /// Swap buffers.
    pub fn swap_buffers(&self) -> Result<(), Error> {
        if egl::swap_buffers(self.egl.display, self.egl.surface) {
            Ok(())
        } else {
            Err(Error::General(format!("Failed to swap EGL buffers (0x{:x})", egl::get_error())))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Drop for EglContext {
    fn drop(&mut self) {
        self.release().expect("Failed to release EGL context");
    }
}

// -------------------------------------------------------------------------------------------------
