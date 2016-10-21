// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains trait of device drivers.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use std::os::unix::io;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;

use qualia::{DeviceKind, Illusion, InputConfig};

use input_gateway::InputGateway;

// -------------------------------------------------------------------------------------------------

/// Trait for input event devices like keyboard, mouse or touchpad.
pub trait InputDriver {
    /// Initialize drive. Return driver instance on success or error otherwise.
    fn initialize_device<F>(devnode: &Path,
                            device_kind: DeviceKind,
                            config: InputConfig,
                            gateway: InputGateway,
                            open_restricted: F)
                            -> Result<Box<Self>, Illusion>
        where F: Fn(&Path, OFlag, Mode) -> Result<io::RawFd, Illusion>;
}

// -------------------------------------------------------------------------------------------------
