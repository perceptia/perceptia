// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality for storing time.

use std::time::{Duration, Instant};

// -------------------------------------------------------------------------------------------------

/// This structure represents amount of time in milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Milliseconds {
    ms: u64,
}

// -------------------------------------------------------------------------------------------------

impl Milliseconds {
    pub fn elapsed_from(instant: &Instant) -> Self {
        Self::from_duration(&instant.elapsed())
    }

    pub fn from_duration(d: &Duration) -> Self {
        Milliseconds { ms: d.as_secs() * 1000 + d.subsec_nanos() as u64 / 1000000 }
    }

    pub fn get_value(&self) -> u64 {
        self.ms
    }
}

// -------------------------------------------------------------------------------------------------
