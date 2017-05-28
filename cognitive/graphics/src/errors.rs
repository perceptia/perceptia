// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides error enumerations.

// -------------------------------------------------------------------------------------------------

use std;
use std::error::Error;

// -------------------------------------------------------------------------------------------------

/// Graphics errors.
pub struct GraphicsError {
    /// Description.
    description: String,
}

// -------------------------------------------------------------------------------------------------

impl GraphicsError {
    /// Constructs new `GraphicsError` with description.
    pub fn new(description: String) -> GraphicsError {
        GraphicsError { description: description }
    }
}

// -------------------------------------------------------------------------------------------------

impl Error for GraphicsError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Debug for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

// -------------------------------------------------------------------------------------------------
