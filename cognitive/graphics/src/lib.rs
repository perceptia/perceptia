// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This create gathers common tools related to hardware graphics.
//!
//! It is set of loose simple tools. It should be replaced by higher-level crate.

// -------------------------------------------------------------------------------------------------

extern crate libc;
extern crate gbm_rs as libgbm;
extern crate egl;
extern crate gl;

mod errors;
pub use errors::GraphicsError;

pub mod attributes;
pub mod gbm_tools;
pub mod egl_tools;
pub mod gl_tools;

// -------------------------------------------------------------------------------------------------
