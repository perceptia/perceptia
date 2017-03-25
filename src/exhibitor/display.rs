// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code ruling redrawing display.

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;
use std::cell::RefCell;

use dharma::Signaler;
use qualia::{Area, Coordinator, Illusion, Milliseconds, SurfaceContext, perceptron, Perceptron};

use frames::{Frame, Displaying};
use output::Output;

use pointer::Pointer;

// -------------------------------------------------------------------------------------------------

/// `Display`
pub struct Display {
    coordinator: Coordinator,
    signaler: Signaler<Perceptron>,
    pointer: Rc<RefCell<Pointer>>,
    output: Output,
    frame: Frame,
    redraw_needed: bool,
    page_flip_scheduled: bool,
}

// -------------------------------------------------------------------------------------------------

impl Display {
    /// `Display` constructor.
    pub fn new(coordinator: Coordinator,
               signaler: Signaler<Perceptron>,
               pointer: Rc<RefCell<Pointer>>,
               output: Output,
               frame: Frame)
               -> Self {
        let mut d = Display {
            coordinator: coordinator,
            signaler: signaler,
            pointer: pointer,
            output: output,
            frame: frame,
            redraw_needed: true,
            page_flip_scheduled: false,
        };
        d.redraw_all(); // TODO: Remove when notifications are supported in Wayland module.
        d
    }

    /// Schedule page flip on assigned output.
    fn schedule_pageflip(&mut self) -> Result<(), Illusion> {
        if !self.page_flip_scheduled {
            self.page_flip_scheduled = true;
            self.output.schedule_pageflip()
        } else {
            Ok(())
        }
    }

    /// Handle page flip: redraw everything.
    pub fn on_pageflip(&mut self) {
        self.page_flip_scheduled = false;
        if self.redraw_needed {
            self.redraw_all();
        }
    }

    /// Handle notification about needed redraw.
    ///
    /// This will cause display redraw. If page flip is already scheduled, display will be redraw
    /// again after page flip.
    pub fn on_notify(&mut self) {
        if !self.redraw_needed {
            if !self.page_flip_scheduled {
                self.redraw_all();
            } else {
                self.redraw_needed = true;
            }
        }
    }

    /// Prepare rendering context for layover.
    pub fn prepare_layover_context(&self) -> SurfaceContext {
        SurfaceContext::new(self.pointer.borrow().get_cursor_sid(),
                            self.pointer.borrow().get_global_position())
    }

    /// Draw the scene and then schedule page flip.
    pub fn redraw_all(&mut self) {
        let surfaces = self.frame
            .get_first_time()
            .expect("display must have at least one workspace")
            .to_array(&self.coordinator);

        let pointer = self.prepare_layover_context();
        self.pointer.borrow_mut().update_hover_state(self.output.get_area(), &surfaces);

        if let Err(err) = self.output.draw(&surfaces, pointer, &self.coordinator) {
            log_error!("Display: {}", err);
        }

        if let Err(err) = self.output.swap_buffers() {
            log_error!("Display: {}", err);
        }

        // Send frame notifications
        for context in surfaces {
            let frame = Perceptron::SurfaceFrame(context.id, Milliseconds::now());
            self.signaler.emit(perceptron::SURFACE_FRAME, frame);
        }

        self.redraw_needed = false;
        if let Err(err) = self.schedule_pageflip() {
            log_error!("Display: {}", err);
        }
    }

    /// Get area of the output in global coordinates.
    pub fn get_area(&self) -> Area {
        self.output.get_area()
    }
}

// -------------------------------------------------------------------------------------------------
