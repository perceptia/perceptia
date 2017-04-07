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

//! Bringing all parts of program together.

use std;

use skylane::client as wl;
use skylane_protocols::client::wayland::wl_display;

use dharma;

use store::{Store, StoreRef};
use event_handler::DisplayEventHandler;
use controller::Controller;
use proxy::{Action, Proxy, ProxyRef};
use listener::ListenerConstructor;
use protocol::{display, callback, registry};

// -------------------------------------------------------------------------------------------------

/// `Application` brings all parts together, initialises them and starts running the application.
pub struct Application {}

// -------------------------------------------------------------------------------------------------

impl Application {
    /// Constructs new `Application`.
    pub fn new() -> Self {
        Application {}
    }

    /// Logger method passed to Skylane.
    fn logger(s: String) {
        println!("Skylane: {}", s);
    }

    /// Runs the application.
    pub fn run<C>(&self, listener_contructor: C) where C: ListenerConstructor {
        // Initialize Wayland
        let mut socket = wl::Socket::connect_default().expect("Create default socket");

        // Set logging on if `WAYLAND_DEBUG` variable is set.
        if let Ok(_) = std::env::var("WAYLAND_DEBUG") {
            socket.set_logger(Some(Self::logger));
        }

        let connection = wl::Connection::new(socket.clone());
        let mut connection_controller = connection.get_controller();
        let mut dispatcher = dharma::LocalDispatcher::new();

        // Create application
        let store = StoreRef::new(Store::new());
        let proxy = Proxy::new(store.clone(), socket.clone());
        let proxy_ref = ProxyRef::new(proxy);
        let controller = Controller::new(connection_controller.clone(),
                                         store.clone(),
                                         dispatcher.get_controller(),
                                         proxy_ref.downgrade());
        let listener = listener_contructor.construct(controller);
        proxy_ref.borrow_mut().set_listener(listener);

        // Setup dispatcher
        let event_handler = DisplayEventHandler::new(connection,
                                                     dispatcher.get_controller());
        dispatcher.add_source(Box::new(event_handler), dharma::event_kind::READ);

        // Initiate communication
        let display_object = display::Display::new_object(proxy_ref.clone());
        let registry_object = registry::Registry::new_object(proxy_ref.clone());
        let callback_object = callback::Callback::new_object(proxy_ref.clone(), Action::InitDone);

        connection_controller.add_next_client_object(display_object);
        let registry_lid = connection_controller.add_next_client_object(registry_object);
        let callback_lid = connection_controller.add_next_client_object(callback_object);
        send!(wl_display::get_registry(&socket, wl::DISPLAY_ID, registry_lid));
        send!(wl_display::sync(&socket, wl::DISPLAY_ID, callback_lid));

        // Run application
        dispatcher.run();
    }
}

// -------------------------------------------------------------------------------------------------
