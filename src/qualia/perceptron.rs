// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions of events used in whole application.

// -------------------------------------------------------------------------------------------------

use std;

use dharma::{SignalId, Transportable};

use defs::{DrmBundle, SurfaceId};

// -------------------------------------------------------------------------------------------------

pub const SURFACE_READY: SignalId = 0;
pub const OUTPUT_FOUND: SignalId = 1;
pub const VERTICAL_BLANK: SignalId = 2;
pub const PAGE_FLIP: SignalId = 3;

// -------------------------------------------------------------------------------------------------

/// Data passed along with signals. Convention it to use enum values only with corresponding signal
/// identifies.
#[repr(C)]
#[derive(Clone)]
pub enum Perceptron {
    SurfaceReady(SurfaceId),
    OutputFound(DrmBundle),
    VerticalBlank(i32),
    PageFlip(i32),
}

// -------------------------------------------------------------------------------------------------

impl Transportable for Perceptron {}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for Perceptron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Perceptron::SurfaceReady(ref sid) => write!(f, "SurfaceReady({})", sid),
            Perceptron::OutputFound(ref bundle) => write!(f, "OutputFound({:?})", bundle),
            Perceptron::VerticalBlank(ref data) => write!(f, "VerticalBlank({:?})", data),
            Perceptron::PageFlip(ref data) => write!(f, "PageFlip({:?})", data),
        }
    }
}

// -------------------------------------------------------------------------------------------------
