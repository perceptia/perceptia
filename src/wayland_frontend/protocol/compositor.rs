// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_compositor`, `wl_surface` and `wl_region` objects.

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_compositor;
use skylane_protocols::server::wayland::wl_surface;
use skylane_protocols::server::wayland::wl_region;

use qualia::{Area, SurfaceId};

use global::Global;
use facade::Facade;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_compositor` object.
struct Compositor {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_compositor::NAME, wl_compositor::VERSION, Rc::new(Compositor::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Compositor {
    fn new(proxy_ref: ProxyRef) -> Self {
        Compositor { proxy: proxy_ref }
    }

    fn new_object(_oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_compositor::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_compositor::Interface for Compositor {
    fn create_surface(&mut self,
                      _this_object_id: ObjectId,
                      _bundle: &mut Bundle,
                      new_surface_id: ObjectId)
                      -> Task {
        let surface = Surface::new_object(new_surface_id, self.proxy.clone());
        Task::Create {
            id: new_surface_id,
            object: surface,
        }
    }

    fn create_region(&mut self,
                     _this_object_id: ObjectId,
                     _bundle: &mut Bundle,
                     new_region_id: ObjectId)
                     -> Task {
        let region = Region::new_object(self.proxy.clone());
        Task::Create {
            id: new_region_id,
            object: region,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_surface` object.
struct Surface {
    proxy: ProxyRef,
    sid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        let sid = {
            let mut proxy = proxy_ref.borrow_mut();
            proxy.create_surface(oid)
        };

        Surface {
            proxy: proxy_ref,
            sid: sid,
        }
    }

    fn new_object(oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_surface::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_surface::Interface for Surface {
    fn destroy(&mut self,
               this_object_id: ObjectId,
               bundle: &mut Bundle)
               -> Task {
        let proxy = self.proxy.borrow();
        proxy.destroy_surface(self.sid);
        Task::Destroy { id: this_object_id }
    }

    fn attach(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle,
              buffer_oid: ObjectId,
              x: i32,
              y: i32)
              -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.attach(buffer_oid, self.sid, x, y);
        Task::None
    }

    fn damage(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle,
              x: i32,
              y: i32,
              width: i32,
              height: i32)
              -> Task {
        Task::None
    }

    fn frame(&mut self,
             this_object_id: ObjectId,
             bundle: &mut Bundle,
             callback: ObjectId)
             -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.set_frame(self.sid, callback);
        Task::None
    }

    fn set_opaque_region(&mut self,
                         this_object_id: ObjectId,
                         bundle: &mut Bundle,
                         region_oid: ObjectId)
                         -> Task {
        let proxy = self.proxy.borrow_mut();
        proxy.set_input_region(self.sid, region_oid);
        Task::None
    }

    fn set_input_region(&mut self,
                        this_object_id: ObjectId,
                        bundle: &mut Bundle,
                        region_oid: ObjectId)
                        -> Task {
        let proxy = self.proxy.borrow_mut();
        proxy.set_input_region(self.sid, region_oid);
        Task::None
    }

    fn commit(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle)
              -> Task {
        let proxy = self.proxy.borrow_mut();
        proxy.commit(self.sid);
        Task::None
    }

    fn set_buffer_transform(&mut self,
                            this_object_id: ObjectId,
                            bundle: &mut Bundle,
                            transform: i32)
                            -> Task {
        Task::None
    }

    fn set_buffer_scale(&mut self,
                        this_object_id: ObjectId,
                        bundle: &mut Bundle,
                        scale: i32)
                        -> Task {
        Task::None
    }

    fn damage_buffer(&mut self,
                     this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     x: i32,
                     y: i32,
                     width: i32,
                     height: i32)
                     -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_region` object.
struct Region {
    proxy: ProxyRef,
    area: Option<Area>,
}

// -------------------------------------------------------------------------------------------------

impl Region {
    fn new(proxy_ref: ProxyRef) -> Self {
        Region {
            proxy: proxy_ref,
            area: None,
        }
    }

    fn new_object(proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_region::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_region::Interface for Region {
    fn destroy(&mut self,
               this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.undefine_region(this_object_id);
        Task::Destroy { id: this_object_id }
    }

    fn add(&mut self,
           this_object_id: ObjectId,
           _bundle: &mut Bundle,
           x: i32,
           y: i32,
           width: i32,
           height: i32)
           -> Task {
        if width > 0 && height > 0 {
            let area = Area::create(x as isize, y as isize, width as usize, height as usize);
            if let Some(ref mut region) = self.area {
                region.inflate(&area);
            } else {
                self.area = Some(area);
            }

            if let Some(region) = self.area {
                let mut proxy = self.proxy.borrow_mut();
                proxy.define_region(this_object_id, region);
            }
        } else {
            log_wayl3!("Received region with non-positive width or height");
        }
        Task::None
    }

    fn subtract(&mut self,
                _this_object_id: ObjectId,
                _bundle: &mut Bundle,
                _x: i32,
                _y: i32,
                _width: i32,
                _height: i32)
                -> Task {
        // Not supported yet
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
