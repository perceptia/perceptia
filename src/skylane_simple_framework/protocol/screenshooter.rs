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

//! Implementation of Wayland `weston_screenshooter` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::weston_screenshooter::weston_screenshooter;

use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Weston `weston_screenshooter` object.
pub struct Screenshooter {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Screenshooter{
    fn new(proxy: ProxyRef) -> Self {
        Screenshooter {
            proxy: proxy,
        }
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, weston_screenshooter::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl weston_screenshooter::Interface for Screenshooter {
    fn done(&mut self,
            _this_object_id: ObjectId,
            _bundle: &mut Bundle)
            -> Task {
        self.proxy.borrow_mut().screenshot_done();
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
