// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality for storing time.

use time;

// -------------------------------------------------------------------------------------------------

/// This structure represents amount of time in milliseconds.
#[derive(Clone, Copy)]
pub struct Milliseconds {
    milliseconds: u64,
}

// -------------------------------------------------------------------------------------------------

impl Milliseconds {
    pub fn now() -> Self {
        let now = time::now();
        Milliseconds { milliseconds: now.tm_sec as u64 * 1000 + now.tm_nsec as u64 / 1000000 }
    }

    pub fn get_value(&self) -> u64 {
        self.milliseconds
    }
}

// -------------------------------------------------------------------------------------------------
