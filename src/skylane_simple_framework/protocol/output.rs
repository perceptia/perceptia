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

//! Implementation of Wayland `wl_output` object.

use skylane::client::{Bundle, Object, ObjectId, Task};
use skylane_protocols::client::Handler;
use skylane_protocols::client::wayland::wl_output;

use defs::OutputInfo;
use proxy::ProxyRef;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_output` object.
pub struct Output {
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    refresh_rate: usize,
    make: String,
    model: String,
    proxy: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl Output {
    fn new(proxy: ProxyRef) -> Self {
        Output {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            refresh_rate: 0,
            make: String::new(),
            model: String::new(),
            proxy: proxy,
        }
    }

    pub fn new_object(proxy: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_output::Dispatcher>::new(Self::new(proxy)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_output::Interface for Output {
    fn geometry(&mut self,
                _this_object_id: ObjectId,
                _bundle: &mut Bundle,
                x: i32,
                y: i32,
                _physical_width: i32,
                _physical_height: i32,
                _subpixel: i32,
                _make: String,
                _model: String,
                _transform: i32)
                -> Task {
        self.x = x as isize;
        self.y = y as isize;
        Task::None
    }

    fn mode(&mut self,
            _this_object_id: ObjectId,
            _bundle: &mut Bundle,
            _flags: u32,
            width: i32,
            height: i32,
            refresh: i32)
            -> Task {
        self.width = width as usize;
        self.height = height as usize;
        self.refresh_rate = refresh as usize;
        Task::None
    }

    fn done(&mut self,
            this_object_id: ObjectId,
            _bundle: &mut Bundle)
            -> Task {
        if self.width != 0 && self.height != 0 {
            let output = OutputInfo::new(self.x,
                                         self.y,
                                         self.width,
                                         self.height,
                                         self.make.clone(),
                                         self.model.clone(),
                                         this_object_id.get_value());
            self.proxy.borrow_mut().add_output(output);
        }
        Task::None
    }

    fn scale(&mut self,
             _this_object_id: ObjectId,
             _bundle: &mut Bundle,
             _factor: i32)
             -> Task {
        // Ignore scale.
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
