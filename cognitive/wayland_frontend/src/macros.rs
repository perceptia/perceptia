// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common macros used in this crate.

// -------------------------------------------------------------------------------------------------

/// This macro executes passed expression and in case of error logs warning about failing to send
/// data. It is intended to use when posting `skylane` events to clients.
#[macro_export]
macro_rules! send {
    {$command:expr} => {
        ($command).unwrap_or_else (
            move |err| log_warn2!("Failed to send data to client: {:?}", err)
        );
    }
}

// -------------------------------------------------------------------------------------------------
