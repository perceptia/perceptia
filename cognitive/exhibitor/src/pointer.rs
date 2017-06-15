// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to pointer like tracking position or setting surface.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;
use std::time::Instant;

use qualia::{Area, Milliseconds, OptionalPosition, Position, Vector};
use qualia::{perceptron, Perceptron, SurfaceContext, SurfaceId, ExhibitorCoordinationTrait};

use display::Display;

// -------------------------------------------------------------------------------------------------

/// State of the pointer.
pub struct Pointer<C>
    where C: ExhibitorCoordinationTrait
{
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

    /// Reference time for obtaining event timestamps.
    reference_time: Instant,

    /// Coordinator.
    coordinator: C,
}

// -------------------------------------------------------------------------------------------------

impl<C> Pointer<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Pointer` constructor.
    pub fn new(reference_time: Instant, coordinator: C) -> Self {
        Pointer {
            position: Position::default(),
            last_position: OptionalPosition::default(),
            last_surface_relative: Position::default(),
            display_area: Area::default(),
            csid: SurfaceId::invalid(),
            pfsid: SurfaceId::invalid(),
            kfsid: SurfaceId::invalid(),
            reference_time: reference_time,
            coordinator: coordinator,
        }
    }

    pub fn change_display(&mut self, area: Area) {
        self.position = area.calculate_center();
        self.display_area = area;
    }
}

// -------------------------------------------------------------------------------------------------

// Getters
impl<C> Pointer<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Pointer` constructor.
    /// Returns pointer position in global coordinates.
    pub fn get_global_position(&self) -> Position {
        self.position
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
impl<C> Pointer<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Pointer` constructor.
    /// Move pointer and cast to correct output.
    pub fn move_and_cast(&mut self, vector: Vector, displays: &HashMap<i32, Display<C>>) {
        let moved = self.position.clone() + vector.clone();
        self.position = self.cast(moved, displays);
    }

    /// Change position of the pointer and cast to correct output.
    pub fn update_position(&mut self, pos: OptionalPosition, displays: &HashMap<i32, Display<C>>) {
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
    pub fn reset_position(&mut self,
                          position: Option<Position>,
                          displays: &HashMap<i32, Display<C>>) {
        self.last_position = OptionalPosition::default();
        if let Some(position) = position {
            self.position = self.cast(position, displays);
        }
    }

    /// Checks for change of surface pointer is hovering or relative position to this surface and
    /// notify rest of the application about changes.
    pub fn update_hover_state(&mut self, display_area: Area, surfaces: &Vec<SurfaceContext>) {
        // Check if this update is for display on which this pointer is placed
        if self.display_area != display_area {
            return;
        }

        let mut sid = SurfaceId::invalid();
        let mut surface_relative = Position::default();
        let display_relative = self.position - display_area.pos;

        // Find surface pointer hovers
        for context in surfaces.iter().rev() {
            if let Some(info) = self.coordinator.get_surface(context.id) {
                let surface_area = Area::new(context.pos, info.requested_size);
                if surface_area.contains(&display_relative) {
                    sid = context.id;
                    surface_relative = display_relative - context.pos.clone() + info.offset;
                    break;
                }
            }
        }

        // Handle focus change if hovered surface is different than current one or handle motion
        // otherwise
        if sid != self.pfsid {
            self.pfsid = sid;
            self.csid = SurfaceId::invalid();
            self.coordinator.set_pointer_focus(sid, surface_relative)
        } else if self.pfsid.is_valid() && (surface_relative != self.last_surface_relative) {
            let ms = Milliseconds::elapsed_from(&self.reference_time);
            self.last_surface_relative = surface_relative;
            self.coordinator.emit(perceptron::POINTER_RELATIVE_MOTION,
                                  Perceptron::PointerRelativeMotion(sid, surface_relative, ms));
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Other requests.
impl<C> Pointer<C>
    where C: ExhibitorCoordinationTrait
{
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

    /// Handles cursor surface change request.
    pub fn on_surface_change(&mut self, sid: SurfaceId) {
        self.csid = sid;
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper methods
impl<C> Pointer<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Pointer` constructor.
    /// Cast position to one of available displays:
    /// - if position is in one of the displays - return it without change
    /// - otherwise cast it to last used display.
    fn cast(&mut self, mut position: Position, displays: &HashMap<i32, Display<C>>) -> Position {
        if !self.display_area.contains(&position) {
            let mut found = false;
            // Iterate display to find the one display is in
            for display in displays.values() {
                let area = display.get_info().area;
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
