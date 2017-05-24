// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic of setting up and tearing down application environment.

// -------------------------------------------------------------------------------------------------

use std;
use std::fs;
use std::time::{Duration, SystemTime};
use std::io::Read;
use std::ops::BitAnd;
use std::error::Error;
use std::default::Default;
use std::path::{Path, PathBuf};

use libc;
use time;
use nix::sys::signal;
use yaml_rust;

use timber;

use errors::Illusion;
use config;
use log;

// -------------------------------------------------------------------------------------------------

const RUNTIME_DIR_VAR: &'static str = "XDG_RUNTIME_DIR";
const DATA_DIR_VAR: &'static str = "XDG_DATA_HOME";
const CACHE_DIR_VAR: &'static str = "XDG_CACHE_HOME";
const CONFIG_DIR_VAR: &'static str = "XDG_CONFIG_HOME";

const DEFAULT_RUNTIME_DIR: &'static str = "/tmp";
const DEFAULT_GLOBAL_CONFIG_DIR: &'static str = "/etc/perceptia";

const DEFAULT_DATA_DIR_FRAGMENT: &'static str = ".local/share";
const DEFAULT_CACHE_DIR_FRAGMENT: &'static str = ".cache";
const DEFAULT_CONFIG_DIR_FRAGMENT: &'static str = ".config";

// -------------------------------------------------------------------------------------------------

pub enum LogDestination {
    StdOut,
    LogFile,
    Disabled,
}

// -------------------------------------------------------------------------------------------------

pub enum Directory {
    Data,
    Cache,
    Runtime,
}

// -------------------------------------------------------------------------------------------------

// FIXME: Do not keep log in runtime directory, as it is removed at exit.
pub struct Env {
    runtime_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    local_config_dir: Option<PathBuf>,
    global_config_dir: Option<PathBuf>,
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
        // Register signals
        Self::register_signal_handler();

        // Create cache directory and initialize logger
        let cache_dir = Self::create_cache_dir().unwrap();
        if let Err(err) = Self::initialize_logger(log_destination, &cache_dir) {
            log_warn1!("{}", err);
        }

        // Create runtime directory
        let data_dir = Self::create_data_dir().unwrap();

        // Create runtime directory
        let runtime_dir = Self::create_runtime_dir().unwrap();

        // Check if configuration directories exist and remember them if so.
        let (global_config_dir, local_config_dir) = Self::check_config_dirs();

        // Construct `Env`
        let mine = Env {
            runtime_dir: runtime_dir,
            data_dir: data_dir,
            cache_dir: cache_dir,
            local_config_dir: local_config_dir,
            global_config_dir: global_config_dir,
        };

        // Remove unneeded files
        mine.remove_old_logs();
        mine
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
            match dir {
                Directory::Data => self.data_dir.clone(),
                Directory::Cache => self.cache_dir.clone(),
                Directory::Runtime => self.runtime_dir.clone(),
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

// Initializing logger
impl Env {
    /// Initializes logger to write log to given destination.
    fn initialize_logger(destination: LogDestination, dir: &Path) -> Result<(), Illusion> {
        match destination {
            LogDestination::LogFile => Self::initialize_logger_for_log_file(dir),
            LogDestination::StdOut => Self::initialize_logger_for_stdout(),
            LogDestination::Disabled => Self::disable_logger(),
        }
    }

    /// Chose log file path and sets logger up to use it.
    fn initialize_logger_for_log_file(dir: &Path) -> Result<(), Illusion> {
        let path = dir.join(format!("log-{}.log", Self::get_time_representation()));
        match timber::init(&path) {
            Ok(ok) => {
                println!("Welcome to perceptia");
                println!("Log file in {:?}", path);
                Ok(ok)
            }
            Err(err) => Err(Illusion::General(err.description().to_owned())),
        }
    }

    /// Sets logger to write logs to `/dev/null`.
    fn disable_logger() -> Result<(), Illusion> {
        if let Err(err) = timber::init(Path::new("/dev/null")) {
            Err(Illusion::General(format!("Failed to disable logger: {}", err)))
        } else {
            Ok(())
        }
    }

    /// Sets logger to write logs to `stdout`.
    fn initialize_logger_for_stdout() -> Result<(), Illusion> {
        // Nothing to do. `timber` prints to `stdout` by default.
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Cleaning up
impl Env {
    /// Cleans up environment: remove runtime directory.
    fn cleanup(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.runtime_dir) {
            log_warn1!("Failed to remove runtime directory: {:?}", err);
        }
    }

    /// Removes logs older than two days.
    fn remove_old_logs(&self) {
        let transition_time = SystemTime::now() - Duration::new(2 * 24 * 60 * 60, 0);
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "log" {
                            // Check if file is old enough to be removed. In case of any error
                            // remove the file.
                            let meta = path.metadata();
                            let good_to_remove = {
                                if let Ok(meta) = meta {
                                    if let Ok(access_time) = meta.accessed() {
                                        access_time < transition_time
                                    } else {
                                        true
                                    }
                                } else {
                                    true
                                }
                            };

                            if good_to_remove {
                                if let Err(err) = fs::remove_file(&path) {
                                    log_warn1!("Failed to remove old log file {:?}: {}", path, err);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Creating directories
impl Env {
    /// Create runtime directory.
    fn create_runtime_dir() -> Result<PathBuf, Illusion> {
        let mut path = Self::read_path(RUNTIME_DIR_VAR, DEFAULT_RUNTIME_DIR);
        path.push(format!("perceptia-{}", Self::get_time_representation()));
        Self::mkdir(&path).and(Ok(path))
    }

    /// Create data directory.
    fn create_data_dir() -> Result<PathBuf, Illusion> {
        let mut default_path = std::env::home_dir().unwrap();
        default_path.push(DEFAULT_DATA_DIR_FRAGMENT);
        let mut path = Self::read_path(DATA_DIR_VAR, default_path.to_str().unwrap());
        path.push("perceptia");
        Self::mkdir(&path).and(Ok(path))
    }

    /// Create cache directory.
    fn create_cache_dir() -> Result<PathBuf, Illusion> {
        let mut default_path = std::env::home_dir().unwrap();
        default_path.push(DEFAULT_CACHE_DIR_FRAGMENT);
        let mut path = Self::read_path(CACHE_DIR_VAR, default_path.to_str().unwrap());
        path.push("perceptia");
        Self::mkdir(&path).and(Ok(path))
    }

    /// Checks if config directories exist.
    ///
    /// Global config directory is `/etc/perceptia/`.
    ///
    /// Local config directory is `$XDG_CONFIG_HOME/perceptia` if the variable exists, else
    /// `~/.config/perceptia`.
    fn check_config_dirs() -> (Option<PathBuf>, Option<PathBuf>) {
        // Check if local config directory exists
        let local_config_dir = {
            let mut default_path = std::env::home_dir().unwrap();
            default_path.push(DEFAULT_CONFIG_DIR_FRAGMENT);
            let mut local = Self::read_path(CONFIG_DIR_VAR, default_path.to_str().unwrap());
            local.push("perceptia");
            if local.exists() && local.is_dir() {
                Some(local)
            } else {
                None
            }
        };

        // Check if global config directory exists
        let global_config_dir = {
            let global = PathBuf::from(DEFAULT_GLOBAL_CONFIG_DIR);
            if global.exists() && global.is_dir() {
                Some(global)
            } else {
                None
            }
        };

        // Return results
        (global_config_dir, local_config_dir)
    }

    /// Reads given environment variable and if exists returns its value or default value otherwise.
    fn read_path(var: &str, default_path: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(std::env::var(var).unwrap_or(default_path.to_string()));
        path
    }

    /// Helper function for creating directory.
    fn mkdir(path: &PathBuf) -> Result<(), Illusion> {
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

// Handling signals.
impl Env {
    /// Registers handler for signals `SIGINT`, `SIGTERM`, `SIGSEGV` and `SIGABRT`. Panics if
    /// something goes wrong.
    fn register_signal_handler() {
        let flags = signal::SaFlags::empty().bitand(signal::SA_SIGINFO);
        let handler = signal::SigHandler::Handler(Self::signal_handler);
        let sa = signal::SigAction::new(handler, flags, signal::SigSet::empty());

        unsafe {
            signal::sigaction(signal::SIGINT, &sa).unwrap();
            signal::sigaction(signal::SIGTERM, &sa).unwrap();
            signal::sigaction(signal::SIGSEGV, &sa).unwrap();
            signal::sigaction(signal::SIGABRT, &sa).unwrap();
        }
    }

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
}

// -------------------------------------------------------------------------------------------------

impl Drop for Env {
    fn drop(&mut self) {
        self.cleanup();
        log_info1!("Thank you for running perceptia! Bye!");
    }
}

// -------------------------------------------------------------------------------------------------
