// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Enum definitions for perceptia.

// -------------------------------------------------------------------------------------------------

/// Enum describing kind of input device.
#[derive(PartialEq)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
    Touchpad,
    Unknown,
}

// -------------------------------------------------------------------------------------------------
