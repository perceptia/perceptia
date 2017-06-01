// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module is wrapper for `timber` crate. It defines application specific log levels and
//! macros.
//!
//! ## Log levels
//!
//! Log levels:
//!
//!  - `ERROR` - indicates heavy but recoverable error. Application may be not fully usable after
//!              such kind of error.
//!  - `DEBUG` - temporary logs for debugging current problem. Logs on this level should not be
//!              committed.
//!  - `NYIMP` - not yet implemented. This log indicates some functionality is planned but not
//!              implemented. It is another way of saying `unimplemented!()` but without panicking.
//!  - `WARN*` - something went wrong, but application can handle this. User may want to fix the
//!              problem.
//!  - `INFO*` - general info that something happened.
//!
//! In surface compositors or window managers some logs may be printed 60 times per second. It is
//! not always desirable to have them all always switched on. Numbers along level names mean:
//!
//!  - `*1` - very important, very rare log
//!  - `*2` - mildly important, rare log
//!  - `*3` - mildly important, frequent log
//!  - `*4` - not important, very frequent log

use backtrace;

use timber;

// -------------------------------------------------------------------------------------------------

pub mod level {
    pub const FATAL: i32 = 1;
    pub const ERROR: i32 = 2;
    pub const DEBUG: i32 = 3;
    pub const NYIMP: i32 = 4;
    pub const WARN1: i32 = 5;
    pub const INFO1: i32 = 6;
    pub const WAYL1: i32 = 7;
    pub const WARN2: i32 = 8;
    pub const INFO2: i32 = 9;
    pub const WAYL2: i32 = 0;
    pub const WARN3: i32 = 0;
    pub const INFO3: i32 = 0;
    pub const WAYL3: i32 = 0;
    pub const WARN4: i32 = 0;
    pub const INFO4: i32 = 0;
    pub const WAYL4: i32 = 0;
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! log_fatal{($($arg:tt)*) => {timber!($crate::level::FATAL, "FATAL", $($arg)*)}}
#[macro_export]
macro_rules! log_error{($($arg:tt)*) => {timber!($crate::level::ERROR, "ERROR", $($arg)*)}}
#[macro_export]
macro_rules! log_debug{($($arg:tt)*) => {timber!($crate::level::DEBUG, "DEBUG", $($arg)*)}}
#[macro_export]
macro_rules! log_nyimp{($($arg:tt)*) => {timber!($crate::level::NYIMP, "NYIMP", $($arg)*)}}
#[macro_export]
macro_rules! log_warn1{($($arg:tt)*) => {timber!($crate::level::WARN1, "WARN1", $($arg)*)}}
#[macro_export]
macro_rules! log_info1{($($arg:tt)*) => {timber!($crate::level::INFO1, "INFO1", $($arg)*)}}
#[macro_export]
macro_rules! log_wayl1{($($arg:tt)*) => {timber!($crate::level::WAYL1, "WAYL1", $($arg)*)}}
#[macro_export]
macro_rules! log_warn2{($($arg:tt)*) => {timber!($crate::level::WARN2, "WARN2", $($arg)*)}}
#[macro_export]
macro_rules! log_info2{($($arg:tt)*) => {timber!($crate::level::INFO2, "INFO2", $($arg)*)}}
#[macro_export]
macro_rules! log_wayl2{($($arg:tt)*) => {timber!($crate::level::WAYL2, "WAYL2", $($arg)*)}}
#[macro_export]
macro_rules! log_warn3{($($arg:tt)*) => {timber!($crate::level::WARN3, "WARN3", $($arg)*)}}
#[macro_export]
macro_rules! log_info3{($($arg:tt)*) => {timber!($crate::level::INFO3, "INFO3", $($arg)*)}}
#[macro_export]
macro_rules! log_wayl3{($($arg:tt)*) => {timber!($crate::level::WAYL3, "WAYL3", $($arg)*)}}
#[macro_export]
macro_rules! log_warn4{($($arg:tt)*) => {timber!($crate::level::WARN4, "WARN4", $($arg)*)}}
#[macro_export]
macro_rules! log_info4{($($arg:tt)*) => {timber!($crate::level::INFO4, "INFO4", $($arg)*)}}
#[macro_export]
macro_rules! log_wayl4{($($arg:tt)*) => {timber!($crate::level::WAYL4, "WAYL4", $($arg)*)}}

// -------------------------------------------------------------------------------------------------

/// This macro behaves much like `try!` but logs on error.
#[macro_export]
macro_rules! ensure {
    ($result:expr) => {
        match $result {
            Ok(ok) => ok,
            Err(err) => {
                log_error!("Ensurence failed: {:?}", err);
                return Err(::std::convert::From::from(err));
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub fn backtrace() {
    let mut timber = timber::lock().unwrap();
    timber.log(format_args!("===============================================\
                             ===============================================\n"));
    if let Some(name) = ::std::thread::current().name() {
        timber.log(format_args!("Backtrace for thread '{}':\n", name));
    } else {
        timber.log(format_args!("Backtrace:"));
    }

    backtrace::trace(|frame| {
        let ip = frame.ip();
        backtrace::resolve(ip, |symbol| {
            let name = if let Some(name) = symbol.name() {
                name
            } else {
                backtrace::SymbolName::new("<unknown>".as_bytes())
            };
            timber.log(format_args!("> {:?}\n", name));
        });
        true
    });

    timber.log(format_args!("===============================================\
                             ===============================================\n"));
}

// -------------------------------------------------------------------------------------------------
