// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality required for accessing devices.

// -------------------------------------------------------------------------------------------------

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;
use std::os::unix::io;

use nix::{self, Errno};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{Mode, stat};

use qualia::{Illusion, Ipc};

// -------------------------------------------------------------------------------------------------

/// Opener of devices with restricted access.
///
/// If application does not have enough permissions to open device `RestrictedOpener` asks `logind`
/// to open it for him. Communication is done via `dbus`.
///
/// `logind` allows to take device only once, and further tries will fail. `RestrictedOpener`
/// handles IDs of open devices and releases them before next take.
pub struct RestrictedOpener {
    ipc: Ipc,
    taken_devices: RefCell<BTreeSet<u64>>,
}

// -------------------------------------------------------------------------------------------------

impl RestrictedOpener {
    /// Constructs new `RestrictedOpener`.
    pub fn new() -> Self {
        RestrictedOpener {
            ipc: Ipc::new(),
            taken_devices: RefCell::new(BTreeSet::new()),
        }
    }

    /// Tries to open device. If we have insufficient permissions asks `logind` to do it for us.
    pub fn open(&self, path: &Path, oflag: OFlag, mode: Mode) -> Result<io::RawFd, Illusion> {
        match open(path, oflag, mode) {
            Ok(fd) => Ok(fd),
            Err(nix::Error::Sys(errno)) => {
                if (errno == Errno::EPERM) || (errno == Errno::EACCES) {
                    self.take_device(path)
                } else {
                    Err(Illusion::InvalidArgument(errno.desc().to_owned()))
                }
            }
            Err(nix::Error::InvalidPath) => {
                Err(Illusion::InvalidArgument(format!("Path '{:?}' does not exist!", path)))
            }
        }
    }

    /// Initialize connection to `logind`.
    pub fn initialize_ipc(&mut self) -> Result<(), Illusion> {
        self.ipc.initialize()
    }
}

// -------------------------------------------------------------------------------------------------

impl RestrictedOpener {
    /// Takes device from `logind` via `dbus`.
    fn take_device(&self, path: &Path) -> Result<io::RawFd, Illusion> {
        match stat(path) {
            Ok(st) => {
                let rdev = st.st_rdev as u64;

                // If device is taken - release it first
                let contains = self.taken_devices.borrow().contains(&rdev);
                if contains {
                    if self.ipc.release_device(rdev).is_ok() {
                        self.taken_devices.borrow_mut().remove(&rdev);
                    }
                }

                // Take the device
                let result = self.ipc.take_device(rdev);
                if result.is_ok() {
                    self.taken_devices.borrow_mut().insert(rdev);
                }
                result
            }
            Err(err) => {
                Err(Illusion::General(format!("Could not stat file '{:?}': {:?}", path, err)))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
