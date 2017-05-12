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

//! Access to application from objects.

use std;
use std::collections::HashSet;

use skylane::client as wl;

use defs::OutputInfo;
use store::StoreRef;
use listener::{Dummy, Listener};

// -------------------------------------------------------------------------------------------------

/// Enumeration to implement very simple state machine using `wl_callback`.
pub enum Action {
    GlobalsDone,
    InitDone,
}

// -------------------------------------------------------------------------------------------------

/// Provides unified access to the rest of framework for `skylane` objects.
pub struct Proxy {
    listener: Box<Listener>,
    socket: wl::Socket,
    globals: HashSet<String>,
    outputs: Vec<OutputInfo>,
    store: StoreRef,
}

// -------------------------------------------------------------------------------------------------

impl Proxy {
    /// Constructs new `Proxy`.
    pub fn new(store: StoreRef, socket: wl::Socket) -> Self {
        Proxy {
            listener: Box::new(Dummy {}),
            socket: socket,
            globals: HashSet::new(),
            outputs: Vec::new(),
            store: store,
        }
    }

    /// Sets listener.
    pub fn set_listener(&mut self, listener: Box<Listener>) {
        self.listener = listener;
    }

    /// Returns copy of Wayland client socket.
    pub fn get_socket(&self) -> wl::Socket {
        self.socket.clone()
    }
}

// -------------------------------------------------------------------------------------------------

// Notification handling.
impl Proxy {
    /// Handles notification that server finished advertising globals.
    pub fn globals_done(&mut self) {
        self.listener.globals_done(self.globals.clone());
    }

    /// Handles notification that server finished advertising outputs.
    pub fn init_done(&mut self) {
        self.listener.outputs_done(self.outputs.clone());
        self.listener.init_done();
    }

    /// Handles notification about successful authentication of DRM device.
    pub fn drm_authenticated(&mut self) {
        let device_name = {
            let store = self.store.borrow();
            store.drm_device_name.clone()
        };

        if let Some(device_name) = device_name {
            self.listener.graphics_done(device_name);
        } else {
            self.listener.graphics_failed();
        }
    }

    /// Handles notification that server finished taking screenshot.
    pub fn screenshot_done(&mut self) {
        let buffer = {
            let mut store = self.store.borrow_mut();
            let buffer = if let Some(ref store) = store.screenshot {
                let slice = unsafe { std::slice::from_raw_parts(store.memory, store.size) };
                let mut buffer = Vec::with_capacity(store.size);
                unsafe { buffer.set_len(store.size) };
                buffer.copy_from_slice(slice);
                Some(buffer)
            } else {
                None
            };
            store.screenshot = None;
            buffer
        };

        if let Some(buffer) = buffer {
            self.listener.screenshot_done(buffer);
        } else {
            self.listener.screenshot_failed();
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Getters/setters
impl Proxy {
    /// Adds new global.
    pub fn add_global(&mut self, interface: String) {
        self.globals.insert(interface);
    }

    /// Adds new output.
    pub fn add_output(&mut self, output: OutputInfo) {
        self.outputs.push(output);
    }

    /// Set DRM device name
    pub fn set_drm_device_name(&mut self, name: String) {
        self.store.borrow_mut().drm_device_name = Some(name);
    }

    /// Sets ID of compositor object.
    pub fn set_compositor_oid(&mut self, oid: wl::ObjectId) {
        self.store.borrow_mut().compositor_oid = Some(oid);
    }

    /// Sets ID of shell object.
    pub fn set_shell_oid(&mut self, oid: wl::ObjectId) {
        self.store.borrow_mut().shell_oid = Some(oid);
    }

    /// Sets ID of DRM object.
    pub fn set_drm_oid(&mut self, oid: wl::ObjectId) {
        self.store.borrow_mut().drm_oid = Some(oid);
    }

    /// Sets ID of `dmabuf` object.
    pub fn set_dmabuf_oid(&mut self, oid: wl::ObjectId) {
        self.store.borrow_mut().dmabuf_oid = Some(oid);
    }

    /// Sets ID of shared memory object.
    pub fn set_shm_oid(&mut self, oid: wl::ObjectId) {
        self.store.borrow_mut().shm_oid = Some(oid);
    }

    /// Sets ID of screenshooter object.
    pub fn set_screenshooter_name(&mut self, name: u32) {
        self.store.borrow_mut().screenshooter_name = Some(name);
    }
}

// -------------------------------------------------------------------------------------------------

define_ref!(struct Proxy as ProxyRef);

// -------------------------------------------------------------------------------------------------
