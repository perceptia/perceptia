// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Container for all data required to draw an image.

// -------------------------------------------------------------------------------------------------

/// Container for all data required to draw an image.
/// FIXME: Finish implementation of Buffer.
pub struct Buffer {
    width: u32,
    height: u32,
    stride: u32,
    data: Box<u8>,
}

// -------------------------------------------------------------------------------------------------

impl Buffer {
    /// `Buffer` constructor.
    pub fn new(width: u32, height: u32, stride: u32) -> Self {
        Buffer {
            width: width,
            height: height,
            stride: stride,
            data: Box::new(0),
        }
    }
}

// -------------------------------------------------------------------------------------------------
