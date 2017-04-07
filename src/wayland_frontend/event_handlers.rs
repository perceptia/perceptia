// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definition of `dharma::EventHandler`s for global (display) socket and client sockets.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::RawFd;

use skylane::server as wl;

use dharma;
use qualia::Perceptron;

use constants;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::EventHandler` for global (display) socket.
pub struct DisplayEventHandler {
    socket: wl::DisplaySocket,
    sender: dharma::Sender<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl DisplayEventHandler {
    pub fn new(socket: wl::DisplaySocket, sender: dharma::Sender<Perceptron>) -> Self {
        DisplayEventHandler {
            socket: socket,
            sender: sender,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper methods.
impl DisplayEventHandler {
    #[inline]
    fn terminate(&mut self) {
        log_error!("Lost connection to display socket!");
    }

    #[inline]
    fn process_events(&mut self) {
        self.sender.send_custom(constants::HANDLE_NEW_CLIENT, Perceptron::CustomEmpty {});
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::EventHandler for DisplayEventHandler {
    fn get_fd(&self) -> RawFd {
        self.socket.get_fd()
    }

    fn process_event(&mut self, event_kind: dharma::EventKind) {
        if event_kind.intersects(dharma::event_kind::HANGUP) {
            self.terminate();
        } else if event_kind.intersects(dharma::event_kind::READ) {
            self.process_events();
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::EventHandler` for client socket.
pub struct ClientEventHandler {
    id: dharma::EventHandlerId,
    socket: wl::Socket,
    sender: dharma::DirectSender<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl ClientEventHandler {
    pub fn new(socket: wl::Socket, sender: dharma::DirectSender<Perceptron>) -> Self {
        ClientEventHandler {
            // No `Option` used here as `Dispatcher` will for sure set this data and we would
            // `expect` it anyway.
            id: 0,
            socket: socket,
            sender: sender,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl ClientEventHandler {
    #[inline]
    fn terminate(&mut self) {
        log_wayl1!("Broken pipe!");
        self.sender.send_custom(constants::TERMINATE_CLIENT, Perceptron::CustomId(self.id));
    }

    #[inline]
    fn process_events(&mut self) {
        self.sender.send_custom(constants::PROCESS_EVENTS, Perceptron::CustomId(self.id));
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::EventHandler for ClientEventHandler {
    fn get_fd(&self) -> RawFd {
        self.socket.get_fd()
    }

    fn process_event(&mut self, event_kind: dharma::EventKind) {
        if event_kind.intersects(dharma::event_kind::HANGUP) {
            self.terminate();
        } else if event_kind.intersects(dharma::event_kind::READ) {
            self.process_events();
        }
    }

    fn set_id(&mut self, id: dharma::EventHandlerId) {
        self.id = id;
    }
}

// -------------------------------------------------------------------------------------------------
