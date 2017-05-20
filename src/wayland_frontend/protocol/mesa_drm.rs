// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of `drm` objects from Mesa project.
//!
//! This protocol is part of internal implementation of mesa, but implementing it here is the
//! simplest way to make EGL applications talk to `perceptia`.

use std::rc::Rc;
use std::os::unix::io::RawFd;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::drm::wl_drm;
use skylane_protocols::server::wayland::wl_buffer;

use qualia::{EglAttributes, PixelFormat, EglImageId};

use global::Global;
use facade::Facade;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Mesa `drm` object.
struct Drm {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_drm::NAME, wl_drm::VERSION, Rc::new(Drm::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Drm {
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let proxy = proxy_ref.borrow();
            if let Some(path) = proxy.get_drm_device_path() {
                send!(wl_drm::device(&proxy.get_socket(), oid, path.to_str().unwrap()));
            }
            send!(wl_drm::format(&proxy.get_socket(), oid, wl_drm::format::ARGB8888));
            send!(wl_drm::format(&proxy.get_socket(), oid, wl_drm::format::XRGB8888));
        }

        Drm { proxy: proxy_ref }
    }

    fn new_object(oid: ObjectId, _version: u32, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_drm::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_drm::Interface for Drm {
    fn authenticate(&mut self,
                    this_object_id: ObjectId,
                    _bundle: &mut Bundle,
                    id: u32)
                    -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.authenticate_drm_device(id);
        send!(wl_drm::authenticated(&proxy.get_socket(), this_object_id));
        Task::None
    }

    fn create_buffer(&mut self,
                     _this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     buffer_oid: ObjectId,
                     name: u32,
                     width: i32,
                     height: i32,
                     stride: u32,
                     format: u32)
                     -> Task {
        let pixel_format = {
            match format {
                wl_drm::format::XRGB8888 => PixelFormat::XRGB8888,
                wl_drm::format::ARGB8888 => PixelFormat::XRGB8888,
                wl_drm::format::XBGR8888 => PixelFormat::XBGR8888,
                wl_drm::format::ABGR8888 => PixelFormat::XBGR8888,
                _ => {
                    log_warn3!("Unsupported format: {}", format);
                    return Task::None;
                }
            }
        };

        let eiid = {
            let mut proxy = self.proxy.borrow_mut();
            let attrs = EglAttributes::new(name, width, height, stride, pixel_format);
            proxy.create_egl_image(buffer_oid, attrs)
        };

        if let Some(eiid) = eiid {
            let buffer = DrmBuffer::new_object(eiid, self.proxy.clone());
            bundle.add_object(buffer_oid, buffer);
        }

        Task::None
    }

    #[allow(unused_variables)]
    fn create_planar_buffer(&mut self,
                            this_object_id: ObjectId,
                            bundle: &mut Bundle,
                            id: ObjectId,
                            name: u32,
                            width: i32,
                            height: i32,
                            format: u32,
                            offset0: i32,
                            stride0: i32,
                            offset1: i32,
                            stride1: i32,
                            offset2: i32,
                            stride2: i32)
                            -> Task {
        log_error!("wl_drm::create_planar_buffer is not implemented yet");
        Task::None
    }

    #[allow(unused_variables)]
    fn create_prime_buffer(&mut self,
                           this_object_id: ObjectId,
                           bundle: &mut Bundle,
                           id: ObjectId,
                           name: RawFd,
                           width: i32,
                           height: i32,
                           format: u32,
                           offset0: i32,
                           stride0: i32,
                           offset1: i32,
                           stride1: i32,
                           offset2: i32,
                           stride2: i32)
                           -> Task {
        log_error!("wl_drm::create_prime_buffer is not implemented yet");
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_buffer` object.
struct DrmBuffer {
    proxy: ProxyRef,
    eiid: EglImageId,
}

// -------------------------------------------------------------------------------------------------

impl DrmBuffer {
    fn new(eiid: EglImageId, proxy_ref: ProxyRef) -> Self {
        DrmBuffer {
            proxy: proxy_ref,
            eiid: eiid,
        }
    }

    fn new_object(eiid: EglImageId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_buffer::Dispatcher>::new(Self::new(eiid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_buffer::Interface for DrmBuffer {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.destroy_egl_image(self.eiid);
        bundle.remove_object(this_object_id);
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
