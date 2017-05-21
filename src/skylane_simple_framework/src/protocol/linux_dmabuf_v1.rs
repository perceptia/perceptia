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

//! Implementation of Wayland `linux_dmabuf` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::linux_dmabuf_unstable_v1::zwp_linux_dmabuf_v1;
use skylane_protocols::client::linux_dmabuf_unstable_v1::zwp_linux_buffer_params_v1;
use skylane_protocols::client::wayland::wl_buffer;

use common;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `linux_dmabuf` object.
pub struct Dmabuf {}

// -------------------------------------------------------------------------------------------------

impl Dmabuf {
    fn new() -> Self {
        Dmabuf {}
    }

    pub fn new_object() -> Box<Object> {
        Box::new(Handler::<_, zwp_linux_dmabuf_v1::Dispatcher>::new(Self::new()))
    }
}

// -------------------------------------------------------------------------------------------------

impl zwp_linux_dmabuf_v1::Interface for Dmabuf {
    fn format(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle, _format: u32) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `linux_dmabuf_params` object.
pub struct DmabufParams {
    proxy: ProxyRef,
    compositor_oid: ObjectId,
    shell_oid: ObjectId,
    width: usize,
    height: usize,
}

// -------------------------------------------------------------------------------------------------

impl DmabufParams {
    fn new(proxy: ProxyRef,
           compositor_oid: ObjectId,
           shell_oid: ObjectId,
           width: usize,
           height: usize)
           -> Self {
        DmabufParams {
            proxy: proxy,
            compositor_oid: compositor_oid,
            shell_oid: shell_oid,
            width: width,
            height: height,
        }
    }

    pub fn new_object(proxy: ProxyRef,
                      compositor_oid: ObjectId,
                      shell_oid: ObjectId,
                      width: usize,
                      height: usize)
                      -> Box<Object> {
        let object = Self::new(proxy, compositor_oid, shell_oid, width, height);
        Box::new(Handler::<_, zwp_linux_buffer_params_v1::Dispatcher>::new(object))
    }
}

// -------------------------------------------------------------------------------------------------

impl zwp_linux_buffer_params_v1::Interface for DmabufParams {
    fn created(&mut self,
               _this_object_id: ObjectId,
               bundle: &mut Bundle,
               buffer_oid: ObjectId)
               -> Task {
        let buffer_object = DmabufBuffer::new_object(self.proxy.clone());
        bundle.add_object(buffer_oid, buffer_object);
        common::create_shell_surface2(bundle,
                                      self.proxy.clone(),
                                      self.compositor_oid,
                                      self.shell_oid,
                                      buffer_oid,
                                      self.width,
                                      self.height);
        Task::None
    }

    fn failed(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_buffer` object.
pub struct DmabufBuffer {}

// -------------------------------------------------------------------------------------------------

impl DmabufBuffer {
    fn new(_proxy: ProxyRef) -> Self {
        DmabufBuffer {}
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_buffer::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_buffer::Interface for DmabufBuffer {
    fn release(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
