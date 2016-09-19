// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Exhibitor` manages tasks related to drawing and compositing surfaces.

// -------------------------------------------------------------------------------------------------

#![feature(deque_extras)]

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate frames;
extern crate output;

mod surface_history;
mod compositor;
mod pointer;
mod display;

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;
use std::collections::HashMap;

use qualia::{Coordinator, SurfaceId};
use output::Output;

use compositor::Compositor;
use pointer::Pointer;
use display::Display;

// -------------------------------------------------------------------------------------------------

/// `Exhibitor` manages tasks related to drawing and compositing surfaces.
pub struct Exhibitor {
    last_output_id: i32,
    compositor: Compositor,
    pointer: Rc<Pointer>,
    displays: HashMap<i32, Display>,
    coordinator: Coordinator,
}

// -------------------------------------------------------------------------------------------------

/// General methods.
impl Exhibitor {
    /// `Exhibitor` constructor.
    pub fn new(mut coordinator: Coordinator) -> Self {
        Exhibitor {
            last_output_id: 0,
            compositor: Compositor::new(coordinator.clone()),
            pointer: Rc::new(Pointer::new(&mut coordinator)),
            displays: HashMap::new(),
            coordinator: coordinator,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Notification handlers.
impl Exhibitor {
    /// This method is called when new output was found.
    pub fn on_output_found(&mut self, bundle: qualia::DrmBundle) {
        log_info1!("Exhibitor: found output");
        let id = self.generate_next_output_id();
        let mut output = match Output::new(bundle, id) {
            Ok(output) => {
                log_info2!("Created output: {}", output.get_name());
                output
            }
            Err(err) => {
                log_error!("Could not create output: {}", err);
                return;
            }
        };
        log_info1!("Exhibitor: creating display");
        let display_frame = self.compositor.create_display(output.get_size(), output.get_name());
        let display = Display::new(self.coordinator.clone(),
                                   self.pointer.clone(),
                                   output,
                                   display_frame);
        self.displays.insert(id, display);
    }

    /// This method is called when pageflip occurred.
    /// `id` is ID of output that scheduled the pageflip.
    pub fn on_pageflip(&mut self, id: i32) {
        // Pass notification to associated display
        if let Some(ref mut display) = self.displays.get_mut(&id) {
            display.on_pageflip();
        }
    }

    /// This method is called when new surface is ready to be managed.
    pub fn on_surface_ready(&mut self, sid: SurfaceId) {
        self.compositor.manage_surface(sid);
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods.
impl Exhibitor {
    /// Generate next output ID.
    fn generate_next_output_id(&mut self) -> i32 {
        self.last_output_id += 1;
        self.last_output_id
    }
}

// -------------------------------------------------------------------------------------------------
