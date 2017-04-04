// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Device manager.

// -------------------------------------------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

use dharma;
use qualia::Context;

use evdev;
use udev;
use device_access::RestrictedOpener;
use output_collector::OutputCollector;
use input_gateway::InputGateway;
use drivers::InputDriver;
use virtual_terminal;

// -------------------------------------------------------------------------------------------------

/// Device Manager manages searching input and output devices and monitoring them.
pub struct DeviceManager<'a> {
    udev: udev::Udev<'a>,
    restricted_opener: Rc<RefCell<RestrictedOpener>>,
    output_collector: OutputCollector,
    context: Context,
}

// -------------------------------------------------------------------------------------------------

impl<'a> DeviceManager<'a> {
    /// `DeviceManager` constructor.
    pub fn new(mut context: Context) -> Self {
        let restricted_opener = Self::prepare_restricted_opener();

        let mut mine = DeviceManager {
            udev: udev::Udev::new(),
            restricted_opener: restricted_opener,
            output_collector: OutputCollector::new(context.get_dispatcher().clone(),
                                                   context.get_signaler().clone()),
            context: context.clone(),
        };

        // Setup virtual terminal
        mine.setup_virtual_terminal(&mut context);

        // Initialize input devices
        mine.initialize_input_devices(&mut context);

        // Initialize output devices
        mine.initialize_output_devices();

        // Initialize device monitor
        mine.initialize_device_monitor(&mut context);

        mine
    }

    /// Prepares device opener.
    fn prepare_restricted_opener() -> Rc<RefCell<RestrictedOpener>> {
        let mut restricted_opener = RestrictedOpener::new();
        if let Err(err) = restricted_opener.initialize_ipc() {
            log_warn1!("Failed to initialize IPC ({:?}). \
                        This may cause problems with access to devices.",
                        err);
        }
        Rc::new(RefCell::new(restricted_opener))
    }

    /// Sets up virtual terminal.
    fn setup_virtual_terminal(&self, context: &mut Context) {
        if let Err(err) = virtual_terminal::setup(context.get_dispatcher().clone(),
                                                  context.get_signaler().clone(),
                                                  &self.restricted_opener.borrow()) {
            log_error!("Device Manager: {:?}", err);
        }
    }

    /// Iterate over input devices to find usable ones and initialize event handlers for them.
    fn initialize_input_devices(&mut self, context: &mut Context) {
        self.udev.iterate_event_devices(|devnode, devkind, _| {
            let config = context.get_config().get_input_config();
            let gateway = InputGateway::new(config,
                                            context.get_input_manager().clone(),
                                            context.get_signaler().clone());
            let r = evdev::Evdev::initialize_device(devnode,
                                                    devkind,
                                                    config,
                                                    gateway,
                                                    &self.restricted_opener.borrow());
            match r {
                Ok(driver) => {
                    context.add_event_handler(driver, dharma::event_kind::READ);
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
            Ok(device_monitor) => {
                context.add_event_handler(Box::new(device_monitor), dharma::event_kind::READ);
            }
            Err(err) => {
                log_warn1!("Device Manager: {}", err);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Event handlers
impl<'a> DeviceManager<'a> {
    pub fn on_suspend(&mut self) {
        // Nothing to do as for now...
    }

    pub fn on_wakeup(&mut self) {
        // Old event devices hung-up so devices must be reinitialized.
        let mut context = self.context.clone();
        self.initialize_input_devices(&mut context);
    }
}

// -------------------------------------------------------------------------------------------------
