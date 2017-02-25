// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `zxdg_shell_v6`, `zxdg_surface_v6` and `zxdg_toplevel_v6` objects.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_shell_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_surface_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_toplevel_v6;

use qualia::show_reason;

use facade::{Facade, ShellSurfaceOid};
use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_shell_v6` object.
#[allow(dead_code)]
struct ZxdgShellV6 {
    oid: wl::common::ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(zxdg_shell_v6::NAME,
                zxdg_shell_v6::VERSION,
                ZxdgShellV6::new_object)
}

// -------------------------------------------------------------------------------------------------

impl ZxdgShellV6 {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        ZxdgShellV6 {
            oid: oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, zxdg_shell_v6::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_shell_v6::Interface for ZxdgShellV6 {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::None
    }

    fn create_positioner(&mut self,
                         this_object_id: wl::common::ObjectId,
                         socket: &mut wl::server::ClientSocket,
                         new_positioner_id: wl::common::ObjectId)
                         -> wl::server::Task {
        wl::server::Task::None
    }

    fn get_xdg_surface(&mut self,
                       this_object_id: wl::common::ObjectId,
                       socket: &mut wl::server::ClientSocket,
                       new_surface_id: wl::common::ObjectId,
                       surface: wl::common::ObjectId)
                       -> wl::server::Task {
        let surface = ZxdgSurfaceV6::new_object(new_surface_id, surface, self.proxy.clone());
        wl::server::Task::Create {
            id: new_surface_id,
            object: surface,
        }
    }

    fn pong(&mut self,
            this_object_id: wl::common::ObjectId,
            socket: &mut wl::server::ClientSocket,
            serial: u32)
            -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_surface_v6` object.
struct ZxdgSurfaceV6 {
    oid: wl::common::ObjectId,
    surface_oid: wl::common::ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl ZxdgSurfaceV6 {
    fn new(oid: wl::common::ObjectId,
           surface_oid: wl::common::ObjectId,
           proxy_ref: ProxyRef)
           -> Self {
        ZxdgSurfaceV6 {
            oid: oid,
            surface_oid: surface_oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: wl::common::ObjectId,
                  surface_oid: wl::common::ObjectId,
                  proxy: ProxyRef)
                  -> Box<wl::server::Object> {
        Box::new(Handler::<_, zxdg_surface_v6::Dispatcher>::new(Self::new(oid, surface_oid, proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_surface_v6::Interface for ZxdgSurfaceV6 {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::None
    }

    fn get_toplevel(&mut self,
                    this_object_id: wl::common::ObjectId,
                    socket: &mut wl::server::ClientSocket,
                    new_toplevel_id: wl::common::ObjectId)
                    -> wl::server::Task {
        let toplevel = ZxdgToplevelV6::new_object(new_toplevel_id,
                                                  self.surface_oid,
                                                  self.oid,
                                                  self.proxy.clone());
        wl::server::Task::Create {
            id: new_toplevel_id,
            object: toplevel,
        }
    }

    fn get_popup(&mut self,
                 this_object_id: wl::common::ObjectId,
                 socket: &mut wl::server::ClientSocket,
                 new_popup_id: wl::common::ObjectId,
                 parent: wl::common::ObjectId,
                 positioner: wl::common::ObjectId)
                 -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_window_geometry(&mut self,
                           this_object_id: wl::common::ObjectId,
                           socket: &mut wl::server::ClientSocket,
                           x: i32,
                           y: i32,
                           width: i32,
                           height: i32)
                           -> wl::server::Task {
        wl::server::Task::None
    }

    fn ack_configure(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket,
                     serial: u32)
                     -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_toplevel_v6` object.
struct ZxdgToplevelV6 {
}

// -------------------------------------------------------------------------------------------------

impl ZxdgToplevelV6 {
    fn new(oid: wl::common::ObjectId,
           surface_oid: wl::common::ObjectId,
           shell_surface_oid: wl::common::ObjectId,
           proxy_ref: ProxyRef)
           -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            proxy.show(surface_oid,
                       ShellSurfaceOid::ZxdgToplevelV6(shell_surface_oid, oid),
                       show_reason::IN_SHELL);
        }

        ZxdgToplevelV6 {}
    }

    fn new_object(oid: wl::common::ObjectId,
                  surface_oid: wl::common::ObjectId,
                  shell_surface_oid: wl::common::ObjectId,
                  proxy_ref: ProxyRef)
                  -> Box<wl::server::Object> {
        let toplevel = Self::new(oid, surface_oid, shell_surface_oid, proxy_ref);
        Box::new(Handler::<_, zxdg_toplevel_v6::Dispatcher>::new(toplevel))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_toplevel_v6::Interface for ZxdgToplevelV6 {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_parent(&mut self,
                  this_object_id: wl::common::ObjectId,
                  socket: &mut wl::server::ClientSocket,
                  parent: wl::common::ObjectId)
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

    fn set_app_id(&mut self,
                  this_object_id: wl::common::ObjectId,
                  socket: &mut wl::server::ClientSocket,
                  app_id: String)
                  -> wl::server::Task {
        wl::server::Task::None
    }

    fn show_window_menu(&mut self,
                        this_object_id: wl::common::ObjectId,
                        socket: &mut wl::server::ClientSocket,
                        seat: wl::common::ObjectId,
                        serial: u32,
                        x: i32,
                        y: i32)
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

    fn set_max_size(&mut self,
                    this_object_id: wl::common::ObjectId,
                    socket: &mut wl::server::ClientSocket,
                    width: i32,
                    height: i32)
                    -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_min_size(&mut self,
                    this_object_id: wl::common::ObjectId,
                    socket: &mut wl::server::ClientSocket,
                    width: i32,
                    height: i32)
                    -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_maximized(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket)
                     -> wl::server::Task {
        wl::server::Task::None
    }

    fn unset_maximized(&mut self,
                       this_object_id: wl::common::ObjectId,
                       socket: &mut wl::server::ClientSocket)
                       -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_fullscreen(&mut self,
                      this_object_id: wl::common::ObjectId,
                      socket: &mut wl::server::ClientSocket,
                      output: wl::common::ObjectId)
                      -> wl::server::Task {
        wl::server::Task::None
    }

    fn unset_fullscreen(&mut self,
                        this_object_id: wl::common::ObjectId,
                        socket: &mut wl::server::ClientSocket)
                        -> wl::server::Task {
        wl::server::Task::None
    }

    fn set_minimized(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket)
                     -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------
