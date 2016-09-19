// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module gathers functionality related to surfaces.

// -------------------------------------------------------------------------------------------------

use defs::Position;
use coordinator::SurfaceId;

// -------------------------------------------------------------------------------------------------

/// This structure defines how the surface should be drawn.
#[derive(Debug)]
pub struct SurfaceContext {
    pub id: SurfaceId,
    pub pos: Position,
}

// -------------------------------------------------------------------------------------------------

impl SurfaceContext {
    /// `SurfaceContext` constructor.
    pub fn new(id: SurfaceId, pos: Position) -> SurfaceContext {
        SurfaceContext { id: id, pos: pos }
    }
}

// -------------------------------------------------------------------------------------------------
