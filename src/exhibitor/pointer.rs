// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to pointer like tracking position or setting surface.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use dharma::Signaler;

use qualia::{Buffer, Coordinator, Area, OptionalPosition, Position, Vector, SurfaceId, SurfaceContext, SurfacePosition, perceptron, Perceptron};

use display::Display;

// -------------------------------------------------------------------------------------------------

const DEFAULT_CURSOR_SIZE: usize = 15;

// -------------------------------------------------------------------------------------------------

/// State of the pointer.
pub struct Pointer {
    /// Position in global coordinates.
    position: Position,

    /// Last position received from input device.
    last_position: OptionalPosition,

    /// Last relative position inside focused surface.
    last_surface_relative: Position,

    /// Area of display on which the pointer is placed.
    display_area: Area,

    /// Surface ID of cursor surface.
    csid: SurfaceId,

    /// Surface ID of pointer-focused surface.
    pfsid: SurfaceId,

    /// Surface ID of keyboard-focused surface.
    kfsid: SurfaceId,

    /// Default surface ID of cursor surface.
    default_csid: SurfaceId,

    /// Signaler.
    signaler: Signaler<Perceptron>,

    /// Coordinator.
    coordinator: Coordinator,
}

// -------------------------------------------------------------------------------------------------

impl Pointer {
    /// `Pointer` constructor.
    pub fn new(signaler: Signaler<Perceptron>, mut coordinator: Coordinator) -> Self {
        let mut data = vec![200; 4 * DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE];
        for z in 0..(DEFAULT_CURSOR_SIZE * DEFAULT_CURSOR_SIZE) {
            data[4 * z + 3] = 100;
        }

        let default_csid = coordinator.create_surface();
        let b = Buffer::new(DEFAULT_CURSOR_SIZE,
                            DEFAULT_CURSOR_SIZE,
                            4 * DEFAULT_CURSOR_SIZE,
                            data);
        coordinator.attach(default_csid, b);
        coordinator.commit_surface(default_csid);

        Pointer {
            position: Position::default(),
            last_position: OptionalPosition::default(),
            last_surface_relative: Position::default(),
            display_area: Area::default(),
            csid: default_csid,
            pfsid: SurfaceId::invalid(),
            kfsid: SurfaceId::invalid(),
            default_csid: default_csid,
            signaler: signaler,
            coordinator: coordinator,
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Getters
impl Pointer {
    /// Returns pointer position in global coordinates.
    pub fn get_global_position(&self) -> Position {
        self.position.clone()
    }

    /// Returns ID of the cursor surface.
    pub fn get_cursor_sid(&self) -> SurfaceId {
        self.csid
    }

    /// Return ID of the surface with keyboard focus.
    pub fn get_keyboard_focussed_sid(&self) -> SurfaceId {
        self.kfsid
    }

    /// Return ID of the surface with pointer focus.
    pub fn get_pointer_focussed_sid(&self) -> SurfaceId {
        self.pfsid
    }
}

// -------------------------------------------------------------------------------------------------

/// Input handlers.
impl Pointer {
    /// Move pointer and cast to correct output.
    pub fn move_and_cast(&mut self, vector: Vector, displays: &HashMap<i32, Display>) {
        let moved = self.position.clone() + vector.clone();
        self.position = self.cast(moved, displays);
    }

    /// Change position of the pointer and cast to correct output.
    pub fn update_position(&mut self, pos: OptionalPosition, displays: &HashMap<i32, Display>) {
        let mut vector = Vector::default();

        // Calculate X-axis part of position
        if let Some(x) = pos.x {
            if let Some(last_x) = self.last_position.x {
                vector.x = x - last_x;
            }
            self.last_position.x = Some(x);
        }

        // Calculate Y-axis part of position
        if let Some(y) = pos.y {
            if let Some(last_y) = self.last_position.y {
                vector.y = y - last_y;
            }
            self.last_position.y = Some(y);
        }

        // Update position
        self.move_and_cast(vector.clone(), displays);
    }

    /// Reset position of the pointer.
    pub fn reset_position(&mut self) {
        self.last_position = OptionalPosition::default()
    }

    /// Checks for change of surface pointer is hovering or relative position to this surface and
    /// notify rest of the application about changes.
    pub fn update_hover_state(&mut self,
                              display_area: Area,
                              surfaces: &Vec<SurfaceContext>) {
        // Check if this update is for display on which this pointer is placed
        if self.display_area != display_area {
            return;
        }

        let mut sid = SurfaceId::invalid();
        let mut surface_relative = Position::default();
        let display_relative = Position::new(self.position.x - display_area.pos.x,
                                             self.position.y - display_area.pos.y);

        // Find surface pointer hovers
        for context in surfaces {
            if let Some(info) = self.coordinator.get_surface(context.id) {
                let surface_area = Area::new(context.pos.clone(), info.requested_size);
                if surface_area.contains(&display_relative) {
                    sid = context.id;
                    surface_relative = display_relative - context.pos.clone() + info.offset;
                    break;
                }
            }
        }

        // Handle focus change if hovered surface is different than current one or handle motion
        // otherwise
        let surface_position = SurfacePosition::new(sid, surface_relative.clone());
        if sid != self.pfsid {
            self.pfsid = sid;
            self.csid = self.default_csid;
            self.signaler.emit(perceptron::POINTER_FOCUS_CHANGED,
                               Perceptron::PointerFocusChanged(surface_position));
        } else if self.pfsid.is_valid() && (surface_relative != self.last_surface_relative) {
            self.last_surface_relative = surface_relative;
            self.signaler.emit(perceptron::POINTER_RELATIVE_MOTION,
                               Perceptron::PointerRelativeMotion(surface_position));
        }
    }

    /// Handles destruction of cursor surface.
    pub fn on_surface_destroyed(&mut self, sid: SurfaceId) {
        if self.csid == sid {
            self.csid = SurfaceId::invalid();
        }
    }

    /// Sets surface ID of currently keyboard focused surface.
    pub fn on_keyboard_focus_changed(&mut self, sid: SurfaceId) {
        self.kfsid = sid;
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper methods
impl Pointer {
    /// Cast position to one of available displays:
    /// - if position is in one of the displays - return it without change
    /// - otherwise cast it to last used display.
    fn cast(&mut self, mut position: Position, displays: &HashMap<i32, Display>) -> Position {
        if !self.display_area.contains(&position) {
            let mut found = false;
            // Iterate display to find the one display is in
            for display in displays.values() {
                let area = display.get_area();
                if area.contains(&position) {
                    // Set new active output and exit
                    self.display_area = area;
                    found = true;
                    break;
                }
            }

            if !found {
                // Pointer outside any known output - cast it to the previous active
                position = position.casted(&self.display_area);
            }
        }
        position
    }
}

// -------------------------------------------------------------------------------------------------
