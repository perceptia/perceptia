// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate implements Wayland functionality.
//!
//! # Sketch of architecture
//!
//! The piece of code running all the business is `Engine`. It sets up everything and handles
//! requests from clients as well as the rests of application. Upon client connection `Engine`
//! creates new
//!
//! - `skylane::Client`, structure registering handlers (representing Wayland objects) and
//! dispatching client requests to them and
//! - `wayland_frontend::Proxy`, structure used for sharing information between handlers, a state
//! of client.
//!
//! `Proxy` is shared between handlers as `RefCell`.
//!
//! Two interfaces were introduced:
//!
//! - `Facade`, for requests from clients (downward?) which is implemented by `Proxy` and
//! - `Gateway`, for requests from application (upward?) which is implemented by `Engine`
//! (dispatching request to correct `Proxy`s) and `Proxy` (making actual request to client).
//!
//! So one `Engine` holds many `Client`s holding many (cell) references to `Proxy`, but at some
//! point `Engine` must be informed that client created surface. For this purpose `Engine` and all
//! `Proxy`s hold (cell) reference to single `Mediator`.
//!
//! Details of threading are left for application. `wayland_frontend` may be configured to receive
//! in one thread and send in other or do everything in one thread. What is however sure accepting
//! new client can not be done in `DisplayEventHandler` and handling requests can not be done in
//! `ClientEventHandler` as it may require mutating `dharma::Dispatcher`, so handling is decoupled
//! from processing using `dharma::DirectSender`.

// TODO: Move common DRM functionality to module.
extern crate drm as libdrm;
extern crate nix;

extern crate skylane;
extern crate skylane_protocols;

extern crate dharma;
#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;

// TODO: Get rid of dependency from `coordination` and `dharma` in `wayland_frontend`. See
// description of `coordination` crate. Provide unit tests.
extern crate coordination;

pub mod constants;

#[macro_use]
mod macros;
mod mediator;
mod global;
mod facade;
mod gateway;
mod proxy;
mod event_handlers;

mod protocol;

pub mod engine;

pub use engine::Engine;
pub use gateway::Gateway;
