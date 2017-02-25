// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Defines data structures and functionality used to build and manipulate space-like and time-like
//! relations between surfaces.
//!
//! ## Structure
//!
//! Frames are organized in tree-like structure with one root. Children of every branch have two
//! orders:
//!
//!  - *time-like* - describing the order of use of frames in given branch
//!  - *space-like* - describing placement order as drawn on screen
//!
//! ## Manipulations
//!
//! Basic manipulation in the tree is to *append*, *prepend* or *join* frames in spatial order.
//! Using these manipulations added frame always becomes last in time order. To become first in
//! time order the frame must be *pop*-ed.
//!
//! ## Extensions
//!
//! Extensions to basic functionality are implemented by traits first to clearly separate
//! functionalities, secondly to make files shorter by breaking code to different modules.
//!
//!  - `searching` - gives more advance or common ways to find specified frames
//!  - `settle` - implements common ways of adding or moving frames
//!
//! ## Implementation
//!
//! Frame tree is cyclic graph with each node optionally pointing to:
//!
//!  - parent
//!  - next sibling in time order
//!  - previous sibling in time order
//!  - first child in time order
//!  - last child in time order
//!  - next sibling in space order
//!  - previous sibling in space order
//!  - first child in space order
//!  - last child in space order
//!
//! Current implementation uses unsafe raw pointers. This make implementation faster and simpler
//! than with other more idiomatic ways, but loses Rusts guaranties. Runtime safety is ensured by
//! unit tests.

// -------------------------------------------------------------------------------------------------

#![feature(alloc, alloc_system, heap_api)]
#![feature(alloc_system)]

extern crate alloc;
extern crate alloc_system;

extern crate qualia;

mod frame;
pub use frame::{Frame, FrameSpaceIterator, FrameTimeIterator, Mode, Geometry, Side, Parameters};

mod displaying;
pub use displaying::Displaying;

pub mod packing;
pub mod searching;
pub mod settling;

// -------------------------------------------------------------------------------------------------
