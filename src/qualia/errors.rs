// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides error enumerations for perceptia.

// -------------------------------------------------------------------------------------------------

use libudev;
use std;

// -------------------------------------------------------------------------------------------------

/// Generic application-wide error.
#[derive(Debug)]
pub enum Error {
    Permissions(String),
    InvalidArgument(String),
    General(String),
    Unknown(String),
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Permissions(ref s) => write!(f, "Wrong permissions: {}", s),
            Error::InvalidArgument(ref s) => write!(f, "Invalid argument: {}", s),
            Error::General(ref s) => write!(f, "{}", s),
            Error::Unknown(ref s) => write!(f, "Unknown error: {}", s),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<libudev::Error> for Error {
    fn from(error: libudev::Error) -> Self {
        Error::General(error.description().to_owned())
    }
}

// -------------------------------------------------------------------------------------------------
