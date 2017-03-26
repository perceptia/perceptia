// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_subcompositor` and `wl_subsurface` objects.

// TODO: Finish implementation of subcompositor.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_subcompositor;
use skylane_protocols::server::wayland::wl_subsurface;

use global::Global;
use facade::Facade;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_subcompositor` object.
struct Subcompositor {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_subcompositor::NAME,
                wl_subcompositor::VERSION,
                Box::new(Subcompositor::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Subcompositor {
    fn new(proxy_ref: ProxyRef) -> Self {
        Subcompositor { proxy: proxy_ref }
    }

    fn new_object(_oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_subcompositor::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_subcompositor::Interface for Subcompositor {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::Destroy { id: this_object_id }
    }

    fn get_subsurface(&mut self,
                      _this_object_id: wl::common::ObjectId,
                      _socket: &mut wl::server::ClientSocket,
                      new_subsurface_oid: wl::common::ObjectId,
                      surface_oid: wl::common::ObjectId,
                      parent_oid: wl::common::ObjectId)
                      -> wl::server::Task {
        let subsurface = Subsurface::new_object(surface_oid, parent_oid, self.proxy.clone());
        wl::server::Task::Create {
            id: new_subsurface_oid,
            object: subsurface,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_subsurface` object.
struct Subsurface {
    surface_oid: wl::common::ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Subsurface {
    fn new(surface_oid: wl::common::ObjectId,
           parent_surface_oid: wl::common::ObjectId,
           proxy_ref: ProxyRef)
           -> Self
    {
        {
            let proxy = proxy_ref.borrow_mut();
            proxy.relate(surface_oid, parent_surface_oid);
        }
        Subsurface {
            surface_oid: surface_oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(surface_oid: wl::common::ObjectId,
                  parent_surface_oid: wl::common::ObjectId,
                  proxy_ref: ProxyRef)
                  -> Box<wl::server::Object> {
        let subsurface = Self::new(surface_oid, parent_surface_oid, proxy_ref);
        Box::new(Handler::<_, wl_subsurface::Dispatcher>::new(subsurface))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_subsurface::Interface for Subsurface {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        let proxy = self.proxy.borrow_mut();
        proxy.unrelate(self.surface_oid);
        wl::server::Task::Destroy { id: this_object_id }
    }

    fn set_position(&mut self,
                    _this_object_id: wl::common::ObjectId,
                    _socket: &mut wl::server::ClientSocket,
                    x: i32,
                    y: i32)
                    -> wl::server::Task {
        let proxy = self.proxy.borrow_mut();
        proxy.set_relative_position(self.surface_oid, x as isize, y as isize);
        wl::server::Task::None
    }

    fn place_above(&mut self,
                   _this_object_id: wl::common::ObjectId,
                   _socket: &mut wl::server::ClientSocket,
                   sibling: wl::common::ObjectId)
                   -> wl::server::Task {
        wl::server::Task::None
    }

    fn place_below(&mut self,
                   this_object_id: wl::common::ObjectId,
                   socket: &mut wl::server::ClientSocket,
                   sibling: wl::common::ObjectId)
                   -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_sync(&mut self,
                this_object_id: wl::common::ObjectId,
                socket: &mut wl::server::ClientSocket)
                -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_desync(&mut self,
                  this_object_id: wl::common::ObjectId,
                  socket: &mut wl::server::ClientSocket)
                  -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------
