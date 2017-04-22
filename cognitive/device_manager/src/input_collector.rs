// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Handling plugging-in and out input devices.

// -------------------------------------------------------------------------------------------------

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use dharma::{EventHandlerId, event_kind};
use qualia::{Illusion, DeviceKind, EventHandling, InputConfig, InputForwarding, InputHandling};

use evdev_driver;
use udev::Udev;
use input_gateway::InputGateway;
use drivers::InputDriver;
use device_access::RestrictedOpener;
use virtual_terminal;

// -------------------------------------------------------------------------------------------------

/// Information identifying input device.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct DeviceInfo {
    pub devnode: PathBuf,
    pub device_kind: DeviceKind,
}

// -------------------------------------------------------------------------------------------------

impl DeviceInfo {
    /// Constructs new `DeviceInfo`.
    fn new(devnode: &Path, device_kind: DeviceKind) -> Self {
        DeviceInfo {
            devnode: devnode.to_owned(),
            device_kind: device_kind,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// `InputCollector` manages plugging-in and out input devices.
pub struct InputCollector<C> where C: EventHandling {
    coordinator: C,
    input_handler: Box<InputHandling>,
    input_forwarder: Box<InputForwarding>,
    input_config: InputConfig,
    vt: Option<virtual_terminal::VirtualTerminal>,
    restricted_opener: Rc<RefCell<RestrictedOpener>>,
    current_devices: HashMap<DeviceInfo, EventHandlerId>,
}

// -------------------------------------------------------------------------------------------------

impl<C> InputCollector<C> where C: EventHandling {
    /// Constructs new `InputCollector`.
    pub fn new(coordinator: C,
               input_handler: Box<InputHandling>,
               input_forwarder: Box<InputForwarding>,
               input_config: InputConfig,
               vt: Option<virtual_terminal::VirtualTerminal>,
               restricted_opener: Rc<RefCell<RestrictedOpener>>)
               -> Self {
        InputCollector {
            coordinator: coordinator,
            input_handler: input_handler,
            input_forwarder: input_forwarder,
            input_config: input_config,
            restricted_opener: restricted_opener,
            vt: vt,
            current_devices: HashMap::new(),
        }
    }

    /// Updates its inned list of input devices and creates new event handlers for newly found ones
    /// and removes them for lost ones.
    pub fn scan_devices(&mut self, udev: &Udev) -> Result<(), Illusion> {
        let old_devices = self.collect_current_devices();
        let mut new_devices = HashSet::<DeviceInfo>::new();

        udev.iterate_input_devices(|devnode, devkind, _| {
            new_devices.insert(DeviceInfo::new(devnode, devkind));
        });

        for dev in new_devices.difference(&old_devices) {
            self.handle_new_device(dev.clone());
        }

        for dev in old_devices.difference(&new_devices) {
            self.handle_lost_device(dev);
        }

        Ok(())
    }
 }

// -------------------------------------------------------------------------------------------------

impl<C> InputCollector<C> where C: EventHandling {
    /// Handles new device by creating new instance of drive for it and adding new event handler.
    fn handle_new_device(&mut self, device: DeviceInfo) {
        let gateway = InputGateway::new(self.input_handler.duplicate(),
                                        self.input_forwarder.duplicate(),
                                        self.vt.clone());
        let r = evdev_driver::Evdev::initialize_device(&device.devnode,
                                                       device.device_kind,
                                                       self.input_config.clone(),
                                                       gateway,
                                                       &self.restricted_opener.borrow());
        match r {
            Ok(driver) => {
                let id = self.coordinator.add_event_handler(driver, event_kind::READ);
                self.current_devices.insert(device, id);
            }
            Err(err) => {
                log_error!("Could not initialize input devices: {}", err);
            }
        }
    }

    /// Handles lost device by removing its event handler.
    fn handle_lost_device(&mut self, device: &DeviceInfo) {
        log_info1!("Lost {:?}: {:?}", device.device_kind, device.devnode);
        if let Some(id) = self.current_devices.remove(&device) {
            self.coordinator.remove_event_handler(id);
        } else {
            log_warn2!("Lost input device which was never found: {:?}", device);
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Helper methods.
impl<C> InputCollector<C> where C: EventHandling {
    /// Converts inner collection of devices to set.
    fn collect_current_devices(&self) -> HashSet<DeviceInfo> {
        let mut set = HashSet::new();
        for key in self.current_devices.keys() {
            set.insert(key.clone());
        }
        set
    }
}

// -------------------------------------------------------------------------------------------------
