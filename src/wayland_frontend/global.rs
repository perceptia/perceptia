// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to Wayland global objects.

// -------------------------------------------------------------------------------------------------

use skylane as wl;

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Type alias for constructor of Wayland global objects.
type GlobalContructor = Fn(wl::common::ObjectId, ProxyRef) -> Box<wl::server::Object>;

// -------------------------------------------------------------------------------------------------

/// Structure representing global Wayland object.
// TODO: Define new type for name.
pub struct Global {
    pub name: u32,
    pub interface: &'static str,
    pub version: u32,
    constructor: Box<GlobalContructor>,
}

// -------------------------------------------------------------------------------------------------

impl Global {
    pub fn new(interface: &'static str, version: u32, constructor: Box<GlobalContructor>) -> Self {
        Global {
            name: 0,
            interface: interface,
            version: version,
            constructor: constructor,
        }
    }

    pub fn construct(&self, id: wl::common::ObjectId, proxy: ProxyRef) -> Box<wl::server::Object> {
        (self.constructor)(id, proxy)
    }
}

// -------------------------------------------------------------------------------------------------
