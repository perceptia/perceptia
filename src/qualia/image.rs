// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This create gathers some functionality related to images and access to them.

use defs::Size;

// -------------------------------------------------------------------------------------------------

/// Format of a pixel.
#[derive(Clone, Copy, Debug)]
pub enum PixelFormat {
    XRGB8888,
    ARGB8888,
    XBGR8888,
    ABGR8888,
}

// -------------------------------------------------------------------------------------------------

impl PixelFormat {
    /// Returns size in bytes of pixel encoded in given format.
    pub fn get_size(&self) -> usize {
        match *self {
            PixelFormat::XBGR8888 => 3,
            PixelFormat::ABGR8888 => 4,
            PixelFormat::XRGB8888 => 3,
            PixelFormat::ARGB8888 => 4,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait providing interface for image storing objects.
pub trait Image {
    /// Get width and height of the image.
    fn get_size(&self) -> Size;

    /// Return width of the image.
    fn get_width(&self) -> usize;

    /// Returns height of the image.
    fn get_height(&self) -> usize;
}

// -------------------------------------------------------------------------------------------------

/// Trait providing interface for pixmap storing objects.
pub trait Pixmap : Image {
    /// Returns pixel format of the pixmap.
    fn get_format(&self) -> PixelFormat;

    /// Returns data as slice.
    fn as_slice(&self) -> &[u8];

    /// Returns data as pointer to `u8`.
    unsafe fn as_ptr(&self) -> *const u8;
}

// -------------------------------------------------------------------------------------------------
