// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of Wayland `wl_shell` object.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_shell;

use qualia::show_reason;

use facade::{Facade, ShellSurfaceOid};
use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shell` object.
#[allow(dead_code)]
struct Shell {
    oid: wl::common::ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_shell::NAME, wl_shell::VERSION, Shell::new_object)
}

// -------------------------------------------------------------------------------------------------

impl Shell {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        Shell {
            oid: oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_shell::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shell::Interface for Shell {
    fn get_shell_surface(&mut self,
                         this_object_id: wl::common::ObjectId,
                         socket: &mut wl::server::ClientSocket,
                         new_shell_surface_oid: wl::common::ObjectId,
                         surface_oid: wl::common::ObjectId)
                         -> wl::server::Task {
        // FIXME: Finish implementation of Wayland shell.
        let mut proxy = self.proxy.borrow_mut();
        proxy.show(surface_oid,
                   ShellSurfaceOid::Shell(this_object_id),
                   show_reason::IN_SHELL);
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------
