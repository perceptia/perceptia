// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic of setting up and tearing down application environment.

// -------------------------------------------------------------------------------------------------

use std;
use libc;
use time;
use std::error::Error;
use nix::sys::signal;
use std::ops::BitAnd;

use errors;
use timber;

use dharma;

// -------------------------------------------------------------------------------------------------

const DATA_DIR_VAR: &'static str = "XDG_DATA_HOME";
const RUNTIME_DIR_VAR: &'static str = "XDG_RUNTIME_DIR";

const DEFAULT_DATA_DIR: &'static str = "/tmp/perceptia";
const DEFAULT_RUNTIME_DIR: &'static str = "/tmp";

// -------------------------------------------------------------------------------------------------

pub struct Env {
    data_dir: Option<std::path::PathBuf>,
    runtime_dir: Option<std::path::PathBuf>,
}

// -------------------------------------------------------------------------------------------------

/// This class represents runtime environment. It cares for creating directories or initializing
/// logger.
impl Env {
    /// Prepare environment:
    ///  - register signal handler
    ///  - create needed directories
    ///  - initialize logger
    ///  - clean old files
    pub fn create() -> Self {
        let mut mine = Env {
            data_dir: None,
            runtime_dir: None,
        };

        // Register signals
        mine.register_signal_handler();

        // Create data directory and initialize logger
        if let Err(err) = mine.create_data_dir() {
            log_warn1!("Failed to create data directory: {}", err);
        } else if let Err(err) = mine.initialize_logger() {
            log_warn1!("{}", err);
        }

        // Create runtime directory
        if let Err(err) = mine.create_runtime_dir() {
            log_warn1!("Failed to create runtime directory: {}", err);
        }

        // Remove unneeded files
        Self::remove_old_logs();

        mine
    }

    /// Clean up environment: remove runtime directory.
    fn cleanup(&mut self) {
        if let Some(ref path) = self.runtime_dir {
            std::fs::remove_dir_all(path);
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Helper methods associated with `Env`.
impl Env {
    /// Registers handler for signals `SIGINT`, `SIGTERM`, `SIGSEGV` and `SIGABRT`. Panics if
    /// something goes wrong.
    fn register_signal_handler(&self) {
        let flags = signal::SaFlags::empty().bitand(signal::SA_SIGINFO);
        let handler = signal::SigHandler::Handler(signal_handler);
        let sa = signal::SigAction::new(handler, flags, signal::SigSet::empty());

        unsafe {
            signal::sigaction(signal::SIGINT, &sa).unwrap();
            signal::sigaction(signal::SIGTERM, &sa).unwrap();
            signal::sigaction(signal::SIGSEGV, &sa).unwrap();
            signal::sigaction(signal::SIGABRT, &sa).unwrap();
        }
    }

    /// Create data directory.
    fn create_data_dir(&mut self) -> Result<(), errors::Error> {
        let path = Self::read_path(DATA_DIR_VAR, DEFAULT_DATA_DIR);
        let result = Self::mkdir(&path);
        if result.is_ok() {
            self.data_dir = Some(path);
        }
        result
    }

    /// Create runtime directory.
    fn create_runtime_dir(&mut self) -> Result<(), errors::Error> {
        let path = Self::read_path(RUNTIME_DIR_VAR, DEFAULT_RUNTIME_DIR);
        let path = path.join(format!("perceptia-{}", Self::get_time_representation()));
        let result = Self::mkdir(&path);
        if result.is_ok() {
            self.runtime_dir = Some(path);
        }
        result
    }

    /// Chose log file path and initialize logger.
    fn initialize_logger(&mut self) -> Result<(), errors::Error> {
        if let Some(ref data_dir) = self.data_dir {
            let path = data_dir.join(format!("log-{}", Self::get_time_representation()));
            match timber::init(&path) {
                Ok(ok) => {
                    println!("Welcome to perceptia");
                    println!("Log file in {:?}", path);
                    Ok(ok)
                }
                Err(err) => {
                    Err(errors::Error::General(err.description().to_owned()))
                }
            }
        } else {
            let text = "Could not create log file! Data directory not available!".to_owned();
            Err(errors::Error::General(text))
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Static functions associated with `Env`.
impl Env {
    /// Reads given environment variable and if exists returns its value or default value otherwise.
    fn read_path(var: &str, default_path: &str) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::new();
        path.push(if let Ok(p) = std::env::var(var) { p } else { default_path.to_owned() });
        path
    }

    /// Helper function for creating directory.
    fn mkdir(path: &std::path::PathBuf) -> Result<(), errors::Error> {
        if path.exists() {
            if path.is_dir() {
                return Ok(());
            } else {
                Err(errors::Error::InvalidArgument(format!("Path '{:?}' is not directory!", path)))
            }
        } else if let Err(err) = std::fs::create_dir(path) {
            Err(errors::Error::General(format!("Could not create directory '{:?}': {}", path, err)))
        } else {
            Ok(())
        }
    }


    /// Removes logs older than one day.
    fn remove_old_logs() {
        // FIXME: Implement removing old log files.
    }

    /// Helper function for generating temporary director and file names. Returns string in format
    /// `ddd-hh-mm-ss`, where
    ///  - `ddd` is zero padded number of current day in year
    ///  - `hh` is zero padded hour
    ///  - `mm` is zero padded minute
    ///  - `ss` is zero padded second
    fn get_time_representation() -> String {
        let tm = time::now().to_local();
        format!("{:03}-{:02}-{:02}-{:02}", tm.tm_yday, tm.tm_hour, tm.tm_min, tm.tm_sec)
    }
}

// -------------------------------------------------------------------------------------------------

impl Drop for Env {
    fn drop(&mut self) {
        self.cleanup();
        log_info1!("Thank you for running perceptia! Bye!");
    }
}

// -------------------------------------------------------------------------------------------------

/// System signal handler. Handle `SIGINT`, `SIGTERM`, `SIGSEGV` and `SIGABRT` by exiting.
extern fn signal_handler(signum: libc::c_int) {
    if (signum == signal::SIGINT)
    || (signum == signal::SIGTERM)
    || (signum == signal::SIGSEGV)
    || (signum == signal::SIGABRT) {
        log_info1!("Signal {} received asynchronously: exit", signum);
        panic!("Received terminating signal");
    } else {
        log_info2!("Signal {} received asynchronously: ignore", signum);
    }
}

// -------------------------------------------------------------------------------------------------
