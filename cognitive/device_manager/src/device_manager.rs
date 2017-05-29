// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Device manager.

// -------------------------------------------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

use dharma;
use qualia::{InputConfig, InputForwarding, InputHandling};
use qualia::{EventHandling, HwGraphics, StatePublishing};

use udev;
use evdev_driver;
use device_access::RestrictedOpener;
use output_collector::OutputCollector;
use input_gateway::InputGateway;
use drivers::InputDriver;
use virtual_terminal;

// -------------------------------------------------------------------------------------------------

/// Device Manager manages searching input and output devices and monitoring them.
pub struct DeviceManager<'a, C>
    where C: EventHandling + StatePublishing + HwGraphics + Clone
{
    udev: udev::Udev<'a>,
    restricted_opener: Rc<RefCell<RestrictedOpener>>,
    output_collector: OutputCollector<C>,
    vt: Option<virtual_terminal::VirtualTerminal>,
    input_handler: Box<InputHandling>,
    input_forwarder: Box<InputForwarding>,
    input_config: InputConfig,
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<'a, C> DeviceManager<'a, C>
    where C: 'static + EventHandling + StatePublishing + HwGraphics + Send + Clone
{
    /// `DeviceManager` constructor.
    pub fn new(input_handler: Box<InputHandling>,
               input_forwarder: Box<InputForwarding>,
               input_config: InputConfig,
               coordinator: C) -> Self {
        let restricted_opener = Self::prepare_restricted_opener();

        let mut mine = DeviceManager {
            udev: udev::Udev::new(),
            restricted_opener: restricted_opener,
            output_collector: OutputCollector::new(coordinator.clone()),
            vt: None,
            input_handler: input_handler,
            input_forwarder: input_forwarder,
            input_config: input_config.clone(),
            coordinator: coordinator.clone(),
        };

        // Setup virtual terminal
        mine.setup_virtual_terminal();

        // Initialize input devices
        mine.initialize_input_devices();

        // Initialize output devices
        mine.initialize_output_devices();

        // Initialize device monitor
        mine.initialize_device_monitor();

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
    fn setup_virtual_terminal(&mut self) {
        match virtual_terminal::VirtualTerminal::new(&self.restricted_opener.borrow()) {
            Ok(vt) => {
                self.vt = Some(vt);
                match vt.get_current() {
                    Ok(tty_num) => {
                        match virtual_terminal::setup(tty_num,
                                                      self.coordinator.clone(),
                                                      &self.restricted_opener.borrow()) {
                            Ok(_) => {}
                            Err(err) => log_error!("Device Manager: {:?}", err),
                        }
                    }
                    Err(err) => {
                        log_error!("Device Manager: {:?}", err);
                    }
                }
            }
            Err(err) => {
                log_error!("Failed to open virtual terminal: {:?}", err);
            }
        }
    }

    /// Iterate over input devices to find usable ones and initialize event handlers for them.
    fn initialize_input_devices(&mut self) {
        let input_handler = self.input_handler.duplicate();
        let input_forwarder = self.input_forwarder.duplicate();
        let input_config = self.input_config.clone();
        let restricted_opener = self.restricted_opener.clone();
        let mut coordinator = self.coordinator.clone();
        let vt = self.vt.clone();

        self.udev.iterate_event_devices(|devnode, devkind, _| {
            let gateway = InputGateway::new(input_handler.duplicate(),
                                            input_forwarder.duplicate(),
                                            vt.clone());
            let r = evdev_driver::Evdev::initialize_device(devnode,
                                                           devkind,
                                                           input_config.clone(),
                                                           gateway,
                                                           &restricted_opener.borrow());
            match r {
                Ok(driver) => {
                    coordinator.add_event_handler(driver, dharma::event_kind::READ);
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
            if let Err(err) = oc.scan_device(devnode) {
                log_error!("{}", err);
            }
        });
    }

    /// Initialize device monitoring.
    fn initialize_device_monitor(&mut self) {
        match self.udev.start_device_monitor() {
            Ok(device_monitor) => {
                self.coordinator.add_event_handler(Box::new(device_monitor),
                                                   dharma::event_kind::READ);
            }
            Err(err) => {
                log_warn1!("Device Manager: {}", err);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Event handlers
impl<'a, C> DeviceManager<'a, C>
    where C: 'static + EventHandling + StatePublishing + HwGraphics + Send + Clone
{
    pub fn on_suspend(&mut self) {
        // Nothing to do as for now...
    }

    pub fn on_wakeup(&mut self) {
        // Old event devices hung-up so devices must be reinitialized.
        self.initialize_input_devices();
    }
}

// -------------------------------------------------------------------------------------------------
