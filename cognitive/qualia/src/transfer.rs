// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Data structures used for data transfers between clients (e.g. copy-paste).

// -------------------------------------------------------------------------------------------------

/// Data related to transferring data.
#[derive(Clone)]
pub struct Transfer {
    pub mime_types: Vec<String>,
}

// -------------------------------------------------------------------------------------------------

impl Transfer {
    /// Constructs new `Transfer`.
    pub fn new() -> Self {
        Transfer { mime_types: Vec::new() }
    }

    /// Adds offered mime type.
    pub fn add_mime_type(&mut self, mime_type: String) {
        self.mime_types.push(mime_type);
    }

    /// Returns list of all offered mime types.
    pub fn get_mime_types(&self) -> &Vec<String> {
        &self.mime_types
    }
}

// -------------------------------------------------------------------------------------------------
