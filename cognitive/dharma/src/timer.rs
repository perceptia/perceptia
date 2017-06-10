// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains helper functionality for handling system signals.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::{RawFd, AsRawFd};
use std::time::Duration;

use timerfd::{TimerFd, TimerState};

use dispatcher::{EventHandler, EventKind, event_kind};

// -------------------------------------------------------------------------------------------------

/// General purpose timer implementing `EventHandler`.
pub struct Timer<F>
    where F: FnMut() -> ()
{
    /// Callback function called every time timer expires.
    callback: F,

    /// TimerFd.
    timer_fd: TimerFd,
}

// -------------------------------------------------------------------------------------------------

impl<F> Timer<F>
    where F: FnMut() -> ()
{
    /// Constructs new `Timer` expiring periodically with given interval and calling given
    /// `callback`.
    pub fn new(interval: Duration, callback: F) -> Result<Self, ()> {
        if let Ok(mut timer_fd) = TimerFd::new_custom(false, true, true) {
            timer_fd.set_state(TimerState::Periodic {
                                   current: interval,
                                   interval: interval,
                               });
            Ok(Timer {
                   callback: callback,
                   timer_fd: timer_fd,
               })
        } else {
            Err(())
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<F> EventHandler for Timer<F>
    where F: FnMut() -> ()
{
    fn get_fd(&self) -> RawFd {
        self.timer_fd.as_raw_fd()
    }

    fn process_event(&mut self, event_kind: EventKind) {
        if event_kind.intersects(event_kind::READ) {
            self.timer_fd.read();
            (self.callback)();
        }
    }
}

// -------------------------------------------------------------------------------------------------
