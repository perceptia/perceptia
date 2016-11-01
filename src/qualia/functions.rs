// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains global functions.

// -------------------------------------------------------------------------------------------------

use libc;
use std;

// -------------------------------------------------------------------------------------------------

const UNKNOWN_MESSAGE: &'static str = "<unknown>";

// -------------------------------------------------------------------------------------------------

/// Shuts down the application by sending `SIGTERM` to itself.
pub fn quit() {
    log_info1!("QUIT!");
    unsafe { libc::kill(libc::getpid(), libc::SIGTERM) };
}

// -------------------------------------------------------------------------------------------------

/// Hook function for panics.
///
/// Logs panic message and location and quits application.
pub fn panic_hook(info: &std::panic::PanicInfo) {
    // Retrieve payload
    let payload = info.payload();
    let message: String = if std::any::Any::is::<String>(payload) {
        if let Some(message) = payload.downcast_ref::<String>() {
            message.clone()
        } else {
            UNKNOWN_MESSAGE.to_owned()
        }
    } else if std::any::Any::is::<&str>(payload) {
        if let Some(message) = payload.downcast_ref::<&str>() {
            message.to_string()
        } else {
            UNKNOWN_MESSAGE.to_owned()
        }
    } else {
        UNKNOWN_MESSAGE.to_owned()
    };

    // Log panic
    log_error!("One of threads panicked with message '{}'", message);
    if let Some(location) = info.location() {
        log_error!("Panic occurred in line {}, file '{}'", location.line(), location.file());
    }

    // Quit application
    quit();
}

// -------------------------------------------------------------------------------------------------
