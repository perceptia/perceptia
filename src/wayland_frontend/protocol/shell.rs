// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of Wayland `wl_shell` and `wl_shell_surface` objects.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_shell;
use skylane_protocols::server::wayland::wl_shell_surface;

use qualia::show_reason;

use facade::{Facade, ShellSurfaceOid};
use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

enum SurfaceType {
    None,
    Toplevel,
    Popup,
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shell` object.
struct Shell {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_shell::NAME, wl_shell::VERSION, Box::new(Shell::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Shell {
    fn new(proxy_ref: ProxyRef) -> Self {
        Shell { proxy: proxy_ref }
    }

    fn new_object(_oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_shell::Dispatcher>::new(Self::new(proxy_ref)))
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
        let surface = Surface::new_object(new_shell_surface_oid, surface_oid, self.proxy.clone());
        wl::server::Task::Create {
            id: new_shell_surface_oid,
            object: surface,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shell_surface` object.
struct Surface {
    surface_oid: wl::common::ObjectId,
    proxy: ProxyRef,
    surface_type: SurfaceType,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    fn new(_oid: wl::common::ObjectId,
           surface_oid: wl::common::ObjectId,
           proxy_ref: ProxyRef)
           -> Self {
        Surface {
            surface_oid: surface_oid,
            proxy: proxy_ref,
            surface_type: SurfaceType::None,
        }
    }

    fn new_object(oid: wl::common::ObjectId,
                  surface_oid: wl::common::ObjectId,
                  proxy_ref: ProxyRef)
                  -> Box<wl::server::Object> {
        let surface = Self::new(oid, surface_oid, proxy_ref);
        Box::new(Handler::<_, wl_shell_surface::Dispatcher>::new(surface))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shell_surface::Interface for Surface {
    fn pong(&mut self,
            this_object_id: wl::common::ObjectId,
            socket: &mut wl::server::ClientSocket,
            serial: u32)
            -> wl::server::Task {
        wl::server::Task::None
    }

    fn move_(&mut self,
             this_object_id: wl::common::ObjectId,
             socket: &mut wl::server::ClientSocket,
             seat: wl::common::ObjectId,
             serial: u32)
             -> wl::server::Task {
        wl::server::Task::None
    }

    fn resize(&mut self,
              this_object_id: wl::common::ObjectId,
              socket: &mut wl::server::ClientSocket,
              seat: wl::common::ObjectId,
              serial: u32,
              edges: u32)
              -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_toplevel(&mut self,
                    this_object_id: wl::common::ObjectId,
                    socket: &mut wl::server::ClientSocket)
                    -> wl::server::Task {
        let mut proxy = self.proxy.borrow_mut();

        // NOTE: Workaround for Qt. It first sets menus as toplevel and later as pop-up.
        //       Here opposite situation added for symmetry.
        match self.surface_type {
            SurfaceType::Popup => proxy.unrelate(self.surface_oid),
            _ => {}
        }
        self.surface_type = SurfaceType::Toplevel;

        proxy.show(self.surface_oid, ShellSurfaceOid::Shell(this_object_id), show_reason::IN_SHELL);
        wl::server::Task::None
    }

    // TODO: Currently transient and pop-up are handled the same way.
    fn set_transient(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket,
                     parent_surface_oid: wl::common::ObjectId,
                     x: i32,
                     y: i32,
                     flags: u32)
                     -> wl::server::Task {
        let mut proxy = self.proxy.borrow_mut();

        // NOTE: Workaround for Qt. It first sets menus as toplevel and later as pop-up.
        //       Here opposite situation added for symmetry.
        match self.surface_type {
            SurfaceType::Toplevel => proxy.hide(self.surface_oid, show_reason::IN_SHELL),
            _ => {}
        }
        self.surface_type = SurfaceType::Popup;

        proxy.relate(self.surface_oid, parent_surface_oid);
        proxy.set_relative_position(self.surface_oid, x as isize, y as isize);
        wl::server::Task::None
    }

    fn set_fullscreen(&mut self,
                      this_object_id: wl::common::ObjectId,
                      socket: &mut wl::server::ClientSocket,
                      method: u32,
                      framerate: u32,
                      output: wl::common::ObjectId)
                      -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_popup(&mut self,
                 this_object_id: wl::common::ObjectId,
                 socket: &mut wl::server::ClientSocket,
                 seat: wl::common::ObjectId,
                 serial: u32,
                 parent_surface_oid: wl::common::ObjectId,
                 x: i32,
                 y: i32,
                 flags: u32)
                 -> wl::server::Task {
        let mut proxy = self.proxy.borrow_mut();

        // NOTE: Workaround for Qt. It first sets menus as toplevel and later as pop-up.
        match self.surface_type {
            SurfaceType::Toplevel => proxy.hide(self.surface_oid, show_reason::IN_SHELL),
            _ => {}
        }
        self.surface_type = SurfaceType::Popup;

        proxy.relate(self.surface_oid, parent_surface_oid);
        proxy.set_relative_position(self.surface_oid, x as isize, y as isize);
        wl::server::Task::None
    }

    fn set_maximized(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket,
                     output: wl::common::ObjectId)
                     -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_title(&mut self,
                 this_object_id: wl::common::ObjectId,
                 socket: &mut wl::server::ClientSocket,
                 title: String)
                 -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_class(&mut self,
                 this_object_id: wl::common::ObjectId,
                 socket: &mut wl::server::ClientSocket,
                 class: String)
                 -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------
