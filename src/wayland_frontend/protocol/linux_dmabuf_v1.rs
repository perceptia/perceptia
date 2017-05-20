// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of `zwp_linux_dmabuf_v1`, `zwp_linux_buffer_params_v1` and `wl_buffer` objects.
//!
//! TODO: Add more validity checks and send reply errors to client.
//!
//! FIXME: There is bug in `nix` crate (issue #464) which prevents reading multiple file
//! descriptors from unix socket. Because `weston-simple-dmabuf-intel` sends requests in one burst
//! we do not receive all of them and import of dmabuf fails.

use std::rc::Rc;
use std::os::unix::io::RawFd;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::linux_dmabuf_unstable_v1::zwp_linux_dmabuf_v1;
use skylane_protocols::server::linux_dmabuf_unstable_v1::zwp_linux_buffer_params_v1;
use skylane_protocols::server::wayland::wl_buffer;

use qualia::{DmabufAttributes, DmabufId, ValidationResult};

use global::Global;
use facade::Facade;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `zwp_linux_dmabuf_v1` object.
struct Dmabuf {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(zwp_linux_dmabuf_v1::NAME,
                zwp_linux_dmabuf_v1::VERSION,
                Rc::new(Dmabuf::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Dmabuf {
    fn new(proxy_ref: ProxyRef) -> Self {
        Dmabuf { proxy: proxy_ref }
    }

    fn new_object(_oid: ObjectId, _version: u32, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, zwp_linux_dmabuf_v1::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl zwp_linux_dmabuf_v1::Interface for Dmabuf {
    fn destroy(&mut self,
               _this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        Task::None
    }

    fn create_params(&mut self,
                     _this_object_id: ObjectId,
                     bundle: &mut Bundle,
                     params_id: ObjectId)
                     -> Task {
        let params = DmabufParams::new_object(params_id, self.proxy.clone());
        bundle.add_object(params_id, params);
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `zwp_linux_buffer_params_v1` object.
struct DmabufParams {
    proxy: ProxyRef,
    attributes: DmabufAttributes,
}

// -------------------------------------------------------------------------------------------------

impl DmabufParams {
    fn new(proxy_ref: ProxyRef) -> Self {
        DmabufParams {
            proxy: proxy_ref,
            attributes: DmabufAttributes::new(),
        }
    }

    fn new_object(_oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, zwp_linux_buffer_params_v1::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl zwp_linux_buffer_params_v1::Interface for DmabufParams {
    fn destroy(&mut self,
               _this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        // Nothing to do
        Task::None
    }

    fn add(&mut self,
           _this_object_id: ObjectId,
           _bundle: &mut Bundle,
           fd: RawFd,
           plane_idx: u32,
           offset: u32,
           stride: u32,
           modifier_hi: u32,
           modifier_lo: u32)
           -> Task {
        let idx = plane_idx as usize;
        let result = self.attributes.add(idx, fd, offset, stride, modifier_hi, modifier_lo);
        handle_validation_result(result);
        Task::None
    }

    fn create(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle,
              width: i32,
              height: i32,
              format: u32,
              flags: u32)
              -> Task {
        self.attributes.create(width, height, format, flags);

        let result = self.attributes.validate();
        if result == ValidationResult::Ok {
            let mut proxy = self.proxy.borrow_mut();
            let oid = bundle.get_next_available_server_object_id();
            if let Some(dbid) = proxy.import_dmabuf(oid, self.attributes.clone()) {
                let buffer = DmabufBuffer::new_object(dbid, self.proxy.clone());
                bundle.add_object(oid, buffer);

                send!(zwp_linux_buffer_params_v1::created(&proxy.get_socket(),
                                                          this_object_id,
                                                          oid));
            } else {
                send!(zwp_linux_buffer_params_v1::failed(&proxy.get_socket(), this_object_id));
            }
        } else {
            handle_validation_result(result);
        }

        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_buffer` object.
struct DmabufBuffer {
    proxy: ProxyRef,
    dbid: DmabufId,
}

// -------------------------------------------------------------------------------------------------

impl DmabufBuffer {
    fn new(dbid: DmabufId, proxy_ref: ProxyRef) -> Self {
        DmabufBuffer {
            proxy: proxy_ref,
            dbid: dbid,
        }
    }

    fn new_object(dbid: DmabufId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_buffer::Dispatcher>::new(Self::new(dbid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_buffer::Interface for DmabufBuffer {
    fn destroy(&mut self, this_object_id: ObjectId, bundle: &mut Bundle) -> Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.destroy_dmabuf(self.dbid);
        bundle.remove_object(this_object_id);
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

fn handle_validation_result(result: ValidationResult) {
    match result {
        ValidationResult::Ok => {}
        ValidationResult::PlaneIdx => {
            log_warn3!("Plane index out of bounds");
        }
        ValidationResult::PlaneSet => {
            log_warn3!("The plane index was already set");
        }
        ValidationResult::Incomplete => {
            log_warn3!("Missing or too many planes to create a buffer");
        }
        ValidationResult::InvalidFormat => {
            log_warn3!("Format not supported");
        }
        ValidationResult::InvalidDimensions => {
            log_warn3!("Invalid width or height");
        }
        ValidationResult::OutOfBounds => {
            log_warn3!("Offset + stride * height goes out of dmabuf bounds");
        }
    }
}

// -------------------------------------------------------------------------------------------------
