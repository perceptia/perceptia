// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate contains code dedicated to drawing the surfaces and other elements of the scene
//! using software rendering.

extern crate cognitive_qualia as qualia;

pub mod renderer_pixmap;
pub use renderer_pixmap::RendererPixmap;
