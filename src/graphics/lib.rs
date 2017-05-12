// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This create gathers common tools related to hardware graphics.

// -------------------------------------------------------------------------------------------------

extern crate libc;
extern crate gbm_rs as libgbm;
extern crate egl;
extern crate gl;

extern crate qualia;

pub mod gbm_tools;
pub mod egl_tools;
pub mod gl_tools;

mod graphics_manager;
pub use graphics_manager::GraphicsManager;

// -------------------------------------------------------------------------------------------------
