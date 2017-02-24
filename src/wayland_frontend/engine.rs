// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code responsible for gluing all other parts of crate together.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use dharma;
use skylane as wl;

use qualia::{Button, Key, Milliseconds, Size, SurfaceId, SurfacePosition, Vector};
use qualia::{Coordinator, Perceptron, surface_state};

use protocol;
use gateway::Gateway;
use proxy::{Proxy, ProxyRef};
use mediator::{Mediator, MediatorRef};
use event_handlers::{ClientEventHandler, DisplayEventHandler};

// -------------------------------------------------------------------------------------------------

/// Helper structure for aggregating `Client` with its `Proxy`.
struct ClientPackage {
    client: wl::server::Client,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

/// This is main structure of `wayland_frontend` crate.
///
/// For information about its role and place among other structures see crate-level documentation.
pub struct Engine {
    display: wl::server::DisplaySocket,
    mediator: MediatorRef,
    clients: HashMap<dharma::EventHandlerId, ClientPackage>,
    coordinator: Coordinator,
    dispatcher: dharma::Dispatcher,
}

// -------------------------------------------------------------------------------------------------

impl Engine {
    /// Creates new `Engine`. Sets display socket up.
    pub fn new(coordinator: Coordinator) -> Self {
        Engine {
            display: wl::server::DisplaySocket::new_default().expect("Creating display socket"),
            mediator: MediatorRef::new(Mediator::new()),
            clients: HashMap::new(),
            coordinator: coordinator,
            dispatcher: dharma::Dispatcher::new(),
        }
    }

    /// Starts `Engine`: adds display socket to `Dispatcher`.
    pub fn start(&mut self, sender: dharma::Sender<Perceptron>) {
        let handler = Box::new(DisplayEventHandler::new(self.display.clone(), sender));
        self.dispatcher.add_source(handler, dharma::event_kind::READ);
    }

    /// Reads client requests without blocking.
    pub fn receive(&mut self) {
        self.dispatcher.wait_and_process(Some(0));
    }
}

// -------------------------------------------------------------------------------------------------

/// Public handlers for client related events.
impl Engine {
    /// Handles new client:
    /// - accepts socket and adds it to `Dispatcher`
    /// - creates proxy for new client and registers global Wayland objects.
    /// - creates global display Wayland objects and bind it to client
    pub fn handle_new_client(&mut self, sender: dharma::DirectSender<Perceptron>) {
        // Accept the client.
        let mut client_socket = self.display.accept().expect("Accepting client");
        client_socket.set_logger(Some(Self::logger));

        // Prepare event handler.
        let id = self.dispatcher
            .add_source(Box::new(ClientEventHandler::new(client_socket.clone(), sender)),
                        dharma::event_kind::READ);

        // Prepare proxy.
        let mut proxy = Proxy::new(id,
                                   self.coordinator.clone(),
                                   self.mediator.clone(),
                                   client_socket.clone());
        proxy.register_global(protocol::shm::get_global());
        proxy.register_global(protocol::compositor::get_global());
        proxy.register_global(protocol::shell::get_global());
        proxy.register_global(protocol::xdg_shell_v6::get_global());
        let proxy_ref = ProxyRef::new(proxy);

        // Prepare client.
        let display = protocol::display::Display::new_object(proxy_ref.clone());
        let mut client = wl::server::Client::new(client_socket);
        client.add_object(wl::common::DISPLAY_ID, display);
        let pkg = ClientPackage {
            client: client,
            proxy: proxy_ref,
        };
        self.clients.insert(id, pkg);
    }

    /// Handles termination (socket hung up) of client.
    pub fn terminate_client(&mut self, id: dharma::EventHandlerId) {
        let result1 = if let Some(_handler) = self.dispatcher.delete_source(id) {
            true
        } else {
            log_warn2!("Dispatching handler not found for client {} on termination", id);
            false
        };

        let result2 = if let Some(_package) = self.clients.remove(&id) {
            true
        } else {
            log_warn2!("Proxy not found for client {} on termination", id);
            false
        };

        if result1 && result2 {
            log_wayl3!("Client {} terminated successfully", id);
        }
    }

    /// Handles request from client associated with given `id`.
    pub fn process_events(&mut self, id: dharma::EventHandlerId) {
        if let Some(ref mut package) = self.clients.get_mut(&id) {
            match package.client.process_events() {
                Ok(_) => {}
                Err(err) => {
                    log_warn3!("Wayland Engine: ERROR: {:?}", err);
                }
            }
        } else {
            log_warn1!("Wayland Engine: No client: {}", id);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Private helper methods.
impl Engine {
    fn logger(s: String) {
        log_wayl3!("Skylane: {}", s);
    }
}

// -------------------------------------------------------------------------------------------------

impl Gateway for Engine {
    fn on_output_found(&self) {}

    fn on_keyboard_input(&self, _key: Key) {}

    fn on_pointer_button(&self, _btn: Button) {}

    fn on_pointer_axis(&self, _axis: Vector) {}

    fn on_surface_frame(&mut self, sid: SurfaceId, milliseconds: Milliseconds) {
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_surface_frame(sid, milliseconds);
            }
        }
    }

    fn on_pointer_focus_changed(&self, _surface_position: SurfacePosition) {}

    fn on_pointer_relative_motion(&self, _surface_position: SurfacePosition) {}

    fn on_keyboard_focus_changed(&self, _old_sid: SurfaceId, _new_sid: SurfaceId) {
        // FIXME: Finish implementation of changing keyboard focus.
        // let (old_size, old_flags) =
        // if let Some(info) = self.coordinator.get_surface(old_sid) {
        // (info.desired_size, info.state_flags as u32)
        // } else {
        // (Size::default(), surface_state::REGULAR as u32)
        // };
        //
        // let (new_size, new_flags) =
        // if let Some(info) = self.coordinator.get_surface(new_sid) {
        // (info.desired_size, info.state_flags as u32)
        // } else {
        // (Size::default(), surface_state::REGULAR as u32)
        // };

    }

    fn on_surface_reconfigured(&mut self,
                               sid: SurfaceId,
                               size: Size,
                               state_flags: surface_state::SurfaceState) {
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_surface_reconfigured(sid, size, state_flags);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
