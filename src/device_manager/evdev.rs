// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Driver for evdev input devices.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io;
use std::path::Path;

use nix::fcntl::{self, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::read;

use qualia::Error;
use dharma::EventHandler;

use drivers;

// -------------------------------------------------------------------------------------------------

/// Structure representing evdev input device driver.
pub struct Evdev {
    fd: io::RawFd,
}

// -------------------------------------------------------------------------------------------------

impl drivers::InputDriver for Evdev {
    fn initialize_device<F>(devnode: &Path, open_restricted: F) -> Result<Box<Self>, Error>
        where F: Fn(&Path, OFlag, Mode) -> Result<io::RawFd, Error>
    {
        let r = open_restricted(devnode, fcntl::O_RDONLY, Mode::empty());
        match r {
            Ok(fd) => Ok(Box::new(Evdev { fd: fd })),
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
        // FIXME: Implement real event handling
        println!("Processing event! {:?}", self.get_fd());
        let mut buf: [u8; 3 * 8] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0];
        match read(self.fd, &mut buf) {
            Ok(size) => println!("Read {}", size),
            Err(err) => println!("Error {:?}", err),
        }
    }
}

// -------------------------------------------------------------------------------------------------
