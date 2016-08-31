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

#![feature(fnbox)]

/// Communication between two endpoints in different threads.
///
pub mod bridge;
pub use bridge::{connect, Sender, Receiver, ReceiveResult, Transportable};

/// Notification sender.
///
pub mod signaler;
pub use signaler::{Event, Signaler, SignalId};

/// This module contains implementation of main thread loop with notification listening.
///
pub mod event_loop;
pub use event_loop::{Context, EventLoop, EventLoopInfo, InitResult, Module};
