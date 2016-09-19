// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate contains code dedicated to managing output device like buffer swapping or controlling
//! v-blanks.

// -------------------------------------------------------------------------------------------------

extern crate gl;
extern crate egl;

#[macro_use(timber)]
extern crate timber;
#[macro_use]
extern crate qualia;

pub mod gl_tools;
pub mod egl_tools;
pub mod renderer_gl;

pub use renderer_gl::RendererGl;

// -------------------------------------------------------------------------------------------------
