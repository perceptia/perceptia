// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Curate package for crates related to building display compositors or window managers.
//!
//! After crates reexported here mature they all will be moved to new repository.
//!
//! ## Documentation
//!
//! Links to sub-crates documentations:
//!
//!  - [dharma](https://docs.rs/dharma)
//!  - [timber](https://docs.rs/timber)
//!  - [graphics](https://docs.rs/cognitive-graphics)
//!  - [qualia](https://docs.rs/cognitive-qualia)
//!  - [renderer-gl](https://docs.rs/cognitive-renderer-gl)
//!  - [inputs](https://docs.rs/cognitive-inputs)
//!  - [outputs](https://docs.rs/cognitive-outputs)
//!  - [device-manager](https://docs.rs/cognitive-device-manager)

#![warn(missing_docs)]

pub extern crate dharma;
pub extern crate timber;
pub extern crate cognitive_graphics as graphics;
pub extern crate cognitive_qualia as qualia;
pub extern crate cognitive_renderer_gl as renderer_gl;
pub extern crate cognitive_inputs as inputs;
pub extern crate cognitive_outputs as outputs;
pub extern crate cognitive_device_manager as device_manager;
