// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to pointer like tracking position or setting surface.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use qualia::{Buffer, Coordinator, Area, Button, OptionalPosition, Position, Vector, SurfaceId};

use display::Display;

// -------------------------------------------------------------------------------------------------

const DEFAULT_CURSOR_SIZE: usize = 15;

// -------------------------------------------------------------------------------------------------

/// State of the pointer.
pub struct Pointer {
    /// Position in global coordinates.
    position: Position,

    /// Last position received from input device.
    last_pos: OptionalPosition,

    /// Area of display on which the pointer is placed.
    display_area: Area,

    /// Surface ID of cursor surface.
    csid: SurfaceId,

    /// Default surface ID of cursor surface.
    default_csid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl Pointer {
    /// `Pointer` constructor.
    pub fn new(coordinator: &mut Coordinator) -> Self {
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
            last_pos: OptionalPosition::default(),
            display_area: Area::default(),
            csid: default_csid,
            default_csid: default_csid,
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Getters
impl Pointer {
    /// Get position in global coordinates.
    pub fn get_global_position(&self) -> Position {
        self.position.clone()
    }

    /// Get ID of the cursor surface.
    pub fn get_sid(&self) -> SurfaceId {
        self.csid.clone()
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
            if let Some(last_x) = self.last_pos.x {
                vector.x = x - last_x;
            }
            self.last_pos.x = Some(x);
        }

        // Calculate Y-axis part of position
        if let Some(y) = pos.y {
            if let Some(last_y) = self.last_pos.y {
                vector.y = y - last_y;
            }
            self.last_pos.y = Some(y);
        }

        // Update position
        self.move_and_cast(vector.clone(), displays);
    }

    /// Reset position of the pointer.
    pub fn reset_position(&mut self) {
        self.last_pos = OptionalPosition::default()
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
