// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `dharma` crate provides high-level multi-threading support.
//!
//! ### `bridge` module
//!
//! `bridge` module provides similar functionality as standard `spmc` but instead of producing
//! fixed pairs `Sender`-`Receiver` it allows to connect them freely, so we can have many one
//! `Sender` sending to many `Receivers` and one `Receiver` listening to many senders in flexible,
//! configurable way.
//!
//! ### `signaler` module
//!
//! On to of that we add `Signaler` which can subscribe receivers for signals (application defined
//! events) creating notification mechanism.
//!
//! ### `event_loop` module
//!
//! On top of `Signaler` we add `EventLoop`, which is event queue assigned to thread. `EventLoop`
//! has assigned `Module`s constituting separate application components. `Module`s can be assigned
//! to `EventLoop`s in flexible way making it easy to control tasks processed in threads. `Module`s
//! do not share memory and communicate with signals.
//!
//! ### `dispatcher` module
//!
//! Every threading framework should provide mechanism for listening on system events. `Dispatcher`
//! allows to register `EventHandler`s (wrapping file descriptors) and invokes them when system
//! events they are assigned are ready to be processed.
//!
//! ### `system` module
//!
//! Last module contains helper code for and handling system signals.

#![feature(fnbox)]

extern crate nix;

/// Communication between two endpoints in different threads.
///
pub mod bridge;
pub use bridge::{connect, Sender, Receiver, ReceiveResult, Transportable};

/// Notification sender.
///
pub mod signaler;
pub use signaler::{Event, Signaler, SignalId};

/// Implementation of main thread loop with notification listening.
///
pub mod event_loop;
pub use event_loop::{EventLoop, EventLoopInfo, InitResult, Module};

/// Handling system events (`epoll` wrapper).
///
pub mod dispatcher;
pub use dispatcher::{EventHandler, Dispatcher};

/// System signal handling.
///
pub mod system;
pub use system::{block_signals, unblock_signals, SignalEventHandler};
