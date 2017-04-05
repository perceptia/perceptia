// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

/// This module contains functionality related to managing virtual terminals.

// -------------------------------------------------------------------------------------------------

use nix::fcntl;
use nix::sys::{signal, signalfd, stat};
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;
use std::path::Path;
use libc;

use dharma::{Dispatcher, EventHandler, EventKind, Signaler, event_kind};

use qualia::{Illusion, Perceptron, perceptron};

use device_access::RestrictedOpener;

// -------------------------------------------------------------------------------------------------

/// Path to default terminal
const DEFAULT_TTY_PATH: &'static str = "/dev/tty";

/// Base part of terminal paths
const BASE_TTY_PATH: &'static str = "/dev/tty";

// -------------------------------------------------------------------------------------------------

mod ioctl {
    const VT_SETMODE: u32 = 0x5602;
    const VT_GETSTATE: u32 = 0x5603;
    const VT_RELDISP: u32 = 0x5605;
    const VT_ACTIVATE: u32 = 0x5606;

    pub const PROCESS: u32 = 0x1;
    pub const ACK_ACQ: u32 = 0x2;

    ioctl!(set_vt_mode with VT_SETMODE);
    ioctl!(get_vt_state with VT_GETSTATE);
    ioctl!(release_vt with VT_RELDISP);
    ioctl!(activate_vt with VT_ACTIVATE);
}

// -------------------------------------------------------------------------------------------------

/// Handler for events about switching virtual terminal.
pub struct SwitchHandler {
    signal_fd: signalfd::SignalFd,
    tty_fd: RawFd,
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl SwitchHandler {
    /// Constructs new `SwitchHandler`.
    pub fn new(tty_fd: RawFd, signaler: Signaler<Perceptron>) -> Self {
        let mut mask = signal::SigSet::empty();
        mask.add(signal::SIGUSR1);
        mask.add(signal::SIGUSR2);
        SwitchHandler {
            signal_fd: signalfd::SignalFd::new(&mask).expect("Creation of signalfd"),
            tty_fd: tty_fd,
            signaler: signaler,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SwitchHandler {
    /// Handles activation of virtual terminal this application is assigned to.
    fn handle_activation(&mut self) {
        log_info1!("Virtual terminal activation");
        self.signaler.emit(perceptron::WAKEUP, Perceptron::WakeUp);
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Handles deactivation of virtual terminal this application is assigned to.
    ///
    /// Immediately releases the virtual terminal. Application should be prepared to loose access to
    /// devices and handle such situation gracefully.
    fn handle_deactivation(&mut self) {
        log_info1!("Virtual terminal deactivation");
        match unsafe { ioctl::release_vt(self.tty_fd, ioctl::ACK_ACQ as *mut u8) } {
            Ok(_) => self.signaler.emit(perceptron::SUSPEND, Perceptron::Suspend),
            Err(err) => log_warn1!("Failed to release VT: {:?}", err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl EventHandler for SwitchHandler {
    fn get_fd(&self) -> RawFd {
        self.signal_fd.as_raw_fd()
    }

    fn process_event(&mut self, _: EventKind) {
        match self.signal_fd.read_signal() {
            Ok(ossi) => {
                match ossi {
                    Some(ssi) => {
                        if ssi.ssi_signo == signal::SIGUSR1 as u32 {
                            self.handle_deactivation();
                        } else if ssi.ssi_signo == signal::SIGUSR2 as u32 {
                            self.handle_activation();
                        } else {
                            log_warn2!("Received unexpected signal ({})", ssi.ssi_signo);
                        }
                    }
                    None => {
                        log_warn1!("Received invalid siginfo!");
                    }
                }
            }
            Err(err) => {
                log_warn1!("Error occurred during processing signal! ({:?})", err);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure used to pass mode information to kernel via `ioctl`.
#[repr(C)]
struct VtMode {
    mode: libc::c_char,
    waitv: libc::c_char,
    relsig: libc::c_short,
    acqsig: libc::c_short,
    frsig: libc::c_short,
}

// -------------------------------------------------------------------------------------------------

impl VtMode {
    /// Constructs new `VtMode`.
    pub fn new(hang_on_writes: bool,
               relsig: signal::Signal,
               acqsig: signal::Signal)
               -> Self {
        VtMode {
            mode: ioctl::PROCESS as libc::c_char,
            waitv: hang_on_writes as libc::c_char,
            relsig: relsig as libc::c_short,
            acqsig: acqsig as libc::c_short,
            frsig: 0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure used to get state information from kernel via `ioctl`.
#[repr(C)]
struct VtState {
    pub active: libc::c_short,
    pub signal: libc::c_short,
    pub state: libc::c_short,
}

// -------------------------------------------------------------------------------------------------

impl VtState {
    /// Constructs new `VtMode`.
    pub fn new() -> Self {
        VtState {
            active: 0,
            signal: 0,
            state: 0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents a virtual terminal.
#[derive(Clone, Copy)]
pub struct VirtualTerminal {
    fd: RawFd,
}

// -------------------------------------------------------------------------------------------------

impl VirtualTerminal {
    /// Constructs new `VirtualTerminal`.
    pub fn new(ro: &RestrictedOpener) -> Result<Self, Illusion> {
        match ro.open(&Path::new(DEFAULT_TTY_PATH), fcntl::O_WRONLY, stat::Mode::empty()) {
            Ok(tty_fd) => Ok(VirtualTerminal { fd: tty_fd }),
            Err(err) => {
                let text = format!("Failed to open VT device {:?}: {:?}", DEFAULT_TTY_PATH, err);
                Err(Illusion::General(text))
            }
        }
    }

    /// Returns number of current virtual terminal.
    pub fn get_current(&self) -> Result<u32, Illusion> {
        let mut state = VtState::new();
        let data = unsafe { mem::transmute::<&mut VtState, &mut u8>(&mut state) as *mut u8 };
        match unsafe { ioctl::get_vt_state(self.fd, data) } {
            Ok(_) => Ok(state.active as u32),
            Err(err) => {
                let text = format!("Failed to get state of terminal: {:?}", err);
                Err(Illusion::General(text))
            }
        }
    }

    /// Switches to given virtual terminal.
    pub fn switch_to(&self, num: u8) -> Result<(), Illusion> {
        match unsafe { ioctl::activate_vt(self.fd, num as *mut u8) } {
            Ok(_) => Ok(()),
            Err(err) => {
                let text = format!("Failed to activate virtual terminal {}: {:?}", num, err);
                Err(Illusion::General(text))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Subscribes for terminal activation and deactivation events.
fn subscribe(path: &Path,
             mut dispatcher: Dispatcher,
             signaler: Signaler<Perceptron>,
             ro: &RestrictedOpener)
             -> Result<(), Illusion> {
    match ro.open(path, fcntl::O_WRONLY, stat::Mode::empty()) {
        Ok(tty_fd) => {
            let mut mode = VtMode::new(true, signal::SIGUSR1, signal::SIGUSR2);
            let data = unsafe { mem::transmute::<&mut VtMode, &mut u8>(&mut mode) as *mut u8 };
            match unsafe { ioctl::set_vt_mode(tty_fd, data) } {
                Ok(_) => {
                    let signal_source = Box::new(SwitchHandler::new(tty_fd, signaler));
                    dispatcher.add_source(signal_source, event_kind::READ);
                    Ok(())
                }
                Err(err) => {
                    let text = format!("Failed to subscribe for terminal events: {:?}", err);
                    Err(Illusion::General(text))
                }
            }

        }
        Err(err) => {
            let text = format!("Failed to open terminal device {:?}: {:?}", path, err);
            Err(Illusion::General(text))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Sets up virtual terminal by subscribing messages about its activation and deactivation.
pub fn setup(tty_num: u32,
             dispatcher: Dispatcher,
             signaler: Signaler<Perceptron>,
             ro: &RestrictedOpener)
             -> Result<(), Illusion> {
    let path_str = format!("{}{}", BASE_TTY_PATH, tty_num);
    let path = Path::new(&path_str);

    log_info2!("Setting up virtual terminal '{}'", path_str);
    subscribe(path, dispatcher, signaler, ro)
}

// -------------------------------------------------------------------------------------------------
