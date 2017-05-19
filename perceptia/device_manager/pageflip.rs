// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module implements helper functionality for handling pageflips.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;
use libdrm::drm;

use dharma::{EventHandler, EventKind, Signaler, event_kind};
use qualia::{perceptron, Perceptron};

// -------------------------------------------------------------------------------------------------

/// Context passed ti libdrm to handle page flips.
struct PageFlipContext {
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl PageFlipContext {
    /// `PageFlipContext` constructor.
    pub fn new(signaler: Signaler<Perceptron>) -> Self {
        PageFlipContext { signaler: signaler }
    }
}

// -------------------------------------------------------------------------------------------------

impl drm::EventContext for PageFlipContext {
    #[allow(unused_variables)]
    fn vblank_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32) {
        self.signaler.emit(perceptron::VERTICAL_BLANK, Perceptron::VerticalBlank(data));
    }

    #[allow(unused_variables)]
    fn page_flip_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32) {
        self.signaler.emit(perceptron::PAGE_FLIP, Perceptron::PageFlip(data));
    }
}

// -------------------------------------------------------------------------------------------------

/// `dharma` event handler for pageflip events.
pub struct PageFlipEventHandler {
    drm_fd: io::RawFd,
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl PageFlipEventHandler {
    /// `PageFlipEventHandler` constructor.
    pub fn new(fd: io::RawFd, signaler: Signaler<Perceptron>) -> Self {
        PageFlipEventHandler {
            drm_fd: fd,
            signaler: signaler,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This code executes in main dispatchers thread.
impl EventHandler for PageFlipEventHandler {
    fn get_fd(&self) -> io::RawFd {
        self.drm_fd
    }

    fn process_event(&mut self, event_kind: EventKind) {
        if event_kind.intersects(event_kind::READ) {
            let ctx = Box::new(PageFlipContext::new(self.signaler.clone()));
            drm::handle_event(self.drm_fd, ctx);
        } else if event_kind.intersects(event_kind::HANGUP) {
            // It seems that DRM devices do not hang-up during virtual terminal switch and after
            // application regains access they are ready to use.
        }
    }
}

// -------------------------------------------------------------------------------------------------
