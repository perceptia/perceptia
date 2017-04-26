// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides error enumerations for perceptia.

// -------------------------------------------------------------------------------------------------

use std;
use libudev;

use std::error::Error;

// -------------------------------------------------------------------------------------------------

/// Generic application-wide error.
///
/// This enum could not be named `Error`. Illusion is "a distortion of the senses" (error in
/// perceptia).
#[derive(Debug)]
pub enum Illusion {
    Permissions(String),
    InvalidArgument(String),
    General(String),
    Config(std::path::PathBuf, String),
    IO(String),
    Unknown(String),
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for Illusion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Illusion::Permissions(ref s) => write!(f, "Wrong permissions: {}", s),
            Illusion::InvalidArgument(ref s) => write!(f, "Invalid argument: {}", s),
            Illusion::General(ref s) => write!(f, "{}", s),
            Illusion::Config(ref path, ref s) => write!(f, "Config error ({:?}): {}", path, s),
            Illusion::IO(ref s) => write!(f, "IO error: {}", s),
            Illusion::Unknown(ref s) => write!(f, "Unknown error: {}", s),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<std::io::Error> for Illusion {
    fn from(error: std::io::Error) -> Self {
        Illusion::IO(error.description().to_owned())
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<libudev::Error> for Illusion {
    fn from(error: libudev::Error) -> Self {
        Illusion::General(error.description().to_owned())
    }
}

// -------------------------------------------------------------------------------------------------
