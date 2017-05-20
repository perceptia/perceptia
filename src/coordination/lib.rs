// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `coordination` keeps functionality related to memory shared between threads that requires
//! synchronized access like buffers and related information for surfaces, screenshots or data
//! transfers.
//!
//! TODO: Other crates should not use `coordination` directly but by traits provided by `qualia`.
//! Ideally `coordination` should be merged with `perceptia` to prevent such use.

#[macro_use(timber)]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate dharma;

mod surfaces;

pub mod resource_storage;
pub use resource_storage::ResourceStorage;

pub mod coordinator;
pub use coordinator::Coordinator;

pub mod context;
pub use context::Context;
