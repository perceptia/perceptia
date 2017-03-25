// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides interface for requests from the rest of application to clients.

// -------------------------------------------------------------------------------------------------

use qualia::{Axis, Button, Key, KeyMods, Milliseconds, Position, Size, SurfaceId};
use qualia::surface_state;

// -------------------------------------------------------------------------------------------------

pub trait Gateway {
    /// Notifies output was found.
    fn on_output_found(&self);

    /// Notifies keyboard key was pressed.
    fn on_keyboard_input(&mut self, key: Key, mods: Option<KeyMods>);

    /// Notifies about redrawing surface.
    fn on_surface_frame(&mut self, sid: SurfaceId, milliseconds: Milliseconds);

    /// Notifies that pointer was moved from above one surface above another.
    fn on_pointer_focus_changed(&self, old_sid: SurfaceId, new_sid: SurfaceId, position: Position);

    /// Notifies that pointer moved.
    fn on_pointer_relative_motion(&self,
                                  sid: SurfaceId,
                                  position: Position,
                                  milliseconds: Milliseconds);

    /// Notifies mouse or touchpad button was pressed.
    fn on_pointer_button(&self, btn: Button);

    /// Notifies about pointer move.
    fn on_pointer_axis(&self, axis: Axis);

    /// Notifies about keyboard focus change.
    fn on_keyboard_focus_changed(&mut self, old_sid: SurfaceId, new_sid: SurfaceId);

    /// Notifies about change of size or state of surface.
    fn on_surface_reconfigured(&self,
                               sid: SurfaceId,
                               size: Size,
                               state_flags: surface_state::SurfaceState);
}

// -------------------------------------------------------------------------------------------------
