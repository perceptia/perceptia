// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic of setting up and tearing down application environment.

// -------------------------------------------------------------------------------------------------

use std::{self, fs};
use std::io::Read;
use std::ops::BitAnd;
use std::error::Error;
use std::default::Default;
use libc;
use time;
use nix::sys::signal;
use yaml_rust;

use timber;

use errors::Illusion;
use config;
use log;

// -------------------------------------------------------------------------------------------------

const DATA_DIR_VAR: &'static str = "XDG_DATA_HOME";
const RUNTIME_DIR_VAR: &'static str = "XDG_RUNTIME_DIR";
const CONFIG_DIR_VAR: &'static str = "XDG_CONFIG_HOME";

const DEFAULT_DATA_DIR: &'static str = "/tmp/perceptia";
const DEFAULT_RUNTIME_DIR: &'static str = "/tmp";
const DEFAULT_GLOBAL_CONFIG_DIR: &'static str = "/etc/perceptia";

// -------------------------------------------------------------------------------------------------

pub enum LogDestination {
    StdOut,
    LogFile,
    Disabled,
}

// -------------------------------------------------------------------------------------------------

pub enum Directory {
    Data,
    Runtime,
}

// -------------------------------------------------------------------------------------------------

// TODO: Directories should not be optional.
// FIXME: Do not keep log in runtime directory, as it is removed at exit.
pub struct Env {
    data_dir: Option<std::path::PathBuf>,
    runtime_dir: Option<std::path::PathBuf>,
    local_config_dir: Option<std::path::PathBuf>,
    global_config_dir: Option<std::path::PathBuf>,
}

// -------------------------------------------------------------------------------------------------

/// This class represents runtime environment. It cares for creating directories or initializing
/// logger.
impl Env {
    /// Prepares environment:
    ///  - register signal handler
    ///  - create needed directories
    ///  - initialize logger
    ///  - clean old files
    pub fn create(log_destination: LogDestination) -> Self {
        let mut mine = Env {
            data_dir: None,
            runtime_dir: None,
            local_config_dir: None,
            global_config_dir: None,
        };

        // Register signals
        mine.register_signal_handler();

        // Create data directory and initialize logger
        if let Err(err) = mine.create_data_dir() {
            log_warn1!("Failed to create data directory: {}", err);
        } else if let Err(err) = mine.initialize_logger(log_destination) {
            log_warn1!("{}", err);
        }

        // Create runtime directory
        if let Err(err) = mine.create_runtime_dir() {
            log_warn1!("Failed to create runtime directory: {}", err);
        }

        // Check if configuration directories exist and remember them if so.
        mine.check_config_dirs();

        // Remove unneeded files
        Self::remove_old_logs();

        mine
    }

    /// Cleans up environment: remove runtime directory.
    fn cleanup(&mut self) {
        if let Some(ref path) = self.runtime_dir {
            if let Err(err) = fs::remove_dir_all(path) {
                log_warn1!("Failed to remove runtime directory: {:?}", err);
            }
        }
    }

    /// Loads configuration.
    ///
    /// TODO: Configuration is currently read only from `perceptia.conf` in config directories.
    /// Make all files with extension `.conf` or `.yaml` be threated as config files.
    ///
    /// TODO: Keep reading files even if parsing one fails.
    pub fn load_config(&self) -> Result<config::Config, Illusion> {
        let mut config = config::Config::default();

        for dir in vec![self.global_config_dir.clone(), self.local_config_dir.clone()] {
            if let Some(mut path) = dir {
                path.push("perceptia.conf");
                if let Ok(mut file) = fs::File::open(&path) {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    match yaml_rust::YamlLoader::load_from_str(&contents) {
                        Ok(yaml) => config.load(&yaml),
                        Err(err) => {
                            return Err(Illusion::Config(path, err.description().to_owned()));
                        }
                    }
                }
            }
        }

        Ok(config)
    }

    /// Reads in configuration. If loading fails returns default configuration.
    pub fn read_config(&self) -> config::Config {
        match self.load_config() {
            Ok(config) => config,
            Err(err) => {
                log_error!("Config error: {}", err);
                config::Config::default()
            }
        }
    }

    /// Opens file in predefined directory.
    pub fn open_file(&self, name: String, dir: Directory) -> Result<fs::File, Illusion> {
        let mut dir = {
            let dir = {
                match dir {
                    Directory::Data => self.data_dir.clone(),
                    Directory::Runtime => self.runtime_dir.clone(),
                }
            };

            if let Some(dir) = dir {
                dir
            } else {
                return Err(Illusion::General(format!("Requested directory is not available")));
            }
        };

        dir.set_file_name(name);
        match fs::OpenOptions::new().read(true).write(true).create(true).open(dir.as_path()) {
            Ok(file) => Ok(file),
            Err(err) => Err(Illusion::IO(err.description().to_string())),
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
    fn create_data_dir(&mut self) -> Result<(), Illusion> {
        let path = Self::read_path(DATA_DIR_VAR, DEFAULT_DATA_DIR);
        let result = Self::mkdir(&path);
        if result.is_ok() {
            self.data_dir = Some(path);
        }
        result
    }

    /// Create runtime directory.
    fn create_runtime_dir(&mut self) -> Result<(), Illusion> {
        let path = Self::read_path(RUNTIME_DIR_VAR, DEFAULT_RUNTIME_DIR);
        let path = path.join(format!("perceptia-{}", Self::get_time_representation()));
        let result = Self::mkdir(&path);
        if result.is_ok() {
            self.runtime_dir = Some(path);
        }
        result
    }

    /// Check if config files exist is store paths.
    ///
    /// Global config directory is `/etc/perceptia/`.
    ///
    /// Local config directory is `$XDG_CONFIG_HOME/perceptia` if the variable exists, else
    /// `~/.config/perceptia`.
    fn check_config_dirs(&mut self) {
        if let Some(mut home_dir) = std::env::home_dir() {
            home_dir.push(".config");
            let mut local = Self::read_path(CONFIG_DIR_VAR, home_dir.to_str().unwrap());
            local.push("perceptia");
            if local.exists() && local.is_dir() {
                self.local_config_dir = Some(local);
            }
        }

        let global = std::path::PathBuf::from(DEFAULT_GLOBAL_CONFIG_DIR);
        if global.exists() && global.is_dir() {
            self.global_config_dir = Some(global);
        }
    }

    /// Initializes logger to write log to given destination.
    fn initialize_logger(&mut self, destination: LogDestination) -> Result<(), Illusion> {
        match destination {
            LogDestination::LogFile => self.initialize_logger_for_log_file(),
            LogDestination::Disabled => self.disable_logger(),
            // Nothing to do. `timber` prints to `stdout` by default.
            LogDestination::StdOut => Ok(()),
        }
    }

    /// Chose log file path and sets logger up to use it.
    fn initialize_logger_for_log_file(&mut self) -> Result<(), Illusion> {
        if let Some(ref data_dir) = self.data_dir {
            let path = data_dir.join(format!("log-{}", Self::get_time_representation()));
            match timber::init(&path) {
                Ok(ok) => {
                    println!("Welcome to perceptia");
                    println!("Log file in {:?}", path);
                    Ok(ok)
                }
                Err(err) => Err(Illusion::General(err.description().to_owned())),
            }
        } else {
            let text = "Could not create log file! Data directory not available!".to_owned();
            Err(Illusion::General(text))
        }
    }

    /// Sets logger to write logs to `/dev/null`.
    fn disable_logger(&mut self) -> Result<(), Illusion> {
        if let Err(err) = timber::init(std::path::Path::new("/dev/null")) {
            Err(Illusion::General(format!("Failed to disable logger: {}", err)))
        } else {
            Ok(())
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Static functions associated with `Env`.
impl Env {
    /// Reads given environment variable and if exists returns its value or default value otherwise.
    fn read_path(var: &str, default_path: &str) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::new();
        path.push(std::env::var(var).unwrap_or(default_path.to_owned()));
        path
    }

    /// Helper function for creating directory.
    fn mkdir(path: &std::path::PathBuf) -> Result<(), Illusion> {
        if path.exists() {
            if path.is_dir() {
                Ok(())
            } else {
                Err(Illusion::InvalidArgument(format!("Path '{:?}' is not directory!", path)))
            }
        } else if let Err(err) = fs::create_dir(path) {
            Err(Illusion::General(format!("Could not create directory '{:?}': {}", path, err)))
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
    ///
    /// - `ddd` is zero padded number of current day in year
    /// - `hh` is zero padded hour
    /// - `mm` is zero padded minute
    /// - `ss` is zero padded second
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

/// System signal handler.
///
/// Normally `SIGINT` and `SIGTERM` signals should be blocked and be handled by `Dispatcher` and
/// this function should be only able to catch these signals after `Dispatcher` exited.
///
/// `SIGSEGV` and `SIGABRT` are handler by exiting.
#[cfg_attr(rustfmt, rustfmt_skip)]
extern fn signal_handler(signum: libc::c_int) {
    if (signum == signal::SIGSEGV as libc::c_int)
    || (signum == signal::SIGABRT as libc::c_int) {
        log_info1!("Signal {} received asynchronously", signum);
        log::backtrace();
        std::process::exit(1);
    } else if (signum == signal::SIGINT as libc::c_int)
    || (signum == signal::SIGTERM as libc::c_int) {
        log_info1!("Signal {} received asynchronously", signum);
        log::backtrace();
    } else {
        log_info2!("Signal {} received asynchronously: ignore", signum);
    }
}

// -------------------------------------------------------------------------------------------------
