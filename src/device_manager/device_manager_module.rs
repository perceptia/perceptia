// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Device Manager.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use std::os::unix::io;
use nix::{self, Errno};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{Mode, stat};

use dharma::{InitResult, Module};
use qualia::{Context, Error, Ipc, Perceptron};

use evdev;
use udev;
use drivers::InputDriver;

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModule {
    udev: udev::Udev,
    ipc: Ipc,
}

// -------------------------------------------------------------------------------------------------

impl DeviceManagerModule {
    /// `DeviceManagerModule` constructor.
    pub fn new() -> Self {
        DeviceManagerModule {
            udev: udev::Udev::new(),
            ipc: Ipc::new(),
        }
    }

    /// Try to open device. If we have insufficient permissions ask `logind` to do it for us.
    fn open_restricted(&self, path: &Path, oflag: OFlag, mode: Mode) -> Result<io::RawFd, Error> {
        match open(path, oflag, mode) {
            Ok(fd) => Ok(fd),
            Err(nix::Error::Sys(errno)) => {
                if (errno == Errno::EPERM) || (errno == Errno::EACCES) {
                    match stat(path) {
                        Ok(st) => self.ipc.take_device(st.st_rdev as u64),
                        _ => Err(Error::General(format!("Could not stat file '{:?}'", path))),
                    }
                } else {
                    Err(Error::InvalidArgument(errno.desc().to_owned()))
                }
            }
            Err(nix::Error::InvalidPath) => {
                Err(Error::InvalidArgument(format!("Path '{:?}' does not exist!", path)))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for DeviceManagerModule {
    type T = Perceptron;
    type C = Context;

    #[allow(unused_variables)]
    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        // Initialize IPC
        match self.ipc.initialize() {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to initialize IPC ({:?}). This may cause problems with access to \
                          devices.",
                         err)
            }
        }

        // Initialize input devices
        self.udev.iterate_event_devices(|devnode| {
            let r = evdev::Evdev::initialize_device(devnode, |path, oflag, mode| {
                self.open_restricted(path, oflag, mode)
            });
            match r {
                Ok(driver) => {
                    context.add_event_handler(driver);
                }
                Err(err) => {
                    println!("Could not initialize {:?}", err);
                }
            }
        });

        Vec::new()
    }

    fn execute(&mut self, package: &Self::T) {}

    fn finalize(&mut self) {}
}

// -------------------------------------------------------------------------------------------------
