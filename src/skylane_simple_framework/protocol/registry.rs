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

//! Implementation of Wayland `wl_registry` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::wayland::{wl_registry, wl_output, wl_shm};
use skylane_protocols::client::weston_screenshooter::weston_screenshooter;

use proxy::ProxyRef;

use protocol::{output, shm, screenshooter};

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_registry` object.
pub struct Registry {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Registry {
    fn new(proxy: ProxyRef) -> Self {
        Registry {
            proxy: proxy,
        }
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_registry::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_registry::Interface for Registry {
    fn global(&mut self,
              this_object_id: ObjectId,
              bundle: &mut Bundle,
              name: u32,
              interface: String,
              version: u32)
              -> Task {
        let s = bundle.get_socket();
        let id = bundle.get_next_available_client_object_id();
        let mut proxy = self.proxy.borrow_mut();
        proxy.add_global(interface.clone());

        if interface == wl_output::NAME {
            send!(wl_registry::bind(&s, this_object_id, name, &interface, version, id));
            let object = output::Output::new_object(self.proxy.clone());
            Task::Create{ id: id, object: object }
        } else if interface == wl_shm::NAME {
            proxy.set_shm_oid(id);
            send!(wl_registry::bind(&s, this_object_id, name, &interface, version, id));
            let object = shm::Shm::new_object(self.proxy.clone());
            Task::Create{ id: id, object: object }
        } else if interface == weston_screenshooter::NAME {
            send!(wl_registry::bind(&s, this_object_id, name, &interface, version, id));
            proxy.set_screenshooter_oid(id);
            let object = screenshooter::Screenshooter::new_object(self.proxy.clone());
            Task::Create{ id: id, object: object }
        } else {
            Task::None
        }
    }

    fn global_remove(&mut self,
                     _this_object_id: ObjectId,
                     _bundle: &mut Bundle,
                     _name: u32)
                     -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
