// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Weston `weston_screenshooter` object.

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::weston_screenshooter::weston_screenshooter;

use facade::Facade;
use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Weston `weston_screenshooter` object.
struct Screenshooter {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(weston_screenshooter::NAME,
                weston_screenshooter::VERSION,
                Rc::new(Screenshooter::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Screenshooter {
    fn new(proxy_ref: ProxyRef) -> Self {
        Screenshooter { proxy: proxy_ref }
    }

    fn new_object(_oid: ObjectId, _version: u32, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, weston_screenshooter::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl weston_screenshooter::Interface for Screenshooter {
    fn shoot(&mut self,
             this_object_id: ObjectId,
             _bundle: &mut Bundle,
             output: ObjectId,
             buffer: ObjectId)
             -> Task {
        self.proxy.borrow_mut().take_screenshot(this_object_id, output, buffer);
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
