// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains helper functionality for handling system signals.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::{RawFd, AsRawFd};
use nix::sys::{signal, signalfd};

use dispatcher::{DispatcherController, EventHandler, EventKind};
use signaler::Signaler;

// -------------------------------------------------------------------------------------------------

/// Blocks signals `SIGINT`, `SIGTERM`, `SIGUSR1` and `SIGUSR2` for current thread.
pub fn block_signals() {
    let mut mask = signal::SigSet::empty();
    mask.add(signal::SIGINT);
    mask.add(signal::SIGTERM);
    mask.add(signal::SIGUSR1);
    mask.add(signal::SIGUSR2);
    mask.thread_block().unwrap();
}

// -------------------------------------------------------------------------------------------------

/// Unblocks signals `SIGINT`, `SIGTERM`, `SIGUSR1` and `SIGUSR2` for current thread.
pub fn unblock_signals() {
    let mut mask = signal::SigSet::empty();
    mask.add(signal::SIGINT);
    mask.add(signal::SIGTERM);
    mask.add(signal::SIGUSR1);
    mask.add(signal::SIGUSR2);
    mask.thread_unblock().unwrap();
}

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::EventHandler` for handling system signals synchronously. For this to
/// work receiving of signals `SIGINT` and `SIGTERM` must be blocked in all threads in application.
/// Otherwise non-blocking threads will catch all signals.
pub struct SignalEventHandler<P>
    where P: Clone + Send + 'static
{
    fd: signalfd::SignalFd,
    dispatcher: DispatcherController,
    signaler: Signaler<P>,
}

// -------------------------------------------------------------------------------------------------

impl<P> SignalEventHandler<P>
    where P: Clone + Send + 'static
{
    /// `SignalEventHandler` constructor. Creates `SignalEventHandler` ready for handling `SIGINT`
    /// and `SIGTERM` signals.
    pub fn new(dispatcher: DispatcherController, signaler: Signaler<P>) -> Self {
        let mut mask = signal::SigSet::empty();
        mask.add(signal::SIGINT);
        mask.add(signal::SIGTERM);
        SignalEventHandler {
            fd: signalfd::SignalFd::new(&mask).unwrap(),
            dispatcher: dispatcher,
            signaler: signaler,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<P> EventHandler for SignalEventHandler<P>
    where P: Clone + Send + 'static
{
    fn get_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }

    fn process_event(&mut self, _: EventKind) -> Result<(), ()> {
        match self.fd.read_signal() {
            Ok(ossi) => {
                match ossi {
                    Some(ssi) => {
                        // FIXME: Do signals have correct type in `nix`?
                        if (ssi.ssi_signo == signal::SIGINT as u32) ||
                           (ssi.ssi_signo == signal::SIGTERM as u32) {
                            self.dispatcher.stop();
                            self.signaler.terminate();
                        }
                    }
                    None => {
                        panic!("Received invalid siginfo!");
                    }
                }
            }
            Err(err) => {
                panic!("Error occurred during processing signal! ({:?})", err);
            }
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
