// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_output` object.

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_output;

use qualia::OutputInfo;

use global::Global;
use proxy::ProxyRef;
use facade::Facade;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_output` object.
struct Output {}

// -------------------------------------------------------------------------------------------------

pub fn get_global(info: OutputInfo) -> Global {
    Global::new(wl_output::NAME,
                wl_output::VERSION,
                Rc::new(move |oid, _version, proxy| Output::new_object(oid, proxy, info.clone())))
}

// -------------------------------------------------------------------------------------------------

impl Output {
    fn new(oid: ObjectId, proxy_ref: ProxyRef, info: OutputInfo) -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            let socket = proxy.get_socket();
            send!(wl_output::geometry(&socket,
                                      oid,
                                      info.area.pos.x as i32,
                                      info.area.pos.y as i32,
                                      info.physical_size.width as i32,
                                      info.physical_size.height as i32,
                                      wl_output::subpixel::UNKNOWN as i32,
                                      &info.make,
                                      &info.model,
                                      wl_output::transform::NORMAL as i32));

            send!(wl_output::mode(&socket,
                                  oid,
                                  wl_output::mode::CURRENT as u32,
                                  info.area.size.width as i32,
                                  info.area.size.height as i32,
                                  info.refresh_rate as i32));

            send!(wl_output::scale(&socket, oid, 1));
            send!(wl_output::done(&socket, oid));

            proxy.relate_output_oid_with_id(oid, info.id);
        }

        Output {}
    }

    fn new_object(oid: ObjectId, proxy_ref: ProxyRef, info: OutputInfo) -> Box<Object> {
        Box::new(Handler::<_, wl_output::Dispatcher>::new(Self::new(oid, proxy_ref, info)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_output::Interface for Output {
    fn release(&mut self, this_object_id: ObjectId, _bundle: &mut Bundle) -> Task {
        Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------
