// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code responsible for gluing all other parts of crate together.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use dharma;
use skylane as wl;

use qualia::{Axis, Button, Key, Milliseconds, OutputInfo, Position, Size, SurfaceId, KeyMods};
use qualia::{Coordinator, KeyboardState, XkbKeymap, Perceptron, Settings, surface_state};

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
    output_infos: Vec<OutputInfo>,
    coordinator: Coordinator,
    settings: Settings,
    dispatcher: dharma::Dispatcher,
    keyboard_state: KeyboardState,
}

// -------------------------------------------------------------------------------------------------

impl Engine {
    /// Creates new `Engine`. Sets display socket up.
    pub fn new(coordinator: Coordinator, settings: Settings) -> Self {
        let xkb_keymap = XkbKeymap::default().expect("Creating XKB map");

        Engine {
            display: wl::server::DisplaySocket::new_default().expect("Creating display socket"),
            mediator: MediatorRef::new(Mediator::new()),
            clients: HashMap::new(),
            output_infos: Vec::new(),
            coordinator: coordinator,
            settings: settings,
            dispatcher: dharma::Dispatcher::new(),
            keyboard_state: KeyboardState::new(&xkb_keymap.keymap),
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
                                   self.settings.clone(),
                                   self.mediator.clone(),
                                   client_socket.clone());
        proxy.register_global(protocol::shm::get_global());
        proxy.register_global(protocol::compositor::get_global());
        proxy.register_global(protocol::shell::get_global());
        proxy.register_global(protocol::xdg_shell_v6::get_global());
        proxy.register_global(protocol::data_device_manager::get_global());
        proxy.register_global(protocol::seat::get_global());
        proxy.register_global(protocol::subcompositor::get_global());
        for info in self.output_infos.iter() {
            proxy.register_global(protocol::output::get_global(info.clone()));
        }
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
        log_wayl4!("Skylane: {}", s);
    }
}

// -------------------------------------------------------------------------------------------------

impl Gateway for Engine {
    fn on_display_created(&mut self, output_info: OutputInfo) {
        self.output_infos.push(output_info.clone());
        for (_, package) in self.clients.iter() {
            package.proxy.borrow_mut().on_display_created(output_info.clone());
        }
    }

    fn on_keyboard_input(&mut self, key: Key, _mods: Option<KeyMods>) {
        let mods = if self.keyboard_state.update(key.code, key.value) {
            Some(self.keyboard_state.get_mods())
        } else {
            None
        };

        let sid = self.coordinator.get_keyboard_focused_sid();
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_keyboard_input(key, mods);
            }
        }
    }

    fn on_surface_frame(&mut self, sid: SurfaceId, milliseconds: Milliseconds) {
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_surface_frame(sid, milliseconds);
            }
        }
    }

    fn on_pointer_focus_changed(&self, old_sid: SurfaceId, new_sid: SurfaceId, position: Position) {
        let mediator = self.mediator.borrow();
        let old_client_id = mediator.get_client_for_sid(old_sid);
        let new_client_id = mediator.get_client_for_sid(new_sid);

        if new_client_id != old_client_id {
            if let Some(client_id) = old_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_pointer_focus_changed(old_sid,
                                                           SurfaceId::invalid(),
                                                           Position::default());
                }
            }
            if let Some(client_id) = new_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_pointer_focus_changed(SurfaceId::invalid(), new_sid, position);
                }
            }
        } else {
            if let Some(client_id) = old_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_pointer_focus_changed(old_sid, new_sid, position);
                }
            }
        }
    }

    fn on_pointer_relative_motion(&self,
                                  sid: SurfaceId,
                                  position: Position,
                                  milliseconds: Milliseconds) {
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_pointer_relative_motion(sid, position, milliseconds);
            }
        }
    }

    fn on_pointer_button(&self, btn: Button) {
        let sid = self.coordinator.get_pointer_focused_sid();
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_pointer_button(btn);
            }
        }
    }

    fn on_pointer_axis(&self, axis: Axis) {
        let sid = self.coordinator.get_pointer_focused_sid();
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow_mut().on_pointer_axis(axis);
            }
        }
    }

    fn on_keyboard_focus_changed(&mut self, old_sid: SurfaceId, new_sid: SurfaceId) {
        let mediator = self.mediator.borrow();
        let old_client_id = mediator.get_client_for_sid(old_sid);
        let new_client_id = mediator.get_client_for_sid(new_sid);

        if new_client_id != old_client_id {
            if let Some(client_id) = old_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_keyboard_focus_changed(old_sid, SurfaceId::invalid());
                }
            }
            if let Some(client_id) = new_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_keyboard_focus_changed(SurfaceId::invalid(), new_sid);
                }
            }
        } else {
            if let Some(client_id) = old_client_id {
                if let Some(package) = self.clients.get(&client_id) {
                    package.proxy.borrow_mut()
                                 .on_keyboard_focus_changed(old_sid, new_sid);
                }
            }
        }
    }

    fn on_surface_reconfigured(&self,
                               sid: SurfaceId,
                               size: Size,
                               state_flags: surface_state::SurfaceState) {
        if let Some(id) = self.mediator.borrow().get_client_for_sid(sid) {
            if let Some(package) = self.clients.get(&id) {
                package.proxy.borrow().on_surface_reconfigured(sid, size, state_flags);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
