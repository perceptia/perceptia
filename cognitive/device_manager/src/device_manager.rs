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
use device_access::RestrictedOpener;
use input_collector::InputCollector;
use output_collector::OutputCollector;
use device_monitor::DeviceMonitor;
use virtual_terminal;

// -------------------------------------------------------------------------------------------------

/// Device Manager manages searching input and output devices and monitoring them.
///
/// It packs all the functionality from `cognitive-device-manager` in in single, easy to use
/// structure.
pub struct DeviceManager<C>
    where C: EventHandling + StatePublishing + HwGraphics + Clone
{
    udev: udev::Udev,
    input_collector: InputCollector<C>,
    output_collector: OutputCollector<C>,
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<C> DeviceManager<C>
    where C: 'static + EventHandling + StatePublishing + HwGraphics + Send + Clone
{
    /// Constructs new `DeviceManager`.
    pub fn new(input_handler: Box<InputHandling>,
               input_forwarder: Box<InputForwarding>,
               input_config: InputConfig,
               coordinator: C) -> Self {
        // Prepare restricted opener
        let restricted_opener = Self::prepare_restricted_opener();

        // Setup virtual terminal
        let vt = Self::setup_virtual_terminal(coordinator.clone(), &restricted_opener.borrow());

        // Create input collector
        let input_collector = InputCollector::new(coordinator.clone(),
                                                  input_handler,
                                                  input_forwarder,
                                                  input_config,
                                                  vt,
                                                  restricted_opener);
        // Create output collector
        let output_collector = OutputCollector::new(coordinator.clone());

        // Create `DeviceManager`
        let mut mine = DeviceManager {
            udev: udev::Udev::new(),
            input_collector: input_collector,
            output_collector: output_collector,
            coordinator: coordinator,
        };

        // Initialize input devices
        mine.scan_input_devices();

        // Initialize output devices
        mine.scan_output_devices();

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
    fn setup_virtual_terminal(coordinator: C,
                              restricted_opener: &RestrictedOpener)
                              -> Option<virtual_terminal::VirtualTerminal> {
        match virtual_terminal::VirtualTerminal::new(restricted_opener) {
            Ok(vt) => {
                match vt.get_current() {
                    Ok(tty_num) => {
                        match virtual_terminal::setup(tty_num, coordinator, restricted_opener) {
                            Ok(_) => {}
                            Err(err) => log_error!("Device Manager: {:?}", err),
                        }
                    }
                    Err(err) => {
                        log_error!("Device Manager: {:?}", err);
                    }
                }
                Some(vt)
            }
            Err(err) => {
                log_error!("Failed to open virtual terminal: {:?}", err);
                None
            }
        }
    }

    /// Iterate over input devices to find usable ones and initialize event handlers for them.
    fn scan_input_devices(&mut self) {
        if let Err(err) = self.input_collector.scan_devices(&self.udev) {
            log_warn1!("Failed to scan input devices: {:?}", err);
        }
    }

    /// Find and initialize outputs.
    fn scan_output_devices(&mut self) {
        let oc = &mut self.output_collector;
        self.udev.iterate_output_devices(|devnode, _| {
            if let Err(err) = oc.scan_device(devnode) {
                log_error!("{}", err);
            }
        });
    }

    /// Initialize device monitoring.
    fn initialize_device_monitor(&mut self) {
        match DeviceMonitor::new(self.coordinator.clone()) {
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
impl<C> DeviceManager<C>
    where C: 'static + EventHandling + StatePublishing + HwGraphics + Send + Clone
{
    /// This method is called when application is going to suspend (e.g. virtual terminal was
    /// switched).
    pub fn on_suspend(&mut self) {
        // Nothing to do as for now...
    }

    /// This method is called when application is going to wake up from suspension.
    pub fn on_wakeup(&mut self) {
        // Old event devices hung-up so devices must be reinitialized.
        self.scan_input_devices();
    }

    /// This method is called when state of one of input devices changed (was added or removed).
    pub fn on_inputs_changed(&mut self) {
        self.scan_input_devices();
    }

    /// This method is called when state of one of output devices changed (was added or removed).
    pub fn on_outputs_changed(&mut self) {
        self.scan_output_devices();
    }
}

// -------------------------------------------------------------------------------------------------
