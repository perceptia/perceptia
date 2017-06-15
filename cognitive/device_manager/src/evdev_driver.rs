// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Driver for evdev input devices.

// -------------------------------------------------------------------------------------------------

use std::mem;
use std::os::unix::io;
use std::path::Path;
use std::sync::{Arc, Mutex};
use libc;

use nix::fcntl;
use nix::sys::stat;
use nix::unistd::read;

use dharma::{EventHandler, EventKind, event_kind};
use qualia::{DeviceKind, Illusion, InputConfig, InputForwarding};
use inputs::codes;

use drivers;
use device_access::RestrictedOpener;

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(C)]
struct InputEvent {
    pub time: libc::timeval,
    pub kind: u16,
    pub code: u16,
    pub value: i32,
}

// -------------------------------------------------------------------------------------------------

/// Structure representing evdev input device driver.
pub struct Evdev {
    fd: io::RawFd,
    device_kind: DeviceKind,
    config: InputConfig,
    gateway: Arc<Mutex<InputForwarding>>,
    pressure: i32,
}

// -------------------------------------------------------------------------------------------------

impl drivers::InputDriver for Evdev {
    fn initialize_device(devnode: &Path,
                         device_kind: DeviceKind,
                         config: InputConfig,
                         gateway: Arc<Mutex<InputForwarding>>,
                         ro: &RestrictedOpener)
                         -> Result<Box<Self>, Illusion> {
        let r = ro.open(devnode, fcntl::O_RDONLY | fcntl::O_CLOEXEC, stat::Mode::empty());
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

    fn process_event(&mut self, event_kind: EventKind) -> Result<(), ()> {
        if event_kind.intersects(event_kind::READ) {
            self.read_events();
        } else if event_kind.intersects(event_kind::HANGUP) {
            self.gateway.lock().unwrap().emit_system_activity_event();
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

impl Evdev {
    /// `Evdev` constructor.
    fn new(fd: io::RawFd,
           device_kind: DeviceKind,
           config: InputConfig,
           gateway: Arc<Mutex<InputForwarding>>)
           -> Self {
        Evdev {
            fd: fd,
            device_kind: device_kind,
            config: config,
            gateway: gateway,
            pressure: 0,
        }
    }

    /// Reads events.
    fn read_events(&mut self) {
        let mut ev: InputEvent = unsafe { mem::uninitialized() };
        let data = unsafe { mem::transmute::<&mut InputEvent, &mut [u8; 3 * 8]>(&mut ev) };
        match read(self.fd, &mut data[..]) {
            Ok(_) => {
                match self.device_kind {
                    DeviceKind::Keyboard => self.process_keyboard_event(&ev),
                    DeviceKind::Mouse => self.process_mouse_event(&ev),
                    DeviceKind::Touchpad => self.process_touchpad_event(&ev),
                    DeviceKind::Unknown => panic!("Received event from device of unknown type"),
                }
            }
            Err(err) => log_warn2!("Error during reading input: {:?}", err),
        }
    }

    /// Helper method for processing keyboard events.
    fn process_keyboard_event(&mut self, ev: &InputEvent) {
        if ev.kind == codes::EV_KEY {
            self.gateway.lock().unwrap().emit_key(ev.code, ev.value);
        }
    }

    /// Helper method for processing mouse events.
    fn process_mouse_event(&mut self, ev: &InputEvent) {
        if ev.kind == codes::EV_SYN {
            // Ignore sync
        } else if ev.kind == codes::EV_KEY {
            if (ev.code == codes::BTN_LEFT) || (ev.code == codes::BTN_MIDDLE) ||
               (ev.code == codes::BTN_RIGHT) {
                self.gateway.lock().unwrap().emit_button(ev.code, ev.value);
            } else {
                log_nyimp!("Unhandled mouse key event (code: {}, value: {})", ev.code, ev.value);
            }
        } else if ev.kind == codes::EV_REL {
            if ev.code == codes::ABS_X {
                let value = ev.value as f32 * self.config.mouse_scale;
                self.gateway.lock().unwrap().emit_motion(value as isize, 0);
            } else if ev.code == codes::ABS_Y {
                let value = ev.value as f32 * self.config.mouse_scale;
                self.gateway.lock().unwrap().emit_motion(0, value as isize);
            } else if ev.code == codes::REL_WHEEL {
                self.gateway.lock().unwrap().emit_axis(0, ev.value as isize);
            } else {
                log_nyimp!("Unhandled mouse relative event (code: {}, value: {})",
                           ev.code,
                           ev.value);
            }
        } else if ev.kind == codes::EV_ABS {
            log_nyimp!("Unhandled mouse absolute event (code: {}, value: {})", ev.code, ev.value);
        } else {
            log_nyimp!("Unhandled mouse event (type: {}, code: {}, value: {})",
                       ev.kind,
                       ev.code,
                       ev.value);
        }
    }

    /// Helper method for processing touchpad events.
    fn process_touchpad_event(&mut self, ev: &InputEvent) {
        if ev.kind == codes::EV_SYN {
            // Ignore sync
        } else if ev.kind == codes::EV_KEY {
            if (ev.code == codes::BTN_LEFT) || (ev.code == codes::BTN_MIDDLE) ||
               (ev.code == codes::BTN_RIGHT) {
                self.gateway.lock().unwrap().emit_button(ev.code, ev.value);
            } else if (ev.code == codes::BTN_TOOL_FINGER) || (ev.code == codes::BTN_TOUCH) {
                self.gateway.lock().unwrap().emit_position_reset(None);
            } else {
                log_nyimp!("Unhandled touchpad key event (code: {}, value: {})", ev.code, ev.value);
            }
        } else if ev.kind == codes::EV_REL {
            log_nyimp!("Unhandled touchpad relative event (code: {}, value: {})",
                       ev.code,
                       ev.value);
        } else if ev.kind == codes::EV_ABS {
            if ev.code == codes::ABS_PRESSURE {
                log_info4!("Touchpad pressure: {:?}", ev.value);
                self.pressure = ev.value;
            } else if ev.code == codes::ABS_MT_TRACKING_ID {
                self.gateway.lock().unwrap().emit_position_reset(None);
            } else if self.pressure > self.config.touchpad_pressure_threshold {
                if (ev.code == codes::ABS_MT_POSITION_X) || (ev.code == codes::ABS_X) {
                    let value = ev.value as f32 * self.config.touchpad_scale;
                    self.gateway.lock().unwrap().emit_position(Some(value as isize), None);
                } else if (ev.code == codes::ABS_MT_POSITION_Y) || (ev.code == codes::ABS_Y) {
                    let value = ev.value as f32 * self.config.touchpad_scale;
                    self.gateway.lock().unwrap().emit_position(None, Some(value as isize));
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
