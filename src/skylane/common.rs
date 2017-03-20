// Copyright 2016 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Common definitions for server and client parts of Skylane crate.

use std;
use std::error::Error;

use nix;

// -------------------------------------------------------------------------------------------------

/// Error enumeration for all Skylane errors.
#[derive(Debug)]
pub enum SkylaneError {
    IO { description: String },
    Socket { description: String },
    WrongObject { object_id: u32 },
    WrongOpcode {
        name: &'static str,
        object_id: u32,
        opcode: u16,
    },
    Other(String),
}

impl std::convert::From<std::io::Error> for SkylaneError {
    fn from(error: std::io::Error) -> Self {
        SkylaneError::IO { description: error.description().to_owned() }
    }
}

impl std::convert::From<nix::Error> for SkylaneError {
    fn from(error: nix::Error) -> Self {
        SkylaneError::Socket { description: error.description().to_owned() }
    }
}

impl std::convert::From<std::env::VarError> for SkylaneError {
    fn from(error: std::env::VarError) -> Self {
        SkylaneError::Other(error.description().to_owned())
    }
}

// -------------------------------------------------------------------------------------------------

/// Header of Wayland message.
#[repr(C)]
#[derive(Debug)]
pub struct Header {
    /// ID of the referred objects.
    pub object_id: u32,

    /// ID of the called method.
    pub opcode: u16,

    /// Size of the message including header.
    pub size: u16,
}

// -------------------------------------------------------------------------------------------------

/// Structure representing ID of protocol object.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId {
    pub fn new(value: u32) -> Self {
        ObjectId(value)
    }

    /// Returns numerical value of objects ID.
    pub fn get_value(&self) -> u32 {
        self.0
    }

    /// Checks if object ID is valid.
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

impl std::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_value())
    }
}

/// Default ID of main global object.
pub const DISPLAY_ID: ObjectId = ObjectId(1);

// -------------------------------------------------------------------------------------------------
