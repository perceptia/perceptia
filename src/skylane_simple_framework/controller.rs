// Copyright 2017 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Controlling application.

use std;
use std::cell::RefCell;
use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::rc::Weak;

use nix;

use skylane::client as wl;
use skylane_protocols::client::wayland::{wl_shm, wl_shm_pool};
use skylane_protocols::client::weston_screenshooter::weston_screenshooter;

use dharma;

use defs::OutputInfo;
use store::{StoreRef, ScreenshotStore};
use proxy::{Proxy, ProxyRef};
use protocol::shm;

// -------------------------------------------------------------------------------------------------

/// Controller allows to control the application and whole framework.
pub struct Controller {
    connection_controller: wl::Controller,
    store: StoreRef,
    dispatcher: dharma::LocalDispatcherController,

    // Controller does not use `Proxy` by itself. It only passes it to new objects.
    proxy: Weak<RefCell<Proxy>>,
}

// -------------------------------------------------------------------------------------------------

impl Controller {
    /// Constructs new `Controller`.
    pub fn new(connection_controller: wl::Controller,
               store: StoreRef,
               dispatcher: dharma::LocalDispatcherController,
               proxy: Weak<RefCell<Proxy>>)
               -> Self {
        Controller {
            connection_controller: connection_controller,
            store: store,
            dispatcher: dispatcher,
            proxy: proxy,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Controller {
    /// Requests server to take screenshot.
    pub fn take_screenshot(&mut self, output: &OutputInfo) {
        let mut store = self.store.borrow_mut();
        let proxy = self.get_proxy();

        let stride = 4 * output.width;
        let size = stride * output.height;
        let output_oid = wl::ObjectId::new(output.id);

        let (fd, path) =
            create_anonymous_file(Some(size), output.id).expect("Create screenshot file");
        let memory = nix::sys::mman::mmap(std::ptr::null_mut(),
                                          size,
                                          nix::sys::mman::PROT_READ | nix::sys::mman::PROT_WRITE,
                                          nix::sys::mman::MAP_SHARED,
                                          fd,
                                          0)
                .expect("Map memory shared for screenshot");

        // Create pool
        let pool_oid = self.connection_controller.get_next_available_client_object_id();
        let pool_object = shm::ShmPool::new_object(proxy.clone());
        self.connection_controller.add_object(pool_oid, pool_object);

        send!(wl_shm::create_pool(&self.connection_controller.get_socket(),
                                  store.shm_oid.unwrap(),
                                  pool_oid,
                                  fd,
                                  size as i32));

        // Create buffer
        let buffer_oid = self.connection_controller.get_next_available_client_object_id();
        let buffer_object = shm::Buffer::new_object(proxy.clone());
        self.connection_controller.add_object(buffer_oid, buffer_object);

        send!(wl_shm_pool::create_buffer(&self.connection_controller.get_socket(),
                                         pool_oid,
                                         buffer_oid,
                                         0, // offset
                                         output.width as i32,
                                         output.height as i32,
                                         stride as i32,
                                         wl_shm::format::ARGB8888));

        // Request screenshot
        send!(weston_screenshooter::shoot(&self.connection_controller.get_socket(),
                                          store.screenshooter_oid.unwrap(),
                                          output_oid,
                                          buffer_oid));

        store.screenshot = Some(ScreenshotStore {
                                    fd: fd,
                                    path: path,
                                    memory: memory as *mut u8,
                                    size: size,
                                    width: output.width,
                                    height: output.height,
                                });
    }

    /// Stops the application.
    pub fn stop(&self) {
        self.dispatcher.stop();
    }
}
// -------------------------------------------------------------------------------------------------

impl Controller {
    /// Gets reference-counted proxy.
    fn get_proxy(&self) -> ProxyRef {
        ProxyRef::transform(self.proxy.upgrade().unwrap())
    }
}

// -------------------------------------------------------------------------------------------------

/// Creates file used to share memory with server.
fn create_anonymous_file(size: Option<usize>, seed: u32) -> nix::Result<(RawFd, PathBuf)> {
    let mut path = PathBuf::new();
    path.push(std::env::var("XDG_RUNTIME_DIR").unwrap_or("/tmp".to_owned()));
    path.push(format!("perceptiactl-screenshot-{}", seed));

    match nix::fcntl::open(&path,
                           nix::fcntl::O_RDWR | nix::fcntl::O_CREAT,
                           nix::sys::stat::Mode::empty()) {
        Ok(fd) => {
            if let Some(size) = size {
                nix::unistd::ftruncate(fd, size as i64)?;
            }
            Ok((fd, path))
        }
        Err(err) => Err(err),
    }
}

// -------------------------------------------------------------------------------------------------
