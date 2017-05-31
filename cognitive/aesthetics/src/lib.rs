// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides aesthetic additions like setting background or default cursor theme.

extern crate image;
extern crate rusttype;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate cognitive_qualia as qualia;

mod cursor;
pub use cursor::Cursor;

mod background;
pub use background::Background;

mod panels;
pub use panels::PanelManager;

mod aesthetics;
pub use aesthetics::Aesthetics;
