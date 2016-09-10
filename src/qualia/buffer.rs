// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Container for all data required to draw an image.

// -------------------------------------------------------------------------------------------------

use defs::Size;

// -------------------------------------------------------------------------------------------------

/// Container for all data required to draw an image.
/// FIXME: Finish implementation of Buffer.
pub struct Buffer {
    width: u32,
    height: u32,
    stride: u32,
    data: Vec<u8>,
}

// -------------------------------------------------------------------------------------------------

impl Buffer {
    /// `Buffer` constructor.
    /// Will panic if passed data size does not match declared size.
    pub fn new(width: u32, height: u32, stride: u32, data: Vec<u8>) -> Self {
        if (stride * height) as usize != data.len() {
            panic!("Data size ({}) does not match declaration ({} * {})",
                   data.len(), stride, height);
        }

        Buffer {
            width: width,
            height: height,
            stride: stride,
            data: data,
        }
    }

    /// Constructor of empty `Buffer`.
    pub fn empty() -> Self {
        Buffer {
            width: 0,
            height: 0,
            stride: 0,
            data: Vec::new(),
        }
    }

    /// Copy data from `other` buffer to `self`.
    pub fn assign_from(&mut self, other: &Buffer) {
        self.width = other.width;
        self.height = other.height;
        self.stride = other.stride;
        // FIXME: This is place for optimisation.
        self.data = other.data.clone();
    }

    /// Get width and height of the buffer.
    #[inline]
    pub fn get_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Check if buffer contains drawable data.
    pub fn is_empty(&self) -> bool {
        (self.width == 0) || (self.height == 0) || (self.stride == 0) || (self.data.len() == 0)
    }
}

// -------------------------------------------------------------------------------------------------
