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

//! Implementation of Wayland `wl_drm` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::drm::wl_drm;
use skylane_protocols::client::wayland::wl_buffer;

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_drm` object.
pub struct Drm {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Drm {
    fn new(proxy: ProxyRef) -> Self {
        Drm { proxy: proxy }
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_drm::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_drm::Interface for Drm {
    fn device(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle, name: String) -> Task {
        self.proxy.borrow_mut().set_drm_device_name(name);
        Task::None
    }

    fn format(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle, _format: u32) -> Task {
        // Nothing to do so far
        Task::None
    }

    fn authenticated(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle) -> Task {
        self.proxy.borrow_mut().drm_authenticated();
        Task::None
    }

    fn capabilities(&mut self,
                    _this_object_id: ObjectId,
                    _bundle: &mut Bundle,
                    _value: u32)
                    -> Task {
        // Nothing to do so far
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_buffer` object.
pub struct DrmBuffer {}

// -------------------------------------------------------------------------------------------------

impl DrmBuffer {
    fn new(_proxy: ProxyRef) -> Self {
        DrmBuffer {}
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_buffer::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_buffer::Interface for DrmBuffer {
    fn release(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle) -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
