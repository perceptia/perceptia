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

#![warn(missing_docs)]

pub extern crate dharma;
pub extern crate timber;
pub extern crate cognitive_graphics as graphics;
