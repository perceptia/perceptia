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

//! Implementation of Wayland `wl_compositor` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::wayland::{wl_compositor, wl_surface};

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_compositor` object.
pub struct Compositor {}

// -------------------------------------------------------------------------------------------------

impl Compositor {
    fn new() -> Self {
        Compositor {}
    }

    pub fn new_object() -> Box<Object> {
        Box::new(Handler::<_, wl_compositor::Dispatcher>::new(Self::new()))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_compositor::Interface for Compositor {}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_surface` object.
pub struct Surface {
    _proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    fn new(proxy: ProxyRef) -> Self {
        Surface {
            _proxy: proxy,
        }
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_surface::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_surface::Interface for Surface {
    fn enter(&mut self,
             _this_object_id: ObjectId,
             _bundle: &mut Bundle,
             _output: ObjectId)
             -> Task {
        // Nothing to do so far
        Task::None
    }

    fn leave(&mut self,
             _this_object_id: ObjectId,
             _bundle: &mut Bundle,
             _output: ObjectId)
             -> Task {
        // Nothing to do so far
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
