// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Exhibitor` manages tasks related to drawing and compositing surfaces.

// -------------------------------------------------------------------------------------------------

extern crate rand;

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

mod strategies;
mod strategist;

pub use strategist::Strategist;

// -------------------------------------------------------------------------------------------------

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use qualia::{SurfaceId, Button, Command, OptionalPosition, Position, Vector};
use qualia::{perceptron, Perceptron};
use qualia::{CompositorConfig, ExhibitorCoordinationTrait};
use output::Output;

use compositor::Compositor;
use pointer::Pointer;
use display::Display;

// -------------------------------------------------------------------------------------------------

/// `Exhibitor` manages tasks related to drawing and compositing surfaces.
pub struct Exhibitor<C> where C: ExhibitorCoordinationTrait {
    compositor: Compositor<C>,
    pointer: Rc<RefCell<Pointer<C>>>,
    displays: HashMap<i32, Display<C>>,
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

/// General methods.
impl<C> Exhibitor<C> where C: ExhibitorCoordinationTrait {
    /// `Exhibitor` constructor.
    pub fn new(coordinator: C,
               strategist: Strategist,
               compositor_config: CompositorConfig)
               -> Self {
        Exhibitor {
            compositor: Compositor::new(coordinator.clone(), strategist, compositor_config),
            pointer: Rc::new(RefCell::new(Pointer::new(coordinator.clone()))),
            displays: HashMap::new(),
            coordinator: coordinator,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Notification handlers.
impl<C> Exhibitor<C> where C: ExhibitorCoordinationTrait {
    /// Handles notification about needed redraw.
    pub fn on_notify(&mut self) {
        for ref mut display in self.displays.values_mut() {
            display.on_notify();
        }
    }

    /// Handles notification about deactivation of seat.
    pub fn on_suspend(&self) {
        // Nothing to do?...
    }

    /// Handles notification about activation of seat.
    ///
    /// Wakes up each display.
    pub fn on_wakeup(&mut self) {
        for ref mut display in self.displays.values_mut() {
            display.on_wakeup();
        }
    }

    /// This method is called when new output was found.
    pub fn on_output_found(&mut self, mut output: Box<Output>) {
        log_info1!("Exhibitor: found output");
        output.set_position(self.choose_new_display_position());
        let info = output.get_info();
        if self.displays.len() == 0 {
            self.pointer.borrow_mut().change_display(info.area);
        }

        log_info1!("Exhibitor: creating display");
        let display_frame = self.compositor.create_display(info.area, info.make.clone());
        let display = Display::new(self.coordinator.clone(),
                                   self.pointer.clone(),
                                   output,
                                   display_frame);
        self.displays.insert(info.id, display);

        self.coordinator.emit(perceptron::DISPLAY_CREATED, Perceptron::DisplayCreated(info));
    }

    /// This method is called when pageflip occurred.
    /// `id` is ID of output that scheduled the pageflip.
    pub fn on_pageflip(&mut self, id: i32) {
        // Pass notification to associated display
        if let Some(ref mut display) = self.displays.get_mut(&id) {
            display.on_pageflip();
        }
    }

    /// This method is called when a command was requested to be executed by compositor.
    pub fn on_command(&mut self, command: Command) {
        log_info2!("Received command: {:?}", command);
        self.compositor.execute_command(command);
    }

    /// This method is called when changing cursor surface was requested.
    pub fn on_cursor_surface_change(&mut self, sid: SurfaceId) {
        self.pointer.borrow_mut().on_surface_change(sid);
    }

    /// This method is called when changing background surface was requested.
    ///
    /// TODO: Make change background request be display specific.
    pub fn on_background_surface_change(&mut self, sid: SurfaceId) {
        for ref mut display in self.displays.values_mut() {
            display.on_background_change(sid);
        }
    }

    /// This method is called when new surface is ready to be managed.
    pub fn on_surface_ready(&mut self, sid: SurfaceId) {
        self.compositor.manage_surface(sid);
    }

    /// This method is called when surface was destroyed.
    pub fn on_surface_destroyed(&mut self, sid: SurfaceId) {
        self.compositor.unmanage_surface(sid);
        self.pointer.borrow_mut().on_surface_destroyed(sid);
    }

    /// This method is called when keyboard focus changed.
    pub fn on_keyboard_focus_changed(&mut self, sid: SurfaceId) {
        self.pointer.borrow_mut().on_keyboard_focus_changed(sid);
    }

    /// This method is called when screenshot was requested.
    pub fn take_screenshot(&mut self, id: i32) {
        if let Some(ref mut display) = self.displays.get_mut(&id) {
            if let Some(buffer) = display.take_screenshot() {
                self.coordinator.set_screenshot_buffer(buffer);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Input handlers.
impl<C> Exhibitor<C> where C: ExhibitorCoordinationTrait {
    /// Handle pointer motion event.
    pub fn on_motion(&mut self, vector: Vector) {
        self.pointer.borrow_mut().move_and_cast(vector, &self.displays);
        self.coordinator.notify();
    }

    /// Handle pointer position event.
    pub fn on_position(&mut self, position: OptionalPosition) {
        self.pointer.borrow_mut().update_position(position, &self.displays);
        self.coordinator.notify();
    }

    /// Handle pointer button event.
    pub fn on_button(&mut self, button: Button) {
        // TODO: Be more specific about button codes and values.
        if button.value != 0 {
            let pfsid = self.pointer.borrow_mut().get_pointer_focussed_sid();
            if self.pointer.borrow_mut().get_keyboard_focussed_sid() != pfsid {
                self.compositor.pop_surface(pfsid);
            }
        }
    }

    /// Handle pointer position reset event.
    pub fn on_position_reset(&self) {
        self.pointer.borrow_mut().reset_position()
    }
}

// -------------------------------------------------------------------------------------------------

/// Getters
impl<C> Exhibitor<C> where C: ExhibitorCoordinationTrait {
    /// Returns root frame.
    pub fn get_root(&self) -> frames::Frame {
        self.compositor.get_root()
    }

    /// Returns selected frame.
    pub fn get_selection(&self) -> frames::Frame {
        self.compositor.get_selection()
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper methods
impl<C> Exhibitor<C> where C: ExhibitorCoordinationTrait {
    /// Chooses new display position.
    ///
    /// New position is always chosen to be right to most right display.
    ///
    /// TODO: Choosing new display position should be configurable, scriptable and cacheable.
    /// TODO: Handle reposition of displays when display is lost.
    pub fn choose_new_display_position(&self) -> Position {
        let mut pos = Position::default();
        for ref display in self.displays.values() {
            let area = display.get_info().area;
            let x = area.pos.x + area.size.width as isize;
            if x > pos.x {
                pos.x = x;
            }
        }
        pos
    }
}

// -------------------------------------------------------------------------------------------------
