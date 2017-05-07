// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to Wayland global objects.

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;

use skylane::server as wl;

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Type alias for constructor of Wayland global objects.
type GlobalContructor = Fn(wl::ObjectId, u32, ProxyRef) -> Box<wl::Object>;

// -------------------------------------------------------------------------------------------------

/// Structure representing global Wayland object.
// TODO: Define new type for name.
#[derive(Clone)]
pub struct Global {
    pub name: u32,
    pub interface: &'static str,
    pub version: u32,
    constructor: Rc<GlobalContructor>,
}

// -------------------------------------------------------------------------------------------------

impl Global {
    pub fn new(interface: &'static str, version: u32, constructor: Rc<GlobalContructor>) -> Self {
        Global {
            name: 0,
            interface: interface,
            version: version,
            constructor: constructor,
        }
    }

    pub fn construct(&self, id: wl::ObjectId, version: u32, proxy: ProxyRef) -> Box<wl::Object> {
        (self.constructor)(id, version, proxy)
    }
}

// -------------------------------------------------------------------------------------------------
