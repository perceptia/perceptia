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

pub struct DeviceManagerModule<'a> {
    udev: udev::Udev<'a>,
    ipc: Ipc,
}

// -------------------------------------------------------------------------------------------------

impl<'a> DeviceManagerModule<'a> {
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

    /// TODO
    fn initialize_ipc(&mut self) {
        match self.ipc.initialize() {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to initialize IPC ({:?}). This may cause problems with access to \
                          devices.",
                         err)
            }
        }
    }

    /// TODO
    fn initialize_input_devices(&mut self, context: &mut Context) {
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
    }

    /// TODO
    fn initialize_output_devices(&mut self, context: &mut Context) {
        // FIXME: Finnish implementation of `initialize_output_devices`.
    }

    /// TODO
    fn initialize_device_monitor(&mut self, context: &mut Context) {
        match self.udev.start_device_monitor() {
            Ok(device_monitor) => context.add_event_handler(Box::new(device_monitor)),
            Err(err) => println!("Device Manager: {:?}", err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Module for DeviceManagerModule<'a> {
    type T = Perceptron;
    type C = Context;

    #[allow(unused_variables)]
    fn initialize(&mut self, mut context: Self::C) -> InitResult {
        // Initialize IPC
        self.initialize_ipc();

        // Initialize input devices
        self.initialize_input_devices(&mut context);

        // Initialize output devices
        self.initialize_output_devices(&mut context);

        // Initialize device monitor
        self.initialize_device_monitor(&mut context);

        Vec::new()
    }

    fn execute(&mut self, package: &Self::T) {}

    fn finalize(&mut self) {}
}

// -------------------------------------------------------------------------------------------------
