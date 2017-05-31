// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `zxdg_shell_v6`, `zxdg_positioner_v6`, `zxdg_surface_v6`,
//! `zxdg_toplevel_v6` and `zxdg_popup_v6` objects.

// FIXME: Finish implementation of XDG pop-up positioning.

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_shell_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_positioner_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_surface_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_toplevel_v6;
use skylane_protocols::server::xdg_shell_unstable_v6::zxdg_popup_v6;

use qualia::{show_reason, Area};

use facade::{Facade, PositionerInfo, ShellSurfaceOid};
use global::Global;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_shell_v6` object.
#[allow(dead_code)]
struct ZxdgShellV6 {
    oid: ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(zxdg_shell_v6::NAME, zxdg_shell_v6::VERSION, Rc::new(ZxdgShellV6::new_object))
}

// -------------------------------------------------------------------------------------------------

impl ZxdgShellV6 {
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        ZxdgShellV6 {
            oid: oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: ObjectId, _version: u32, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, zxdg_shell_v6::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_shell_v6::Interface for ZxdgShellV6 {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::Destroy { id: this_object_id }
    }

    fn create_positioner(&mut self,
                         this_object_id: ObjectId,
                         bundle: &mut Bundle,
                         new_positioner_oid: ObjectId)
                         -> Task {
        let positioner = ZxdgPositionerV6::new_object(new_positioner_oid, self.proxy.clone());
        Task::Create {
            id: new_positioner_oid,
            object: positioner,
        }
    }

    fn get_xdg_surface(&mut self,
                       this_object_id: ObjectId,
                       bundle: &mut Bundle,
                       new_surface_oid: ObjectId,
                       surface: ObjectId)
                       -> Task {
        let surface = ZxdgSurfaceV6::new_object(new_surface_oid, surface, self.proxy.clone());
        Task::Create {
            id: new_surface_oid,
            object: surface,
        }
    }

    fn pong(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, serial: u32) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_positioner_v6` object.
struct ZxdgPositionerV6 {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl ZxdgPositionerV6 {
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            proxy.set_positioner(oid, PositionerInfo::new());
        }
        ZxdgPositionerV6 { proxy: proxy_ref }
    }

    fn new_object(oid: ObjectId, proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, zxdg_positioner_v6::Dispatcher>::new(Self::new(oid, proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_positioner_v6::Interface for ZxdgPositionerV6 {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.remove_positioner(this_object_id);
        Task::Destroy { id: this_object_id }
    }

    fn set_size(&mut self,
                this_object_id: ObjectId,
                bundle: &mut Bundle,
                width: i32,
                height: i32)
                -> Task {
        let mut proxy = self.proxy.borrow_mut();
        if let Some(mut positioner) = proxy.get_positioner(this_object_id) {
            positioner.size.width = width as usize;
            positioner.size.height = height as usize;
            proxy.set_positioner(this_object_id, positioner);
        }
        Task::None
    }

    fn set_anchor_rect(&mut self,
                       this_object_id: ObjectId,
                       bundle: &mut Bundle,
                       x: i32,
                       y: i32,
                       width: i32,
                       height: i32)
                       -> Task {
        let mut proxy = self.proxy.borrow_mut();
        if let Some(mut positioner) = proxy.get_positioner(this_object_id) {
            positioner.anchor.pos.x = x as isize;
            positioner.anchor.pos.y = y as isize;
            positioner.anchor.size.width = width as usize;
            positioner.anchor.size.height = height as usize;
            proxy.set_positioner(this_object_id, positioner);
        }
        Task::None
    }

    fn set_anchor(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, anchor: u32) -> Task {
        Task::None
    }

    fn set_gravity(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, gravity: u32) -> Task {
        Task::None
    }

    fn set_constraint_adjustment(&mut self,
                                 this_object_id: ObjectId,
                                 bundle: &mut Bundle,
                                 constraint_adjustment: u32)
                                 -> Task {
        Task::None
    }

    fn set_offset(&mut self,
                  this_object_id: ObjectId,
                  _bundle: &mut Bundle,
                  x: i32,
                  y: i32)
                  -> Task {
        let mut proxy = self.proxy.borrow_mut();
        if let Some(mut positioner) = proxy.get_positioner(this_object_id) {
            positioner.offset.x = x as isize;
            positioner.offset.y = y as isize;
            proxy.set_positioner(this_object_id, positioner);
        }
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_surface_v6` object.
struct ZxdgSurfaceV6 {
    oid: ObjectId,
    surface_oid: ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl ZxdgSurfaceV6 {
    fn new(oid: ObjectId, surface_oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        ZxdgSurfaceV6 {
            oid: oid,
            surface_oid: surface_oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: ObjectId, surface_oid: ObjectId, proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, zxdg_surface_v6::Dispatcher>::new(Self::new(oid, surface_oid, proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_surface_v6::Interface for ZxdgSurfaceV6 {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.hide(self.surface_oid, show_reason::IN_SHELL);
        Task::Destroy { id: this_object_id }
    }

    fn get_toplevel(&mut self,
                    this_object_id: ObjectId,
                    bundle: &mut Bundle,
                    new_toplevel_id: ObjectId)
                    -> Task {
        let toplevel = ZxdgToplevelV6::new_object(new_toplevel_id,
                                                  self.surface_oid,
                                                  self.oid,
                                                  self.proxy.clone());
        Task::Create {
            id: new_toplevel_id,
            object: toplevel,
        }
    }

    fn get_popup(&mut self,
                 this_object_id: ObjectId,
                 bundle: &mut Bundle,
                 new_popup_oid: ObjectId,
                 parent_shell_surface_oid: ObjectId,
                 positioner_oid: ObjectId)
                 -> Task {
        let area = {
            let mut proxy = self.proxy.borrow_mut();
            if let Some(positioner) = proxy.get_positioner(positioner_oid) {
                positioner.get_area()
            } else {
                Area::default()
            }
        };

        let popup = ZxdgPopupV6::new_object(self.surface_oid,
                                            parent_shell_surface_oid,
                                            area,
                                            self.proxy.clone());

        // GTK does not map surface without configuring it.
        let serial = bundle.get_socket().get_next_serial();
        send!(zxdg_popup_v6::configure(&bundle.get_socket(),
                                       new_popup_oid,
                                       area.pos.x as i32,
                                       area.pos.y as i32,
                                       area.size.width as i32,
                                       area.size.height as i32));
        send!(zxdg_surface_v6::configure(&bundle.get_socket(), this_object_id, serial));

        Task::Create {
            id: new_popup_oid,
            object: popup,
        }
    }

    fn set_window_geometry(&mut self,
                           this_object_id: ObjectId,
                           bundle: &mut Bundle,
                           x: i32,
                           y: i32,
                           width: i32,
                           height: i32)
                           -> Task {
        Task::None
    }

    fn ack_configure(&mut self,
                     this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     serial: u32)
                     -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_toplevel_v6` object.
struct ZxdgToplevelV6 {}

// -------------------------------------------------------------------------------------------------

impl ZxdgToplevelV6 {
    fn new(oid: ObjectId,
           surface_oid: ObjectId,
           shell_surface_oid: ObjectId,
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

    fn new_object(oid: ObjectId,
                  surface_oid: ObjectId,
                  shell_surface_oid: ObjectId,
                  proxy_ref: ProxyRef)
                  -> Box<Object> {
        let toplevel = Self::new(oid, surface_oid, shell_surface_oid, proxy_ref);
        Box::new(Handler::<_, zxdg_toplevel_v6::Dispatcher>::new(toplevel))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_toplevel_v6::Interface for ZxdgToplevelV6 {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::Destroy { id: this_object_id }
    }

    fn set_parent(&mut self,
                  this_object_id: ObjectId,
                  bundle: &mut Bundle,
                  parent: ObjectId)
                  -> Task {
        Task::None
    }

    fn set_title(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, title: String) -> Task {
        Task::None
    }

    fn set_app_id(&mut self,
                  this_object_id: ObjectId,
                  bundle: &mut Bundle,
                  app_id: String)
                  -> Task {
        Task::None
    }

    fn show_window_menu(&mut self,
                        this_object_id: ObjectId,
                        bundle: &mut Bundle,
                        seat: ObjectId,
                        serial: u32,
                        x: i32,
                        y: i32)
                        -> Task {
        Task::None
    }

    fn move_(&mut self,
             this_object_id: ObjectId,
             bundle: &mut Bundle,
             seat: ObjectId,
             serial: u32)
             -> Task {
        Task::None
    }

    fn resize(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle,
              seat: ObjectId,
              serial: u32,
              edges: u32)
              -> Task {
        Task::None
    }

    fn set_max_size(&mut self,
                    this_object_id: ObjectId,
                    bundle: &mut Bundle,
                    width: i32,
                    height: i32)
                    -> Task {
        Task::None
    }

    fn set_min_size(&mut self,
                    this_object_id: ObjectId,
                    bundle: &mut Bundle,
                    width: i32,
                    height: i32)
                    -> Task {
        Task::None
    }

    fn set_maximized(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::None
    }

    fn unset_maximized(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::None
    }

    fn set_fullscreen(&mut self,
                      this_object_id: ObjectId,
                      bundle: &mut Bundle,
                      output: ObjectId)
                      -> Task {
        Task::None
    }

    fn unset_fullscreen(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::None
    }

    fn set_minimized(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zxdg_popup_v6` object.
struct ZxdgPopupV6 {
    surface_oid: ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl ZxdgPopupV6 {
    fn new(surface_oid: ObjectId,
           parent_shell_surface_oid: ObjectId,
           area: Area,
           proxy_ref: ProxyRef)
           -> Self {
        {
            let proxy = proxy_ref.borrow();
            let parent_surface_oid = proxy.get_surface_oid_for_shell(parent_shell_surface_oid);
            if let Some(parent_surface_oid) = parent_surface_oid {
                proxy.relate(surface_oid, parent_surface_oid);
                proxy.set_relative_position(surface_oid, area.pos.x, area.pos.y);
            }
        }

        ZxdgPopupV6 {
            surface_oid: surface_oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(surface_oid: ObjectId,
                  parent_shell_surface_oid: ObjectId,
                  area: Area,
                  proxy_ref: ProxyRef)
                  -> Box<Object> {
        let popup = Self::new(surface_oid, parent_shell_surface_oid, area, proxy_ref);
        Box::new(Handler::<_, zxdg_popup_v6::Dispatcher>::new(popup))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl zxdg_popup_v6::Interface for ZxdgPopupV6 {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let proxy = self.proxy.borrow();
        proxy.unrelate(self.surface_oid);
        Task::Destroy { id: this_object_id }
    }

    fn grab(&mut self,
            this_object_id: ObjectId,
            bundle: &mut Bundle,
            seat: ObjectId,
            serial: u32)
            -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
