// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate contains code dedicated to managing output device like buffer swapping or controlling
//! v-blanks.

// -------------------------------------------------------------------------------------------------

extern crate libc;
extern crate gl;
extern crate egl;

extern crate cognitive_graphics;

// TODO: Enable logging only for debugging.
#[macro_use(timber)]
extern crate timber;
#[macro_use]
extern crate qualia;

mod cache_gl;

pub mod renderer_gl;

pub use renderer_gl::RendererGl;

// -------------------------------------------------------------------------------------------------
