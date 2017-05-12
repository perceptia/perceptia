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

//! `dharma` event handlers.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::RawFd;

use skylane::client as wl;

use dharma;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::EventHandler` for server socket.
pub struct DisplayEventHandler {
    id: dharma::EventHandlerId,
    connection: wl::Connection,
    dispatcher: dharma::LocalDispatcherController,
}

// -------------------------------------------------------------------------------------------------

impl DisplayEventHandler {
    /// Constructs new `DisplayEventHandler`.
    pub fn new(connection: wl::Connection, dispatcher: dharma::LocalDispatcherController) -> Self {
        DisplayEventHandler {
            // No `Option` used here as `Dispatcher` will for sure set this data and we would
            // `expect` it anyway.
            id: 0,
            connection: connection,
            dispatcher: dispatcher,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::EventHandler for DisplayEventHandler {
    fn get_fd(&self) -> RawFd {
        self.connection.get_socket().get_fd()
    }

    fn process_event(&mut self, event_kind: dharma::EventKind) {
        if event_kind.intersects(dharma::event_kind::HANGUP) {
            println!("Server hung the connection up");
            self.dispatcher.stop();
        } else if event_kind.intersects(dharma::event_kind::READ) {
            self.connection.process_events().expect("Processing Wayland events");
        }
    }

    fn set_id(&mut self, id: dharma::EventHandlerId) {
        self.id = id;
    }
}

// -------------------------------------------------------------------------------------------------
