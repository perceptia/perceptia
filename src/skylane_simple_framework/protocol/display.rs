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

//! Implementation of Wayland `wl_display` object.

use skylane::client as wl;
use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::wayland::wl_display;

use proxy::{Action, ProxyRef};
use protocol::callback;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_display` object.
pub struct Display {}

// -------------------------------------------------------------------------------------------------

impl Display {
    fn new(_proxy: ProxyRef) -> Self {
        Display {}
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_display::Dispatcher>::new(Self::new(proxy)))
    }

    pub fn synchronize(proxy_ref: ProxyRef, callback_id: ObjectId, action: Action) -> Box<Object> {
        let proxy = proxy_ref.borrow_mut();
        let socket = proxy.get_socket();
        let callback_object = callback::Callback::new_object(proxy_ref.clone(), action);
        send!(wl_display::sync(&socket, wl::DISPLAY_ID, callback_id));
        callback_object
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_display::Interface for Display {
    fn error(&mut self,
             _this_object_id: ObjectId,
             _bundle: &mut Bundle,
             _object_id: ObjectId,
             code: u32,
             message: String)
             -> Task {
        println!("Server Error ({}): {}", code, message);
        Task::None
    }

    fn delete_id(&mut self, _this_object_id: ObjectId, _bundle: &mut Bundle, id: u32) -> Task {
        Task::Destroy { id: ObjectId::new(id) }
    }
}

// -------------------------------------------------------------------------------------------------
