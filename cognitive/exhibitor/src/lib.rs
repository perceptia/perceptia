// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides functionality for managing displays, surfaces, handle pointer movements,
//! etc. - high level logic for drawing surfaces.

extern crate rand;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;
extern crate cognitive_outputs as outputs;
extern crate cognitive_frames as frames;

mod surface_history;
pub use surface_history::SurfaceHistory;

mod compositor;
pub use compositor::Compositor;

mod pointer;
pub use pointer::Pointer;

mod display;
pub use display::Display;

mod exhibitor;
pub use exhibitor::Exhibitor;

mod strategies;
mod strategist;
pub use strategist::Strategist;
