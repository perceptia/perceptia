// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_output` object.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_output;

use qualia::{Area, Position, Size};

use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_output` object.
struct Output {}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_output::NAME,
                wl_output::VERSION,
                Output::new_object)
}

// -------------------------------------------------------------------------------------------------

impl Output {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            // FIXME: Send real data for output.
            let area = Area::new(Position::new(0, 0), Size::new(1366, 768));
            let physical_size = Size::new(200, 150);
            let refresh_rate = 60;
            let name = "out";

            let proxy = proxy_ref.borrow();
            let socket = proxy.get_socket();
            send!(wl_output::geometry(&socket,
                                      oid,
                                      area.pos.x as i32,
                                      area.pos.y as i32,
                                      physical_size.width as i32,
                                      physical_size.height as i32,
                                      wl_output::subpixel::UNKNOWN as i32,
                                      name,
                                      name,
                                      wl_output::transform::NORMAL as i32));

            send!(wl_output::mode(&socket,
                                  oid,
                                  wl_output::mode::CURRENT as u32,
                                  area.size.width as i32,
                                  area.size.height as i32,
                                  refresh_rate));

            send!(wl_output::scale(&socket, oid, 1));
            send!(wl_output::done(&socket, oid));
        }

        Output {}
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_output::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_output::Interface for Output {
    fn release(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------
