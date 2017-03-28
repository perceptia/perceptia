// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of Wayland `wl_registry` object.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_display;
use skylane_protocols::server::wayland::wl_registry;

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_registry` object.
pub struct Registry {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Registry {
    /// Creates new `Registry` and posts curretn globals.
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let proxy = proxy_ref.borrow();
            let socket = proxy.get_socket();
            for (name, global) in proxy.get_globals() {
                send!(wl_registry::global(&socket, oid, *name, global.interface, global.version));
            }
        }

        Registry { proxy: proxy_ref }
    }

    pub fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_registry::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_registry::Interface for Registry {
    fn bind(&mut self,
            this_object_id: wl::common::ObjectId,
            client: &mut wl::server::ClientSocket,
            name: u32,
            interface: String,
            version: u32,
            new_object_id: wl::common::ObjectId)
            -> wl::server::Task {
        let proxy = self.proxy.borrow();
        match {
            if let Some(global) = proxy.get_globals().get(&name) {
                if global.interface != interface {
                    Err(format!("Interface names do not match. Expected '{}', received: '{}'.",
                                global.interface,
                                interface))
                } else if version == 0 {
                    Err(format!("Invalid version for global '{}': 0 is not valid version.",
                                interface))
                } else if global.version < version {
                    Err(format!("Invalid version for global '{}': server has: {}, client wanted: {}.",
                                interface,
                                global.version,
                                version))
                } else {
                    Ok(global)
                }
            } else {
                Err(format!("Requested for not registered global '{}' ({})",
                            interface,
                            name))
            }
        } {
            Ok(global) => {
                let object = global.construct(new_object_id, self.proxy.clone());
                wl::server::Task::Create {
                    id: new_object_id,
                    object: object,
                }
            }
            Err(msg) => {
                log_warn1!("{}", msg);
                send!(wl_display::error(client,
                                        wl::common::DISPLAY_ID,
                                        this_object_id,
                                        name,
                                        &msg));
                wl::server::Task::None
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
