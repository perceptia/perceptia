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

//! Interface for implementing listening to events in applications.

use std::collections::HashSet;

use defs::OutputInfo;
use controller::Controller;

// -------------------------------------------------------------------------------------------------

pub trait Listener {
    /// Called when initialization is done.
    ///
    /// List of globals advertised by server is passed. If needed interface is not supported
    /// application can now warn and exit.
    fn init_done(&mut self, _globals: HashSet<String>) {}

    /// Called when list of available outputs was updated.
    fn outputs_done(&mut self, _outputs: Vec<OutputInfo>) {}

    /// Called when screenshot request ended successfully.
    fn screenshot_done(&mut self, _buffer: Vec<u8>) {}

    /// Called when screenshot request failed.
    fn screenshot_failed(&mut self) {}
}

// -------------------------------------------------------------------------------------------------

pub trait ListenerConstructor {
    type Listener: Listener + 'static;

    fn construct(&self, controller: Controller) -> Box<Self::Listener>;
}

// -------------------------------------------------------------------------------------------------

pub struct Dummy {}
impl Listener for Dummy {}

// -------------------------------------------------------------------------------------------------
