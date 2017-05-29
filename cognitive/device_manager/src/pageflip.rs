// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module implements helper functionality for handling pageflips.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;
use libdrm::drm;

use dharma::{EventHandler, EventKind, event_kind};
use qualia::StatePublishing;

// -------------------------------------------------------------------------------------------------

/// Context passed ti libdrm to handle page flips.
struct PageFlipContext<P> where P: StatePublishing {
    state_publisher: P,
}

// -------------------------------------------------------------------------------------------------

impl<P> PageFlipContext<P> where P: StatePublishing {
    /// `PageFlipContext` constructor.
    pub fn new(state_publisher: P) -> Self {
        PageFlipContext { state_publisher: state_publisher }
    }
}

// -------------------------------------------------------------------------------------------------

impl<P> drm::EventContext for PageFlipContext<P> where P: StatePublishing {
    #[allow(unused_variables)]
    fn vblank_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32) {
        self.state_publisher.emit_vblank(data);
    }

    #[allow(unused_variables)]
    fn page_flip_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32) {
        self.state_publisher.emit_page_flip(data);
    }
}

// -------------------------------------------------------------------------------------------------

/// `dharma` event handler for pageflip events.
pub struct PageFlipEventHandler<P> where P: StatePublishing {
    drm_fd: io::RawFd,
    state_publisher: P,
}

// -------------------------------------------------------------------------------------------------

impl<P> PageFlipEventHandler<P> where P: StatePublishing {
    /// `PageFlipEventHandler` constructor.
    pub fn new(fd: io::RawFd, state_publisher: P) -> Self {
        PageFlipEventHandler {
            drm_fd: fd,
            state_publisher: state_publisher,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This code executes in main dispatchers thread.
impl<P> EventHandler for PageFlipEventHandler<P> where P: 'static + StatePublishing + Send + Clone {
    fn get_fd(&self) -> io::RawFd {
        self.drm_fd
    }

    fn process_event(&mut self, event_kind: EventKind) {
        if event_kind.intersects(event_kind::READ) {
            let ctx = Box::new(PageFlipContext::new(self.state_publisher.clone()));
            drm::handle_event(self.drm_fd, ctx);
        } else if event_kind.intersects(event_kind::HANGUP) {
            // It seems that DRM devices do not hang-up during virtual terminal switch and after
            // application regains access they are ready to use.
        }
    }
}

// -------------------------------------------------------------------------------------------------
