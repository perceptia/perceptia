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

//! This is simple frame framework demonstrating how `skylane` crate could be used to implement
//! Wayland framework.
//!
//! This crate is used by `perceptiactl` for taking screenshots.

extern crate nix;

extern crate dharma;
#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;

extern crate skylane;
extern crate skylane_protocols;

#[macro_use]
mod defs;
pub use defs::OutputInfo;

mod store;
mod event_handler;

mod listener;
pub use listener::{Listener, ListenerConstructor};

mod protocol;

mod controller;
pub use controller::Controller;

mod proxy;

mod application;
pub use application::Application;
