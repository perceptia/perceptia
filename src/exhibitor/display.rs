// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code ruling redrawing display.

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;
use std::cell::RefCell;

use qualia::{Buffer, Illusion, Milliseconds, OutputInfo, perceptron, Perceptron, Position};
use qualia::{ExhibitorCoordinationTrait, SurfaceContext, SurfaceId};

use frames::{Frame, Displaying};
use output::Output;

use pointer::Pointer;

// -------------------------------------------------------------------------------------------------

/// `Display`
pub struct Display<C>
    where C: ExhibitorCoordinationTrait
{
    coordinator: C,
    pointer: Rc<RefCell<Pointer<C>>>,
    output: Box<Output>,
    frame: Frame,
    redraw_needed: bool,
    page_flip_scheduled: bool,
    background_sid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl<C> Display<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Display` constructor.
    pub fn new(coordinator: C,
               pointer: Rc<RefCell<Pointer<C>>>,
               output: Box<Output>,
               frame: Frame)
               -> Self {
        let mut d = Display {
            coordinator: coordinator,
            pointer: pointer,
            output: output,
            frame: frame,
            redraw_needed: true,
            page_flip_scheduled: false,
            background_sid: SurfaceId::invalid(),
        };
        d.redraw_all(); // TODO: Remove when notifications are supported in Wayland module.
        d
    }

    /// Get information about output (size, position, model name, etc.).
    pub fn get_info(&self) -> OutputInfo {
        self.output.get_info()
    }

    /// Schedule page flip on assigned output.
    pub fn schedule_pageflip(&mut self) -> Result<(), Illusion> {
        if !self.page_flip_scheduled {
            self.output.schedule_pageflip()?;
            self.page_flip_scheduled = true;
        }
        Ok(())
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

    /// Handle notification about wakeup.
    pub fn on_wakeup(&mut self) {
        let output = self.output.recreate();
        match output {
            Ok(output) => {
                self.output = output;
                self.redraw_all();
            }
            Err(err) => {
                log_error!("Failed to reset output after wakeup: {:?}", err);
            }
        }
    }

    /// Prepare rendering context for layover.
    pub fn prepare_layover_context(&self, display_position: Position) -> Vec<SurfaceContext> {
        vec![SurfaceContext::new(self.pointer.borrow().get_cursor_sid(),
                                 self.pointer.borrow().get_global_position() - display_position)]
    }

    /// Prepare rendering context for layunder.
    pub fn prepare_layunder_context(&self) -> Vec<SurfaceContext> {
        if self.background_sid.is_valid() {
            vec![SurfaceContext::new(self.background_sid, Position::default())]
        } else {
            Vec::new()
        }
    }

    /// Draw the scene and then schedule page flip.
    ///
    /// TODO: Benchmark drawing.
    fn redraw_all(&mut self) {
        let info = self.output.get_info();

        let surfaces = self.frame
            .get_first_time()
            .expect("display must have at least one workspace")
            .to_array(Position::default(), &self.coordinator);

        let layover = self.prepare_layover_context(info.area.pos);
        let layunder = self.prepare_layunder_context();
        self.pointer.borrow_mut().update_hover_state(info.area, &surfaces);

        if let Err(err) = self.output.draw(&layunder, &surfaces, &layover, &self.coordinator) {
            log_error!("Display: {}", err);
        }

        if let Err(err) = self.output.swap_buffers() {
            log_error!("Display: {}", err);
        }

        // Send frame notifications
        for context in surfaces {
            let frame = Perceptron::SurfaceFrame(context.id, Milliseconds::now());
            self.coordinator.emit(perceptron::SURFACE_FRAME, frame);
        }

        self.redraw_needed = false;
        if let Err(err) = self.schedule_pageflip() {
            log_error!("Display: {}", err);
        }
    }

    /// Requests output to take screenshot. Return `Buffer` containing image data.
    pub fn take_screenshot(&self) -> Option<Buffer> {
        match self.output.take_screenshot() {
            Ok(buffer) => Some(buffer),
            Err(err) => {
                log_error!("Display: {}", err);
                None
            }
        }
    }

    /// Handles request to change background surface ID.
    pub fn on_background_change(&mut self, sid: SurfaceId) {
        self.background_sid = sid;
    }
}

// -------------------------------------------------------------------------------------------------
