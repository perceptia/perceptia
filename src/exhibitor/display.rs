// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code ruling redrawing display.

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;

use qualia::{Coordinator, Error, SurfaceContext};

use frames::Frame;
use output::Output;

use pointer::Pointer;

// -------------------------------------------------------------------------------------------------

/// `Display`
pub struct Display {
    coordinator: Coordinator,
    pointer: Rc<Pointer>,
    output: Output,
    frame: Frame,
    redraw_needed: bool,
    pageflip_scheduled: bool,
}

// -------------------------------------------------------------------------------------------------

impl Display {
    /// `Display` constructor.
    pub fn new(coordinator: Coordinator,
               pointer: Rc<Pointer>,
               output: Output,
               frame: Frame)
               -> Self {
        let mut d = Display {
            coordinator: coordinator,
            pointer: pointer,
            output: output,
            frame: frame,
            redraw_needed: true,
            pageflip_scheduled: false,
        };
        d.redraw_all(); // TODO: Remove when notifications are supported in Wayland module.
        d
    }

    /// Schedule pageflip on assigned output.
    fn schedule_pageflip(&mut self) -> Result<(), Error> {
        if !self.pageflip_scheduled {
            self.pageflip_scheduled = true;
            self.output.schedule_pageflip()
        } else {
            Ok(())
        }
    }

    /// Handle pageflip: redraw everything.
    pub fn on_pageflip(&mut self) {
        self.pageflip_scheduled = false;
        self.redraw_all();
    }

    /// Prepare rendering context for layover.
    pub fn prepare_layover_context(&self) -> SurfaceContext {
        SurfaceContext::new(self.pointer.get_sid(), self.pointer.get_global_position())
    }

    /// Draw the scene and then schedule pageflip.
    pub fn redraw_all(&mut self) {
        if self.redraw_needed {
            let surfaces = Vec::new();
            let pointer = self.prepare_layover_context();

            if let Err(err) = self.output.draw(&surfaces, pointer, &self.coordinator) {
                log_error!("Display: {}", err);
            }

            if let Err(err) = self.output.swap_buffers() {
                log_error!("Display: {}", err);
            }

            if let Err(err) = self.schedule_pageflip() {
                log_error!("Display: {}", err);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
