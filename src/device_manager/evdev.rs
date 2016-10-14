// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Driver for evdev input devices.

// -------------------------------------------------------------------------------------------------

use std::{fmt, mem};
use std::os::unix::io;
use std::path::Path;
use uinput_sys::{self, input_event};

use nix::fcntl::{self, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::read;

use qualia::{DeviceKind, Error, InputConfig};
use dharma::EventHandler;

use drivers;
use input_gateway::InputGateway;

// -------------------------------------------------------------------------------------------------

/// Structure representing evdev input device driver.
pub struct Evdev {
    fd: io::RawFd,
    device_kind: DeviceKind,
    config: InputConfig,
    gateway: InputGateway,
    pressure: i32,
}

// -------------------------------------------------------------------------------------------------

impl drivers::InputDriver for Evdev {
    fn initialize_device<F>(devnode: &Path,
                            device_kind: DeviceKind,
                            config: InputConfig,
                            gateway: InputGateway,
                            open_restricted: F)
                            -> Result<Box<Self>, Error>
        where F: Fn(&Path, OFlag, Mode) -> Result<io::RawFd, Error>
    {
        let r = open_restricted(devnode, fcntl::O_RDONLY, Mode::empty());
        match r {
            Ok(fd) => Ok(Box::new(Evdev::new(fd, device_kind, config, gateway))),
            Err(err) => Err(err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl EventHandler for Evdev {
    fn get_fd(&self) -> io::RawFd {
        self.fd
    }

    fn process_event(&mut self) {
        let mut ev: input_event = unsafe { mem::uninitialized() };
        match read(self.fd,
                   unsafe { mem::transmute::<&mut input_event, &mut [u8; 3 * 8]>(&mut ev) }) {
            Ok(size) => {
                match self.device_kind {
                    DeviceKind::Keyboard => self.process_keyboard_event(&ev),
                    DeviceKind::Mouse => self.process_mouse_event(&ev),
                    DeviceKind::Touchpad => self.process_touchpad_event(&ev),
                    DeviceKind::Unknown => panic!("Received event from device of unknown type"),
                }
            }
            Err(err) => log_warn2!("Error during reading imput: {:?}", err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Evdev {
    /// `Evdev` constructor.
    fn new(fd: io::RawFd,
           device_kind: DeviceKind,
           config: InputConfig,
           gateway: InputGateway)
           -> Self {
        Evdev {
            fd: fd,
            device_kind: device_kind,
            config: config,
            gateway: gateway,
            pressure: 0,
        }
    }

    /// Helper method for processing keyboard events.
    fn process_keyboard_event(&mut self, ev: &input_event) {
        if ev.kind == uinput_sys::EV_KEY as _ {
            self.gateway.emit_key(ev.code, ev.value);
        }
    }

    /// Helper method for processing mouse events.
    fn process_mouse_event(&mut self, ev: &input_event) {
        if ev.kind == uinput_sys::EV_SYN as _ {
            // Ignore sync
        } else if ev.kind == uinput_sys::EV_KEY as _ {
            if (ev.code == uinput_sys::BTN_LEFT as _) || (ev.code == uinput_sys::BTN_MIDDLE as _) ||
               (ev.code == uinput_sys::BTN_RIGHT as _) {
                self.gateway.emit_button(ev.code, ev.value);
            } else {
                log_nyimp!("Unhandled mouse key event (code: {}, value: {})",
                           ev.code,
                           ev.value);
            }
        } else if ev.kind == uinput_sys::EV_REL as _ {
            if ev.code == uinput_sys::ABS_X as _ {
                self.gateway.emit_motion(ev.value, 0);
            } else if ev.code == uinput_sys::ABS_Y as _ {
                self.gateway.emit_motion(0, ev.value);
            } else if ev.code == uinput_sys::REL_WHEEL as _ {
                self.gateway.emit_axis(0, ev.value);
            } else {
                log_nyimp!("Unhandled mouse relative event (code: {}, value: {})",
                           ev.code,
                           ev.value);
            }
        } else if ev.kind == uinput_sys::EV_ABS as _ {
            log_nyimp!("Unhandled mouse absolute event (code: {}, value: {})",
                       ev.code,
                       ev.value);
        } else {
            log_nyimp!("Unhandled mouse event (type: {}, code: {}, value: {})",
                       ev.kind,
                       ev.code,
                       ev.value);
        }
    }

    /// Helper method for processing touchpad events.
    fn process_touchpad_event(&mut self, ev: &input_event) {
        if ev.kind == uinput_sys::EV_SYN as _ {
            // Ignore sync
        } else if ev.kind == uinput_sys::EV_KEY as _ {
            if (ev.code == uinput_sys::BTN_LEFT as _) || (ev.code == uinput_sys::BTN_MIDDLE as _) ||
               (ev.code == uinput_sys::BTN_RIGHT as _) {
                self.gateway.emit_button(ev.code, ev.value);
            } else if (ev.code == uinput_sys::BTN_TOOL_FINGER as _) ||
                      (ev.code == uinput_sys::BTN_TOUCH as _) {
                self.gateway.emit_position_reset();
            } else {
                log_nyimp!("Unhandled touchpad key event (code: {}, value: {})",
                           ev.code,
                           ev.value);
            }
        } else if ev.kind == uinput_sys::EV_REL as _ {
            log_nyimp!("Unhandled touchpad relative event (code: {}, value: {})",
                       ev.code,
                       ev.value);
        } else if ev.kind == uinput_sys::EV_ABS as _ {
            if ev.code == uinput_sys::ABS_PRESSURE as _ {
                log_info4!("Touchpad pressure: {:?}", ev.value);
                self.pressure = ev.value;
            } else if ev.code == uinput_sys::ABS_MT_TRACKING_ID as _ {
                self.gateway.emit_position_reset();
            } else if self.pressure > self.config.touchpad_pressure_treshold {
                if (ev.code == uinput_sys::ABS_MT_POSITION_X as _) ||
                   (ev.code == uinput_sys::ABS_X as _) {
                    self.gateway.emit_position(Some(ev.value), None);
                } else if (ev.code == uinput_sys::ABS_MT_POSITION_Y as _) ||
                          (ev.code == uinput_sys::ABS_Y as _) {
                    self.gateway.emit_position(None, Some(ev.value));
                }
            }
        } else {
            log_nyimp!("Unhandled touchpad event (type: {}, code: {}, value: {})",
                       ev.kind,
                       ev.code,
                       ev.value);
        }
    }
}

// -------------------------------------------------------------------------------------------------
