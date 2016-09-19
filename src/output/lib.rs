// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate contains code dedicated to managing output device like buffer swapping or controlling
//! v-blanks.

// -------------------------------------------------------------------------------------------------

extern crate libc;
extern crate egl;
extern crate drm as libdrm;
extern crate gbm_rs as libgbm;

#[macro_use(timber)]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate dharma;
extern crate renderer_gl;

pub mod gbm_tools;
pub mod output;

pub use output::Output;

// -------------------------------------------------------------------------------------------------
