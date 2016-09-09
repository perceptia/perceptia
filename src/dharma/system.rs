// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains helper functionality for handling system signals.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::{RawFd, AsRawFd};
use nix::sys::{signal, signalfd};

use bridge;
use dispatcher::{Dispatcher, EventHandler};
use signaler::Signaler;

// -------------------------------------------------------------------------------------------------

/// Block signals `SIGINT` and `SIGTERM` for current thread.
pub fn block_signals() {
    let mut mask = signal::SigSet::empty();
    mask.add(signal::SIGINT).unwrap();
    mask.add(signal::SIGTERM).unwrap();
    mask.thread_block().unwrap();
}

// -------------------------------------------------------------------------------------------------

/// Unblock signals `SIGINT` and `SIGTERM` for current thread.
pub fn unblock_signals() {
    let mut mask = signal::SigSet::empty();
    mask.add(signal::SIGINT).unwrap();
    mask.add(signal::SIGTERM).unwrap();
    mask.thread_unblock().unwrap();
}

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::EventHandler` for handling system signals synchronously. For this to
/// work receiving of signals `SIGINT` and `SIGTERM` must be blocked in all threads in application.
/// Otherwise non-blocking threads will catch all signals.
pub struct SignalEventHandler<P>
    where P: bridge::Transportable + 'static
{
    fd: signalfd::SignalFd,
    dispatcher: Dispatcher,
    signaler: Signaler<P>,
}

// -------------------------------------------------------------------------------------------------

impl<P> SignalEventHandler<P>
    where P: bridge::Transportable + 'static
{
    /// `SignalEventHandler` constructor. Creates `SignalEventHandler` ready for handling `SIGINT`
    /// and `SIGTERM` signals.
    pub fn new(dispatcher: Dispatcher, signaler: Signaler<P>) -> Self {
        let mut mask = signal::SigSet::empty();
        mask.add(signal::SIGINT).unwrap();
        mask.add(signal::SIGTERM).unwrap();
        SignalEventHandler {
            fd: signalfd::SignalFd::new(&mask).unwrap(),
            dispatcher: dispatcher,
            signaler: signaler,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<P> EventHandler for SignalEventHandler<P>
    where P: bridge::Transportable + 'static
{
    fn get_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }

    fn process_event(&mut self) {
        match self.fd.read_signal() {
            Ok(ossi) => {
                match ossi {
                    Some(ssi) => {
                        // FIXME: Do signals have correct type in `nix`?
                        if (ssi.ssi_signo as i32 == signal::SIGINT) ||
                           (ssi.ssi_signo as i32 == signal::SIGTERM) {
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
    }
}

// -------------------------------------------------------------------------------------------------
