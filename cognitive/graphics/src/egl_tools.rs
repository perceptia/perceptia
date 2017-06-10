// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common EGL-related tools.

// -------------------------------------------------------------------------------------------------

use libc;
use std;
use egl;

use errors::GraphicsError;
use attributes::{DmabufAttributes, EglAttributes};

// -------------------------------------------------------------------------------------------------

/// Raw hardware image.
pub type RawHwImage = *const std::os::raw::c_void;

// -------------------------------------------------------------------------------------------------

/// Wrapper for hardware image. Conveying its size and adding ability to send between threads.
#[derive(Clone, Debug)]
pub struct HwImage {
    image: RawHwImage,
    width: usize,
    height: usize,
}

/// `HwImage` contains only simple data and handle to raw image. It is as safe to send it as to use.
unsafe impl Send for HwImage {}

// -------------------------------------------------------------------------------------------------

impl HwImage {
    /// Constructs new `HwImage`.
    pub fn new(image: RawHwImage, width: usize, height: usize) -> Self {
        HwImage {
            image: image,
            width: width,
            height: height,
        }
    }

    /// Returns raw image.
    pub fn as_raw(&self) -> RawHwImage {
        self.image
    }

    /// Returns width of the image.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Returns height of the image.
    pub fn get_height(&self) -> usize {
        self.height
    }
}

// -------------------------------------------------------------------------------------------------

/// Module with some constants for extensions.
pub mod ext {
    use egl;
    use super::RawHwImage;

    // Extension names
    pub const IMAGE_BASE_EXT: &'static str = "EGL_KHR_image_base";
    pub const IMAGE_EXTERNAL_EXT: &'static str = "GL_OES_EGL_image_external";

    pub const DRM_BUFFER_USE_SHARE_MESA: egl::EGLint = 0x00000002;
    pub const OPENGL_ES3_BIT_KHR: egl::EGLint = 0x00000040;

    pub const DRM_BUFFER_FORMAT_MESA: egl::EGLint = 0x31D0;
    pub const DRM_BUFFER_USE_MESA: egl::EGLint = 0x31D1;
    pub const DRM_BUFFER_FORMAT_ARGB32_MESA: egl::EGLint = 0x31D2;
    pub const DRM_BUFFER_MESA: egl::EGLenum = 0x31D3;
    pub const DRM_BUFFER_STRIDE_MESA: egl::EGLint = 0x31D4;

    pub const PLATFORM_GBM_KHR: egl::EGLenum = 0x31D7;

    pub const LINUX_DMA_BUF_EXT: egl::EGLenum = 0x3270;
    pub const LINUX_DRM_FOURCC_EXT: egl::EGLint = 0x3271;
    pub const DMA_BUF_PLANE0_FD_EXT: egl::EGLint = 0x3272;
    pub const DMA_BUF_PLANE0_OFFSET_EXT: egl::EGLint = 0x3273;
    pub const DMA_BUF_PLANE0_PITCH_EXT: egl::EGLint = 0x3274;

    /// Indicates image creation failure.
    pub const NO_IMAGE: RawHwImage = 0 as RawHwImage;
}

// -------------------------------------------------------------------------------------------------

/// List of attributes for create of configuration.
#[cfg_attr(rustfmt, rustfmt_skip)]
const CONFIG_ATTRIB_LIST: [egl::EGLint; 13] = [
        egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
        egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
        egl::EGL_RED_SIZE,        1,
        egl::EGL_GREEN_SIZE,      1,
        egl::EGL_BLUE_SIZE,       1,
        egl::EGL_ALPHA_SIZE,      1,
        egl::EGL_NONE
    ];

/// List of attributes for create of context.
const CONTEXT_ATTRIB_LIST: [egl::EGLint; 3] = [egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE];

/// List of attributes for create of surface.
const SURFACE_ATTRIB_LIST: [egl::EGLint; 0] = [];

// -------------------------------------------------------------------------------------------------

/// Type definition for `eglGetPlatformDisplayEXT` function.
pub type GetPlatformDisplayFn = extern "C" fn(egl::EGLenum,
                                              egl::EGLNativeDisplayType,
                                              *const egl::EGLint)
                                              -> egl::EGLDisplay;

/// Type definition for `eglGetPlatformDisplayEXT` function.
pub type CreatePlatformSurfaceFn = extern "C" fn(egl::EGLDisplay,
                                                 egl::EGLConfig,
                                                 egl::EGLNativeWindowType,
                                                 *const egl::EGLint)
                                                 -> egl::EGLSurface;

/// Type definition for `eglCreateImageKHR` function.
pub type CreateImageKhrFn = extern "C" fn(egl::EGLDisplay,
                                          egl::EGLContext,
                                          egl::EGLenum,
                                          egl::EGLClientBuffer,
                                          *const egl::EGLint)
                                          -> RawHwImage;

/// Type definition for `eglCreateImageKHR` function.
pub type DestroyImageKhrFn = extern "C" fn(egl::EGLDisplay, RawHwImage) -> egl::EGLBoolean;

/// Type definition for `eglCreateDRMImageMESA` function.
pub type CreateDrmImageMesaFn = extern "C" fn(egl::EGLDisplay, *const egl::EGLint) -> RawHwImage;

/// Type definition for `eglExportDRMImageMESA` function.
pub type ExportDrmImageFn = extern "C" fn(egl::EGLDisplay,
                                          RawHwImage,
                                          *mut egl::EGLint,
                                          *mut egl::EGLint,
                                          *mut egl::EGLint)
                                          -> egl::EGLBoolean;

/// Type definition for `glEGLImageTargetTexture2DOES` function.
pub type ImageTargetTexture2DOesFn = extern "C" fn(egl::EGLenum, RawHwImage);

/// Type definition for `glEGLImageTargetRenderbufferStorageOES` function.
pub type ImageTargetRenderStorageFn = extern "C" fn(egl::EGLenum, RawHwImage);

// -------------------------------------------------------------------------------------------------

/// Returns address of extension function.
pub fn get_proc_addr_of_get_platform_display() -> Option<GetPlatformDisplayFn> {
    unsafe {
        let func = egl::get_proc_address("eglGetPlatformDisplayEXT") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, GetPlatformDisplayFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_create_platform_surface() -> Option<CreatePlatformSurfaceFn> {
    unsafe {
        let func = egl::get_proc_address("eglCreatePlatformWindowSurfaceEXT") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, CreatePlatformSurfaceFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_create_image_khr() -> Option<CreateImageKhrFn> {
    unsafe {
        let func = egl::get_proc_address("eglCreateImageKHR") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, CreateImageKhrFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_destroy_image_khr() -> Option<DestroyImageKhrFn> {
    unsafe {
        let func = egl::get_proc_address("eglDestroyImageKHR") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, DestroyImageKhrFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_create_drm_image_mesa() -> Option<CreateDrmImageMesaFn> {
    unsafe {
        let func = egl::get_proc_address("eglCreateDRMImageMESA") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, CreateDrmImageMesaFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_export_drm_image_mesa() -> Option<ExportDrmImageFn> {
    unsafe {
        let func = egl::get_proc_address("eglExportDRMImageMESA") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, ExportDrmImageFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_image_target_texture_2d_oes() -> Option<ImageTargetTexture2DOesFn> {
    unsafe {
        let func = egl::get_proc_address("glEGLImageTargetTexture2DOES") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, ImageTargetTexture2DOesFn>(func))
        } else {
            None
        }
    }
}

/// Returns address of extension function.
pub fn get_proc_addr_of_image_target_render_storage_oes() -> Option<ImageTargetRenderStorageFn> {
    unsafe {
        let func = egl::get_proc_address("glEGLImageTargetRenderbufferStorageOES") as *const ();
        if !func.is_null() {
            Some(std::mem::transmute::<_, ImageTargetRenderStorageFn>(func))
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Gets GBM display.
///
/// First tries `eglGetDisplay`. If that fails, tries `eglGetPlatformDisplayEXT`.
pub fn get_gbm_display(native_display: egl::EGLNativeDisplayType)
                       -> Result<egl::EGLDisplay, GraphicsError> {
    if let Some(display) = egl::get_display(native_display) {
        Ok(display)
    } else {
        if let Some(get_platform_display) = self::get_proc_addr_of_get_platform_display() {
            let display =
                get_platform_display(ext::PLATFORM_GBM_KHR, native_display, std::ptr::null());
            if !display.is_null() {
                Ok(display)
            } else {
                Err(GraphicsError::new(format!("Failed to get EGL display")))
            }
        } else {
            Err(GraphicsError::new(format!("GBM platform is not supported")))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Returns true if extension is available, false otherwise.
pub fn has_extension(display: egl::EGLDisplay, extension: &str) -> bool {
    if let Some(extensions) = egl::query_string(display, egl::EGL_EXTENSIONS) {
        if let Ok(extensions) = extensions.to_owned().into_string() {
            extensions.contains(extension)
        } else {
            false
        }
    } else {
        false
    }
}

// -------------------------------------------------------------------------------------------------

/// Creates EGL image from given parameters.
pub fn create_image(display: egl::EGLDisplay, attrs: &EglAttributes) -> Option<HwImage> {
    if let Some(create_image) = get_proc_addr_of_create_image_khr() {
        // Create attributes
        let mut attribs = [egl::EGL_NONE; 9];

        attribs[0] = egl::EGL_WIDTH;
        attribs[1] = attrs.width;
        attribs[2] = egl::EGL_HEIGHT;
        attribs[3] = attrs.height;
        attribs[4] = ext::DRM_BUFFER_STRIDE_MESA;
        attribs[5] = (attrs.stride / 4) as egl::EGLint;
        attribs[6] = ext::DRM_BUFFER_FORMAT_MESA;
        attribs[7] = ext::DRM_BUFFER_FORMAT_ARGB32_MESA;
        attribs[8] = egl::EGL_NONE;

        // Create image
        let img = create_image(display,
                               egl::EGL_NO_CONTEXT,
                               ext::DRM_BUFFER_MESA,
                               attrs.name as *mut libc::c_void,
                               (&attribs) as *const egl::EGLint);

        if img != ext::NO_IMAGE {
            Some(HwImage::new(img, attrs.width as usize, attrs.height as usize))
        } else {
            None
        }
    } else {
        None
    }
}

// -------------------------------------------------------------------------------------------------

/// Imports dmabuf as EGL image.
pub fn import_dmabuf(display: egl::EGLDisplay, attrs: &DmabufAttributes) -> Option<HwImage> {
    if let Some(create_image) = get_proc_addr_of_create_image_khr() {
        // Create attributes
        let mut attribs = [egl::EGL_NONE; 25];

        attribs[0] = egl::EGL_WIDTH;
        attribs[1] = attrs.width;
        attribs[2] = egl::EGL_HEIGHT;
        attribs[3] = attrs.height;
        attribs[4] = ext::LINUX_DRM_FOURCC_EXT;
        attribs[5] = attrs.format as egl::EGLint;

        for i in 0..attrs.get_num_of_planes() {
            let idx = 5 + (6 * i);
            attribs[idx + 1] = ext::DMA_BUF_PLANE0_FD_EXT;
            attribs[idx + 2] = attrs.planes[i].fd;
            attribs[idx + 3] = ext::DMA_BUF_PLANE0_OFFSET_EXT;
            attribs[idx + 4] = attrs.planes[i].offset as egl::EGLint;
            attribs[idx + 5] = ext::DMA_BUF_PLANE0_PITCH_EXT;
            attribs[idx + 6] = attrs.planes[i].stride as egl::EGLint;
        }

        // Create image
        let img = create_image(display,
                               egl::EGL_NO_CONTEXT,
                               ext::LINUX_DMA_BUF_EXT,
                               std::ptr::null_mut(),
                               (&attribs) as *const egl::EGLint);

        if img != ext::NO_IMAGE {
            Some(HwImage::new(img, attrs.width as usize, attrs.height as usize))
        } else {
            None
        }
    } else {
        None
    }
}

// -------------------------------------------------------------------------------------------------

/// Destroys given EGL image.
pub fn destroy_image(display: egl::EGLDisplay, image: HwImage) -> Result<(), ()> {
    if let Some(destroy_image) = get_proc_addr_of_destroy_image_khr() {
        // Create image
        let result = destroy_image(display, image.as_raw());
        if result == egl::EGL_TRUE {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
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

impl EglBucket {
    /// Destroys surface, context and terminates display.
    pub fn destroy(self) {
        egl::destroy_surface(self.display, self.surface);
        egl::destroy_context(self.display, self.context);
        egl::terminate(self.display);
    }
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
    pub fn new(native_display: egl::EGLNativeDisplayType,
               window_type: egl::EGLNativeWindowType)
               -> Result<Self, GraphicsError> {
        // Get display
        let display = self::get_gbm_display(native_display)?;

        // Initialize EGL
        let mut major = 0;
        let mut minor = 0;
        if !egl::initialize(display, &mut major, &mut minor) {
            return Err(GraphicsError::new(format!("Failed to initialize EGL")));
        };

        if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
            return Err(GraphicsError::new(format!("Failed to bind EGL API")));
        };

        // Choose config
        let config = if let Some(config) = egl::choose_config(display, &CONFIG_ATTRIB_LIST, 1) {
            config
        } else {
            return Err(GraphicsError::new(format!("Failed to choose EGL config")));
        };

        // Create context
        let c = egl::create_context(display, config, egl::EGL_NO_CONTEXT, &CONTEXT_ATTRIB_LIST);
        let context = if let Some(context) = c {
            context
        } else {
            return Err(GraphicsError::new(format!("Failed to create EGL context")));
        };

        // Create window surface
        let s = egl::create_window_surface(display, config, window_type, &SURFACE_ATTRIB_LIST);
        let surface = if let Some(surface) = s {
            surface
        } else {
            return Err(GraphicsError::new(format!("Failed to create EGL window surface")));
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
    pub fn make_current(&self) -> Result<EglContext, GraphicsError> {
        if !egl::make_current(self.display, self.surface, self.surface, self.context) {
            Err(GraphicsError::new(format!("Failed to make EGL context current")))
        } else {
            Ok(EglContext::new(*self))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl EglContext {
    /// `EglContext` constructor.
    fn new(egl: EglBucket) -> Self {
        EglContext { egl: egl }
    }
}

// -------------------------------------------------------------------------------------------------

impl EglContext {
    /// Release EGL context.
    fn release(&self) -> Result<(), GraphicsError> {
        if !egl::make_current(self.egl.display,
                              egl::EGL_NO_SURFACE,
                              egl::EGL_NO_SURFACE,
                              egl::EGL_NO_CONTEXT) {
            Err(GraphicsError::new(format!("Failed to release EGL context")))
        } else {
            Ok(())
        }
    }

    /// Swap buffers.
    pub fn swap_buffers(&self) -> Result<(), GraphicsError> {
        if egl::swap_buffers(self.egl.display, self.egl.surface) {
            Ok(())
        } else {
            Err(GraphicsError::new(format!("Failed to swap EGL buffers (0x{:x})",
                                           egl::get_error())))
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
