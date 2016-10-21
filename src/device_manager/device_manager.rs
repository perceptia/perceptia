// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Device manager.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use std::os::unix::io;
use nix::{self, Errno};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{Mode, stat};

use qualia::{Context, Illusion, Ipc};

use evdev;
use udev;
use output_collector::OutputCollector;
use input_gateway::InputGateway;
use drivers::InputDriver;

// -------------------------------------------------------------------------------------------------

/// Device Manager manages searching input and output devices and monitoring them.
pub struct DeviceManager<'a> {
    udev: udev::Udev<'a>,
    ipc: Ipc,
    output_collector: OutputCollector,
}

// -------------------------------------------------------------------------------------------------

impl<'a> DeviceManager<'a> {
    /// `DeviceManager` constructor.
    pub fn new(mut context: Context) -> Self {
        let mut mine = DeviceManager {
            udev: udev::Udev::new(),
            ipc: Ipc::new(),
            output_collector: OutputCollector::new(context.get_dispatcher().clone(),
                                                   context.get_signaler().clone()),
        };

        // Initialize IPC
        mine.initialize_ipc();

        // Initialize input devices
        mine.initialize_input_devices(&mut context);

        // Initialize output devices
        mine.initialize_output_devices();

        // Initialize device monitor
        mine.initialize_device_monitor(&mut context);

        mine
    }

    /// Try to open device. If we have insufficient permissions ask `logind` to do it for us.
    fn open_restricted(&self, path: &Path, oflag: OFlag, mode: Mode) -> Result<io::RawFd, Illusion> {
        match open(path, oflag, mode) {
            Ok(fd) => Ok(fd),
            Err(nix::Error::Sys(errno)) => {
                if (errno == Errno::EPERM) || (errno == Errno::EACCES) {
                    match stat(path) {
                        Ok(st) => self.ipc.take_device(st.st_rdev as u64),
                        _ => Err(Illusion::General(format!("Could not stat file '{:?}'", path))),
                    }
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
    fn initialize_ipc(&mut self) {
        match self.ipc.initialize() {
            Ok(_) => (),
            Err(err) => {
                log_warn1!("Failed to initialize IPC ({:?}). \
                           This may cause problems with access to devices.",
                           err);
            }
        }
    }

    /// Iterate over input devices to find usable ones and initialize event handlers for them.
    fn initialize_input_devices(&mut self, context: &mut Context) {
        self.udev.iterate_event_devices(|devnode, devkind, _| {
            let config = context.get_config().get_input_config();
            let gateway = InputGateway::new(config, context.get_signaler().clone());
            let r = evdev::Evdev::initialize_device(devnode,
                                                    devkind,
                                                    config,
                                                    gateway,
                                                    |path, oflag, mode| {
                                                        self.open_restricted(path, oflag, mode)
                                                    });
            match r {
                Ok(driver) => {
                    context.add_event_handler(driver);
                }
                Err(err) => {
                    log_error!("Could not initialize input devices: {}", err);
                }
            }
        });
    }

    /// Find and initialize outputs.
    fn initialize_output_devices(&mut self) {
        let oc = &mut self.output_collector;
        self.udev.iterate_drm_devices(|devnode, _| {
            // FIXME: Can not do:
            // self.output_collector.scan_device(devnode);
            // Is it compiler bug?
            if let Err(err) = oc.scan_device(devnode) {
                log_error!("{}", err);
            }
        });
    }

    /// Initialize device monitoring.
    fn initialize_device_monitor(&mut self, context: &mut Context) {
        match self.udev.start_device_monitor() {
            Ok(device_monitor) => context.add_event_handler(Box::new(device_monitor)),
            Err(err) => log_warn1!("Device Manager: {}", err),
        }
    }
}

// -------------------------------------------------------------------------------------------------
