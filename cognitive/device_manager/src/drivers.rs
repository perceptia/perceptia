// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains trait of device drivers.

// -------------------------------------------------------------------------------------------------

use std::path::Path;
use std::sync::{Arc, Mutex};

use qualia::{DeviceKind, Illusion, InputConfig, InputForwarding};

use device_access::RestrictedOpener;

// -------------------------------------------------------------------------------------------------

/// Trait for input event devices like keyboard, mouse or touchpad.
pub trait InputDriver {
    /// Initialize drive. Return driver instance on success or error otherwise.
    fn initialize_device(devnode: &Path,
                         device_kind: DeviceKind,
                         config: InputConfig,
                         gateway: Arc<Mutex<InputForwarding>>,
                         ro: &RestrictedOpener)
                         -> Result<Box<Self>, Illusion>;
}

// -------------------------------------------------------------------------------------------------
