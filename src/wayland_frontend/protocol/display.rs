// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of Wayland `wl_display` object.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_display;
use skylane_protocols::server::wayland::wl_callback;

use proxy::ProxyRef;
use protocol::registry::Registry;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_display` object.
pub struct Display {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Display {
    fn new(proxy: ProxyRef) -> Self {
        Display { proxy: proxy }
    }

    pub fn new_object(proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_display::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_display::Interface for Display {
    fn sync(&mut self,
            this_object_id: wl::common::ObjectId,
            socket: &mut wl::server::ClientSocket,
            callback: wl::common::ObjectId)
            -> wl::server::Task {
        let serial = socket.get_next_serial();
        send!(wl_callback::done(socket, callback, serial));
        send!(wl_display::delete_id(socket, this_object_id, callback.get_value()));
        wl::server::Task::None
    }

    fn get_registry(&mut self,
                    _this_object_id: wl::common::ObjectId,
                    _socket: &mut wl::server::ClientSocket,
                    new_registry_id: wl::common::ObjectId)
                    -> wl::server::Task {
        let registry = Registry::new_object(new_registry_id, self.proxy.clone());
        wl::server::Task::Create {
            id: new_registry_id,
            object: registry,
        }
    }
}

// -------------------------------------------------------------------------------------------------
