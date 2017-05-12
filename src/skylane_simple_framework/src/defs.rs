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

//! Various definitions used around `skylane_simple_framework`.

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct OutputInfo {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
    pub make: String,
    pub model: String,
    pub id: u32,
}

// -------------------------------------------------------------------------------------------------

impl OutputInfo {
    /// Constructs new `OutputInfo`.
    pub fn new(x: isize,
               y: isize,
               width: usize,
               height: usize,
               make: String,
               model: String,
               id: u32)
               -> Self {
        OutputInfo {
            x: x,
            y: y,
            width: width,
            height: height,
            make: make,
            model: model,
            id: id,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This macro executes passed expression and in case of error logs warning about failing to send
/// data. It is intended to use when posting `skylane` requests to server.
#[macro_export]
macro_rules! send {
    {$command:expr} => {
        ($command).expect("Sending message to display server");
    }
}

// -------------------------------------------------------------------------------------------------
