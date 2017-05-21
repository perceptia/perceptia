// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Defines for graphics related functionality like DRM, EGL and linux dmabuf.

use std;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use defs::Size;
use image::{Image, PixelFormat};

// -------------------------------------------------------------------------------------------------

pub const MAX_PLANES: usize = 3;

// -------------------------------------------------------------------------------------------------

/// Result of validation of image attributes.
#[derive(PartialEq)]
pub enum ValidationResult {
    /// Plane index out of bounds.
    PlaneIdx,

    /// The plane index was already set.
    PlaneSet,

    /// Missing or too many planes to create a buffer.
    Incomplete,

    /// Format not supported.
    InvalidFormat,

    /// Invalid width or height.
    InvalidDimensions,

    /// Offset + stride * height goes out of dmabuf bounds.
    OutOfBounds,

    /// Everything Ok
    Ok,
}

// -------------------------------------------------------------------------------------------------

/// Raw hardware image.
pub type RawHwImage = *const std::os::raw::c_void;

/// Wrapper for hardware image. Adds `Image` implementation and ability to send between threads.
#[derive(Clone, Debug)]
pub struct HwImage {
    image: RawHwImage,
    size: Size,
}

/// `HwImage` contains only simple data and handle to raw image. It is as safe to send it as to use.
unsafe impl Send for HwImage {}

// -------------------------------------------------------------------------------------------------

impl HwImage {
    /// Constructs new `HwImage`.
    pub fn new(image: RawHwImage, width: usize, height: usize) -> Self {
        HwImage {
            image: image,
            size: Size::new(width, height),
        }
    }

    /// Return raw image.
    pub fn as_raw(&self) -> RawHwImage {
        self.image
    }
}

impl Image for HwImage {
    /// Get width and height of the image.
    fn get_size(&self) -> Size {
        self.size
    }

    /// Return width of the image.
    fn get_width(&self) -> usize {
        self.size.width
    }

    /// Returns height of the image.
    fn get_height(&self) -> usize {
        self.size.height
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure containing all data needed to initialize DRM output.
#[derive(Clone, Debug)]
pub struct DrmBundle {
    pub path: PathBuf,
    pub fd: RawFd,
    pub crtc_id: u32,
    pub connector_id: u32,
}

// -------------------------------------------------------------------------------------------------

/// Attributes for creation EGL image.
#[derive(Debug, Clone)]
pub struct EglAttributes {
    pub name: u32,
    pub width: i32,
    pub height: i32,
    pub stride: u32,
    pub format: PixelFormat,
}

// -------------------------------------------------------------------------------------------------

impl EglAttributes {
    /// Constructs new `EglAttributes`.
    pub fn new(name: u32, width: i32, height: i32, stride: u32, format: PixelFormat) -> Self {
        EglAttributes {
            name: name,
            width: width,
            height: height,
            stride: stride,
            format: format,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Image for EglAttributes {
    /// Get width and height of the underlying hardware image.
    fn get_size(&self) -> Size {
        Size::new(self.width as usize, self.height as usize)
    }

    /// Return width of the underlying hardware image.
    fn get_width(&self) -> usize {
        self.width as usize
    }

    /// Returns height of the underlying hardware image.
    fn get_height(&self) -> usize {
        self.height as usize
    }
}

// -------------------------------------------------------------------------------------------------

/// Attributes for creation of plane for EGL image from dmabuf.
#[derive(Debug, Copy, Clone)]
pub struct DmabufPlane {
    pub fd: RawFd,
    pub offset: u32,
    pub stride: u32,
    pub modifier_hi: u32,
    pub modifier_lo: u32,
}

// -------------------------------------------------------------------------------------------------

impl DmabufPlane {
    /// Constructs new `DmabufPlane`.
    fn new(fd: RawFd, offset: u32, stride: u32, modifier_hi: u32, modifier_lo: u32) -> Self {
        DmabufPlane {
            fd: fd,
            offset: offset,
            stride: stride,
            modifier_hi: modifier_hi,
            modifier_lo: modifier_lo,
        }
    }

    /// Constructs default `DmabufPlane`.
    fn default() -> Self {
        DmabufPlane {
            fd: -1,
            offset: 0,
            stride: 0,
            modifier_hi: 0x0,
            modifier_lo: 0x0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Attributes for creation of EGL image from dmabuf.
///
/// TODO: Add unit tests for `DmabufAttributes`.
#[derive(Debug, Clone)]
pub struct DmabufAttributes {
    pub width: i32,
    pub height: i32,
    pub format: u32,
    pub flags: u32,
    pub num_planes: usize,
    pub planes: [DmabufPlane; MAX_PLANES],
}

// -------------------------------------------------------------------------------------------------

impl DmabufAttributes {
    /// Constructs new `DmabufAttributes`.
    pub fn new() -> Self {
        DmabufAttributes {
            width: 0,
            height: 0,
            format: 0,
            flags: 0x0,
            num_planes: 0,
            planes: [DmabufPlane::default(), DmabufPlane::default(), DmabufPlane::default()],
        }
    }

    /// Adds attributes for plane.
    pub fn add(&mut self,
               plane_idx: usize,
               fd: RawFd,
               offset: u32,
               stride: u32,
               modifier_hi: u32,
               modifier_lo: u32)
               -> ValidationResult {
        if plane_idx >= MAX_PLANES {
            return ValidationResult::PlaneIdx;
        }

        if self.planes[plane_idx].fd != -1 {
            return ValidationResult::PlaneSet;
        }

        self.planes[plane_idx] = DmabufPlane::new(fd, offset, stride, modifier_hi, modifier_lo);
        self.num_planes += 1;
        ValidationResult::Ok
    }

    /// Sets image parameters.
    pub fn create(&mut self, width: i32, height: i32, format: u32, flags: u32) {
        self.width = width;
        self.height = height;
        self.format = format;
        self.flags = flags;
    }

    /// Validates the attributes.
    ///
    /// TODO: Add more validation checks.
    pub fn validate(&self) -> ValidationResult {
        for i in 0..self.num_planes {
            if self.planes[i].fd == -1 {
                return ValidationResult::Incomplete;
            }
        }
        ValidationResult::Ok
    }

    /// Returns number of planes configured.
    pub fn get_num_of_planes(&self) -> usize {
        self.num_planes
    }
}

// -------------------------------------------------------------------------------------------------

impl Image for DmabufAttributes {
    /// Get width and height of the underlying hardware image.
    fn get_size(&self) -> Size {
        Size::new(self.width as usize, self.height as usize)
    }

    /// Return width of the underlying hardware image.
    fn get_width(&self) -> usize {
        self.width as usize
    }

    /// Returns height of the underlying hardware image.
    fn get_height(&self) -> usize {
        self.height as usize
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait every graphics manager should implement.
///
/// Graphics manager is peace of code abstracting hardware image creation.
pub trait GraphicsManagement {
    /// Creates EGL image from given parameters.
    fn create_egl_image(&mut self, attrs: &EglAttributes) -> Option<HwImage>;

    /// Imports dmabuf as EGL image.
    fn import_dmabuf(&mut self, attrs: &DmabufAttributes) -> Option<HwImage>;

    /// Destroys given hardware image.
    fn destroy_hw_image(&mut self, image: HwImage) -> Result<(), ()>;
}

// -------------------------------------------------------------------------------------------------
