// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `Aesthetics` manages tasks related to visual appearance. It uses the same API as exposed to
//! client frontends (i.e. it only provides surfaces that `Exhibitor` will draw).

// -------------------------------------------------------------------------------------------------

extern crate image;

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;

mod cursor;
mod background;

use qualia::{SurfaceId, AestheticsConfig, AestheticsCoordinationTrait};

use cursor::Cursor;
use background::Background;

// -------------------------------------------------------------------------------------------------

/// `Aesthetics` manages tasks related to visual appearance. It uses the same API as exposed to
/// client frontends.
pub struct Aesthetics<C>
    where C: AestheticsCoordinationTrait
{
    cursor: Cursor<C>,
    background: Background<C>,
}

// -------------------------------------------------------------------------------------------------

/// General methods.
impl<C> Aesthetics<C>
    where C: AestheticsCoordinationTrait + Clone
{
    /// Constructs new `Aesthetics`.
    pub fn new(coordinator: C, config: AestheticsConfig) -> Self {
        Aesthetics {
            cursor: Cursor::new(coordinator.clone()),
            background: Background::new(coordinator.clone(), config),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Notification handlers.
impl<C> Aesthetics<C>
    where C: AestheticsCoordinationTrait + Clone
{
    /// This method is called when changing cursor surface was requested.
    pub fn on_cursor_surface_change(&mut self, sid: SurfaceId) {
        self.cursor.on_surface_change(sid);
    }

    /// This method is called when changing background surface was requested.
    pub fn on_background_surface_change(&mut self, sid: SurfaceId) {
        self.background.on_surface_change(sid);
    }

    /// This method is called when pointer focus changed.
    pub fn on_pointer_focus_changed(&mut self, old_pfsid: SurfaceId, new_pfsid: SurfaceId) {
        self.cursor.on_focus_changed(old_pfsid, new_pfsid);
    }

    /// This method is called when surface was destroyed.
    pub fn on_surface_destroyed(&mut self, sid: SurfaceId) {
        self.cursor.on_surface_destroyed(sid);
    }

    /// This method is called when new display was created.
    pub fn on_display_created(&mut self) {
        self.cursor.on_display_created();
        self.background.on_display_created();
    }
}

// -------------------------------------------------------------------------------------------------
