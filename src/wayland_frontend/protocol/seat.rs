// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_seat` and `wl_keyboard` objects.

use skylane as wl;
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_seat;
use skylane_protocols::server::wayland::wl_pointer;
use skylane_protocols::server::wayland::wl_keyboard;
use skylane_protocols::server::wayland::wl_touch;

use global::Global;
use facade::Facade;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_seat` object.
struct Seat {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_seat::NAME,
                wl_seat::VERSION,
                Box::new(Seat::new_object))
}

// -------------------------------------------------------------------------------------------------

impl Seat {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            // TODO: Add more capabilities
            let proxy = proxy_ref.borrow();
            let socket = proxy.get_socket();
            let caps = wl_seat::capability::POINTER | wl_seat::capability::KEYBOARD;
            send!(wl_seat::capabilities(&socket, oid, caps));
            send!(wl_seat::name(&socket, oid, "seat0"));
        }
        Seat { proxy: proxy_ref }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_seat::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_seat::Interface for Seat {
    fn get_pointer(&mut self,
                   _this_object_id: wl::common::ObjectId,
                   _socket: &mut wl::server::ClientSocket,
                   new_pointer_id: wl::common::ObjectId)
                   -> wl::server::Task {
        wl::server::Task::Create {
            id: new_pointer_id,
            object: Pointer::new_object(new_pointer_id, self.proxy.clone()),
        }
    }

    fn get_keyboard(&mut self,
                    _this_object_id: wl::common::ObjectId,
                    _socket: &mut wl::server::ClientSocket,
                    new_keyboard_id: wl::common::ObjectId)
                    -> wl::server::Task {
        wl::server::Task::Create {
            id: new_keyboard_id,
            object: Keyboard::new_object(new_keyboard_id, self.proxy.clone()),
        }
    }

    fn get_touch(&mut self,
                 _this_object_id: wl::common::ObjectId,
                 _socket: &mut wl::server::ClientSocket,
                 new_touch_id: wl::common::ObjectId)
                 -> wl::server::Task {
        wl::server::Task::Create {
            id: new_touch_id,
            object: Touch::new_object(new_touch_id, self.proxy.clone()),
        }
    }

    fn release(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_pointer` object.
struct Pointer {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Pointer {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        proxy_ref.borrow_mut().add_pointer_oid(oid);
        Pointer { proxy: proxy_ref }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_pointer::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_pointer::Interface for Pointer {
    fn set_cursor(&mut self,
                  _this_object_id: wl::common::ObjectId,
                  _socket: &mut wl::server::ClientSocket,
                  _serial: u32,
                  surface_oid: wl::common::ObjectId,
                  hotspot_x: i32,
                  hotspot_y: i32)
                  -> wl::server::Task {
        self.proxy.borrow_mut().set_as_cursor(surface_oid, hotspot_x as isize, hotspot_y as isize);
        wl::server::Task::None
    }

    fn release(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        self.proxy.borrow_mut().remove_pointer_oid(this_object_id);
        wl::server::Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_keyboard` object.
struct Keyboard {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Keyboard {
    fn new(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            let socket = proxy.get_socket();
            let keymap = proxy.get_settings().get_keymap();
            proxy.add_keyboard_oid(oid);
            send!(wl_keyboard::keymap(&socket, oid, keymap.format, keymap.fd, keymap.size as u32));
        }

        Keyboard { proxy: proxy_ref }
    }

    fn new_object(oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_keyboard::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_keyboard::Interface for Keyboard {
    fn release(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        self.proxy.borrow_mut().remove_keyboard_oid(this_object_id);
        wl::server::Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_touch` object.
#[allow(dead_code)]
struct Touch {
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Touch {
    fn new(proxy_ref: ProxyRef) -> Self {
        Touch { proxy: proxy_ref }
    }

    fn new_object(_oid: wl::common::ObjectId, proxy_ref: ProxyRef) -> Box<wl::server::Object> {
        Box::new(Handler::<_, wl_touch::Dispatcher>::new(Self::new(proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_touch::Interface for Touch {
    fn release(&mut self,
               this_object_id: wl::common::ObjectId,
               _socket: &mut wl::server::ClientSocket)
               -> wl::server::Task {
        wl::server::Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------
