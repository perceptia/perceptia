// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std;
use std::sync::{Arc, Mutex};

use defs::{Size, Vector};
use buffer::Buffer;

// -------------------------------------------------------------------------------------------------

pub type SurfaceId = u64;
type Map = std::collections::HashMap<SurfaceId, Surface>;

// -------------------------------------------------------------------------------------------------

/// This enum describes readiness of `Surface` to be displayed.
pub enum ShowReason {
    UNINITIALIZED,
    DRAWABLE,
    IN_SHELL,
    READY,
}

// -------------------------------------------------------------------------------------------------

/// This structure represents surface.
struct Surface {
    id: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    /// `Surface` constructor.
    pub fn new(id: &SurfaceId) -> Self {
        Surface {
            id: *id,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure contains logic related to maintaining shared application state about surfaces.
/// For thread-safe public version see `Coordinator`.
struct InnerCoordinator {
    last_id: SurfaceId,
    surfaces: Map,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinator {
    /// `InnerCoordinator` constructor.
    pub fn new() -> Self {
        InnerCoordinator {
            last_id: 0,
            surfaces: Map::new(),
        }
    }

    // FIXME: Finish implementation of Coordinator
    pub fn create_surface(&mut self) -> SurfaceId {
        let id = self.generate_next_surface_id();
        self.surfaces.insert(id, Surface::new(&id));
        id
    }

    // FIXME: Finish implementation of Coordinator
    pub fn destroy_surface(&self, sid: SurfaceId) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn attach(&self, sid: SurfaceId, buffer: Buffer) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn commit_surface(&self, sid: SurfaceId) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn show_surface(&self, sid: SurfaceId, reason: ShowReason) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_offset(&self, sid: SurfaceId, offset: Vector) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_requested_size(&self, sid: SurfaceId, size: Size) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
    }

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_as_cursor(&self, sid: SurfaceId) {
    }

    // FIXME: Finish implementation of Coordinator
    fn generate_next_surface_id(&mut self) -> SurfaceId {
        self.last_id += 1;
        self.last_id
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for locking `InnerCoordinator` shared between threads.
#[derive(Clone)]
pub struct Coordinator {
    inner: Arc<Mutex<InnerCoordinator>>,
}

// -------------------------------------------------------------------------------------------------

impl Coordinator {
    /// `Coordinator` constructor.
    pub fn new() -> Self {
        Coordinator {
            inner: Arc::new(Mutex::new(InnerCoordinator::new())),
        }
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn create_surface(&mut self) -> SurfaceId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_surface()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn destroy_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn attach(&self, sid: SurfaceId, buffer: Buffer) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach(sid, buffer)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn commit_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.commit_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn show_surface(&self, sid: SurfaceId, reason: ShowReason) {
        let mut mine = self.inner.lock().unwrap();
        mine.show_surface(sid, reason)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_surface_offset(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_offset(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_surface_requested_size(&self, sid: SurfaceId, size: Size) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_requested_size(sid, size)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_relative_position(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.relate_surfaces(sid, parent_sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_surface_as_cursor(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_as_cursor(sid)
    }
}

// -------------------------------------------------------------------------------------------------
