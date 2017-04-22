// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of Wayland `wl_shell` and `wl_shell_surface` objects.

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
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
    Global::new(wl_shell::NAME, wl_shell::VERSION, Rc::new(Shell::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Shell {
    fn new(proxy_ref: ProxyRef) -> Self {
        Shell { proxy: proxy_ref }
    }

    fn new_object(_oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_shell::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shell::Interface for Shell {
    fn get_shell_surface(&mut self,
                         this_object_id: ObjectId,
                         bundle: &mut Bundle,
                         new_shell_surface_oid: ObjectId,
                         surface_oid: ObjectId)
                         -> Task {
        let surface = Surface::new_object(new_shell_surface_oid, surface_oid, self.proxy.clone());
        Task::Create {
            id: new_shell_surface_oid,
            object: surface,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shell_surface` object.
struct Surface {
    surface_oid: ObjectId,
    proxy: ProxyRef,
    surface_type: SurfaceType,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    fn new(_oid: ObjectId, surface_oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        Surface {
            surface_oid: surface_oid,
            proxy: proxy_ref,
            surface_type: SurfaceType::None,
        }
    }

    fn new_object(oid: ObjectId, surface_oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        let surface = Self::new(oid, surface_oid, proxy_ref);
        Box::new(Handler::<_, wl_shell_surface::Dispatcher>::new(surface))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shell_surface::Interface for Surface {
    fn pong(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, serial: u32) -> Task {
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

    fn set_toplevel(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let mut proxy = self.proxy.borrow_mut();

        // NOTE: Workaround for Qt. It first sets menus as toplevel and later as pop-up.
        //       Here opposite situation added for symmetry.
        match self.surface_type {
            SurfaceType::Popup => proxy.unrelate(self.surface_oid),
            _ => {}
        }
        self.surface_type = SurfaceType::Toplevel;

        proxy.show(self.surface_oid, ShellSurfaceOid::Shell(this_object_id), show_reason::IN_SHELL);
        Task::None
    }

    // TODO: Currently transient and pop-up are handled the same way.
    fn set_transient(&mut self,
                     this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     parent_surface_oid: ObjectId,
                     x: i32,
                     y: i32,
                     flags: u32)
                     -> Task {
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
        Task::None
    }

    fn set_fullscreen(&mut self,
                      this_object_id: ObjectId,
                      bundle: &mut Bundle,
                      method: u32,
                      framerate: u32,
                      output: ObjectId)
                      -> Task {
        Task::None
    }

    fn set_popup(&mut self,
                 this_object_id: ObjectId,
                 bundle: &mut Bundle,
                 seat: ObjectId,
                 serial: u32,
                 parent_surface_oid: ObjectId,
                 x: i32,
                 y: i32,
                 flags: u32)
                 -> Task {
        let mut proxy = self.proxy.borrow_mut();

        // NOTE: Workaround for Qt. It first sets menus as toplevel and later as pop-up.
        match self.surface_type {
            SurfaceType::Toplevel => proxy.hide(self.surface_oid, show_reason::IN_SHELL),
            _ => {}
        }
        self.surface_type = SurfaceType::Popup;

        proxy.relate(self.surface_oid, parent_surface_oid);
        proxy.set_relative_position(self.surface_oid, x as isize, y as isize);
        Task::None
    }

    fn set_maximized(&mut self,
                     this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     output: ObjectId)
                     -> Task {
        Task::None
    }

    fn set_title(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, title: String) -> Task {
        Task::None
    }

    fn set_class(&mut self, this_object_id: ObjectId, bundle: &mut Bundle, class: String) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
