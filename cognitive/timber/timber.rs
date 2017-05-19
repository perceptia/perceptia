// Copyright 2016 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! `Timber` is simple logger facility. It provides means to write logs to given file in concurrent
//! applications.
//!
//! `timber!` macro takes as argument level number and writes log only if it is greater than zero.
//! If user defines log levels as constants compiler will be able to ignore strings passed to unused
//! logs and make application smaller. This way user can keep set of debugging logs, but compile
//! them out for release.
//!
//! By default `timber` writes logs to `stdout`. To write to a file one have to pass file path with
//! `timber::init(path)`.
//!
//! Example wrapper for `timber` could look like:
//!
//! ```
//! #[macro_use(timber)]
//! use timber;
//!
//! #[cfg(debug)]
//! pub mod level {
//!     pub const ERR: i32 = 1;
//!     pub const DEB: i32 = 2;
//!     pub const INF: i32 = 7;
//! }
//!
//! #[cfg(not(debug))]
//! pub mod level {
//!     pub const ERR: i32 = 1;
//!     pub const DEB: i32 = 0;
//!     pub const INF: i32 = 3;
//! }
//!
//! macro_rules! log_err{($($arg:tt)*) => {timber!($crate::level::ERR, "ERR", $($arg)*)}}
//! macro_rules! log_deb{($($arg:tt)*) => {timber!($crate::level::DEB, "DEB", $($arg)*)}}
//! macro_rules! log_inf{($($arg:tt)*) => {timber!($crate::level::INF, "INF", $($arg)*)}}
//!
//! //log_err!("This is error! I'm visible!");
//! //log_deb!("I'm debug. I'm visible only in debug mode.");
//! ```

// -------------------------------------------------------------------------------------------------

extern crate time;

use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Once, ONCE_INIT};
use std::io::Write;

// -------------------------------------------------------------------------------------------------

/// Prints timber (processed log). Timber prints time (with microseconds), name of current thread,
/// line number and module name + log text.
#[macro_export]
macro_rules! timber {
    ($lnum:expr, $lname:expr, $($arg:tt)*) => {
        if $lnum > 0 {
            $crate::timber($lname, module_path!(), line!(), format_args!($($arg)*))
        }
    };
}

// -------------------------------------------------------------------------------------------------

/// Timber struct - used as singleton.
pub struct Timber {
    log_file: Option<std::fs::File>,
}

// -------------------------------------------------------------------------------------------------

/// Wrapper for `Timber` struct ensuring thread safety.
struct Wrapper {
    inner: Arc<Mutex<Timber>>,
}

// -------------------------------------------------------------------------------------------------

impl Timber {
    /// Print not formated log.
    pub fn log(&mut self, args: std::fmt::Arguments) {
        match self.log_file {
            Some(ref mut log_file) => {
                log_file.write(format!("{}", args).as_bytes()).expect("Failed to log!");
            }
            None => {
                print!("{}", args);
            }
        }
    }

    /// Print formated log.
    pub fn timber(&mut self, level: &str, module: &str, line: u32, args: std::fmt::Arguments) {
        // Get local time
        let tm = time::now().to_local();

        // Get current thread name
        let current_thread = std::thread::current();
        let thread = current_thread.name().unwrap_or("<unknown>");

        // Format log entry
        let entry = format!("{:02}:{:02}:{:02}.{:06} | {} | {:16} | {:4} | {:40} | {}",
                            tm.tm_hour,
                            tm.tm_min,
                            tm.tm_sec,
                            tm.tm_nsec / 1000,
                            level,
                            thread,
                            line,
                            module,
                            args);

        // Write log entry
        match self.log_file {
            Some(ref mut log_file) => {
                log_file.write(entry.as_bytes()).expect("Failed to timber!");
                log_file.write("\n".as_bytes()).expect("Failed to timber!");
            }
            None => {
                println!("{}", entry);
            }
        }
    }

    /// Initialize logger by providing output log file. Before call to this method logs will be
    /// printed to standard output.
    pub fn init(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        self.log_file = Some(std::fs::File::create(path)?);
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Get instance of logger singleton.
fn get_instance() -> &'static Wrapper {
    static mut LOGGER: *const Wrapper = 0 as *const Wrapper;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let logger = Wrapper { inner: Arc::new(Mutex::new(Timber { log_file: None })) };

            LOGGER = std::mem::transmute(Box::new(logger));
        });

        &(*LOGGER)
    }
}

// -------------------------------------------------------------------------------------------------

/// Get locked instance of `Timber` for guarded loging.
pub fn lock<'a>() -> Result<MutexGuard<'a, Timber>, PoisonError<MutexGuard<'a, Timber>>> {
    get_instance().inner.lock()
}

// -------------------------------------------------------------------------------------------------

/// Print formated log.
pub fn timber(level: &str, module: &str, line: u32, args: std::fmt::Arguments) {
    let mut timber = get_instance().inner.lock().unwrap();
    timber.timber(level, module, line, args);
}

// -------------------------------------------------------------------------------------------------

/// Initialize logger by providing output log file. Before call to this method logs will be printed
/// to standard output.
pub fn init(path: &std::path::Path) -> Result<(), std::io::Error> {
    let mut timber = get_instance().inner.lock().unwrap();
    timber.init(path)
}

// -------------------------------------------------------------------------------------------------
