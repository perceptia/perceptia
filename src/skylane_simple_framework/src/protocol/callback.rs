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

//! Implementation of Wayland `wl_callback` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::wayland::wl_callback;

use proxy::{Action, ProxyRef};
use protocol::display::Display;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_callback` object.
pub struct Callback {
    action: Action,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Callback {
    fn new(proxy: ProxyRef, action: Action) -> Self {
        Callback {
            action: action,
            proxy: proxy,
        }
    }

    pub fn new_object(proxy: ProxyRef, action: Action) -> Box<Object> {
        Box::new(Handler::<_, wl_callback::Dispatcher>::new(Self::new(proxy, action)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_callback::Interface for Callback {
    fn done(&mut self,
            _this_object_id: ObjectId,
            bundle: &mut Bundle,
            _callback_data: u32)
            -> Task {
        match self.action {
            Action::GlobalsDone => {
                self.proxy.borrow_mut().globals_done();
                let id = bundle.get_next_available_client_object_id();
                let object = Display::synchronize(self.proxy.clone(), id, Action::InitDone);
                Task::Create { id, object }
            }
            Action::InitDone => {
                self.proxy.borrow_mut().init_done();
                Task::None
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
