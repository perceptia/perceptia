// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_shm`, `wl_shm_pool` and `wl_buffer` objects.

// TODO: Consider if it would be simpler to have all objects in one handler.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_display;
use skylane_protocols::server::wayland::wl_shm;
use skylane_protocols::server::wayland::wl_shm_pool;
use skylane_protocols::server::wayland::wl_buffer;

use qualia::{MappedMemory, MemoryPoolId, MemoryViewId};

use global::Global;
use proxy::ProxyRef;
use facade::Facade;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shm` object.
#[allow(dead_code)]
struct Shm {
    oid: wl::common::ObjectId,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_shm::NAME, wl_shm::VERSION, Shm::new_object)
}

// -------------------------------------------------------------------------------------------------

impl Shm {
    /// Creates new `Shm` and posts supprted formats.
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let mut socket = proxy_ref.borrow().get_socket();
            send!(wl_shm::format(&mut socket, oid, wl_shm::format::XRGB8888));
            send!(wl_shm::format(&mut socket, oid, wl_shm::format::ARGB8888));
        }

        Shm {
            oid: oid,
            proxy: proxy_ref,
        }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_shm::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shm::Interface for Shm {
    fn create_pool(&mut self,
                   this_object_id: wl::common::ObjectId,
                   socket: &mut wl::server::ClientSocket,
                   new_pool_id: wl::common::ObjectId,
                   fd: i32,
                   size: i32)
                   -> wl::server::Task {
        match MappedMemory::new(fd, size as usize) {
            Ok(memory) => {
                let mut proxy = self.proxy.borrow_mut();
                let mpid = proxy.create_memory_pool(memory);
                let pool = ShmPool::new_object(self.proxy.clone(), mpid, fd, size as usize);
                wl::server::Task::Create {
                    id: new_pool_id,
                    object: pool,
                }
            }
            Err(err) => {
                log_error!("Failed to map memory! {:?}", err);
                wl::server::Task::None
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_shm_pool` object.
struct ShmPool {
    proxy: ProxyRef,
    mpid: MemoryPoolId,
    fd: i32,
    size: usize,
}

// -------------------------------------------------------------------------------------------------

impl ShmPool {
    fn new(proxy_ref: ProxyRef, mpid: MemoryPoolId, fd: i32, size: usize) -> Self {
        ShmPool {
            proxy: proxy_ref,
            mpid: mpid,
            fd: fd,
            size: size,
        }
    }

    fn new_object(proxy_ref: ProxyRef,
                  mpid: MemoryPoolId,
                  fd: i32,
                  size: usize)
                  -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_shm_pool::Dispatcher>::new(Self::new(proxy_ref, mpid, fd, size)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_shm_pool::Interface for ShmPool {
    fn create_buffer(&mut self,
                     this_object_id: wl::common::ObjectId,
                     socket: &mut wl::server::ClientSocket,
                     new_buffer_id: wl::common::ObjectId,
                     offset: i32,
                     width: i32,
                     height: i32,
                     stride: i32,
                     format: u32)
                     -> wl::server::Task {
        let mut proxy = self.proxy.borrow_mut();
        if let Some(mvid) = proxy.create_memory_view(self.mpid,
                                                     new_buffer_id,
                                                     offset as usize,
                                                     width as usize,
                                                     height as usize,
                                                     stride as usize) {
            let buffer = ShmBuffer::new_object(self.proxy.clone(), mvid);
            wl::server::Task::Create {
                id: new_buffer_id,
                object: buffer,
            }
        } else {
            wl::server::Task::None
        }
    }

    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        let mut proxy = self.proxy.borrow_mut();
        proxy.destroy_memory_pool(self.mpid);
        send!(wl_display::delete_id(&mut proxy.get_socket(),
                                    wl::common::DISPLAY_ID,
                                    this_object_id.get_value()));
        wl::server::Task::None
    }

    fn resize(&mut self,
              this_object_id: wl::common::ObjectId,
              socket: &mut wl::server::ClientSocket,
              size: i32)
              -> wl::server::Task {
        match MappedMemory::new(self.fd, size as usize) {
            Ok(memory) => {
                let mut proxy = self.proxy.borrow_mut();
                proxy.replace_memory_pool(self.mpid, memory);
                self.size = size as usize;
            }
            Err(err) => {
                log_error!("Failed to map memory! {:?}", err);
            }
        }
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_buffer` object.
#[allow(dead_code)]
struct ShmBuffer {
    proxy: ProxyRef,
    mvid: MemoryViewId,
}

// -------------------------------------------------------------------------------------------------

impl ShmBuffer {
    fn new(proxy_ref: ProxyRef, mvid: MemoryViewId) -> Self {
        ShmBuffer {
            proxy: proxy_ref,
            mvid: mvid,
        }
    }

    fn new_object(proxy_ref: ProxyRef, mvid: MemoryViewId) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_buffer::Dispatcher>::new(Self::new(proxy_ref, mvid)))
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl wl_buffer::Interface for ShmBuffer {
    fn destroy(&mut self,
               this_object_id: wl::common::ObjectId,
               socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::None
    }
}

// -------------------------------------------------------------------------------------------------
