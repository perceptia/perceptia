// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std;
use std::sync::{Arc, Mutex};

use dharma;

use defs::{Size, Vector};
use buffer::Buffer;
use perceptron::{self, Perceptron};
use surface::{Surface, SurfaceAccess, SurfaceContext, SurfaceId, SurfaceInfo, show_reason};

// -------------------------------------------------------------------------------------------------

type Map = std::collections::HashMap<SurfaceId, Surface>;

// -------------------------------------------------------------------------------------------------

macro_rules! try_get_surface {
    ($coordinator:expr, $sid:ident) => {
        match $coordinator.surfaces.get_mut(&$sid) {
            Some(surface) => surface,
            None => {
                log_warn2!("Surface {} not found!", $sid);
                return
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! try_get_surface_or_none {
    ($coordinator:expr, $sid:ident) => {
        match $coordinator.surfaces.get(&$sid) {
            Some(surface) => surface,
            None => {
                log_warn2!("Surface {} not found!", $sid);
                return None
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure contains logic related to maintaining shared application state about surfaces.
/// For thread-safe public version see `Coordinator`.
struct InnerCoordinator {
    signaler: dharma::Signaler<Perceptron>,
    surfaces: Map,
    last_id: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinator {
    /// `InnerCoordinator` constructor.
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        InnerCoordinator {
            signaler: signaler,
            surfaces: Map::new(),
            last_id: SurfaceId::invalid(),
        }
    }

    /// Notifies coordinator about event that requires screen to be refreshed.
    pub fn notify(&mut self) {
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Returns information about surface.
    pub fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let surface = try_get_surface_or_none!(self, sid);
        Some(surface.get_info())
    }

    /// Returns buffer of the surface.
    pub fn get_buffer(&self, sid: SurfaceId) -> Option<Arc<Buffer>> {
        let surface = try_get_surface_or_none!(self, sid);
        Some(surface.get_buffer())
    }

    /// Returns surface context.
    pub fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mut result = Vec::new();
        let surface = try_get_surface_or_none!(self, sid);
        result.push(surface.get_renderer_context());
        Some(result)
    }

    /// Creates new surface with newly generated unique ID.
    pub fn create_surface(&mut self) -> SurfaceId {
        let id = self.generate_next_surface_id();
        self.surfaces.insert(id, Surface::new(&id));
        id
    }

    // FIXME: Finish implementation of Coordinator
    pub fn destroy_surface(&self, sid: SurfaceId) {
        unimplemented!()
    }

    /// Sets given buffer as pending for given surface.
    pub fn attach(&mut self, sid: SurfaceId, buffer: Buffer) {
        let surface = try_get_surface!(self, sid);
        surface.attach(buffer)
    }

    /// Sets pending buffer of given surface as current. Corrects sizes adds `drawable` show reason.
    pub fn commit_surface(&mut self, sid: SurfaceId) {
        if {
            let surface = try_get_surface!(self, sid);
            surface.commit()
        } {
            self.show_surface(sid, show_reason::DRAWABLE);
        }
    }

    /// Adds given show reason flag to set of surfaces show reason. If all reasons needed for
    /// surface to be drawn are meet, emit signal `surface ready`.
    pub fn show_surface(&mut self, sid: SurfaceId, reason: i32) {
        let surface = try_get_surface!(self, sid);
        if surface.show(reason) == show_reason::READY {
            self.signaler.emit(perceptron::SURFACE_READY, Perceptron::SurfaceReady(sid));
        }
    }

    /// Sets position offset given surface.
    pub fn set_surface_offset(&mut self, sid: SurfaceId, offset: Vector) {
        let surface = try_get_surface!(self, sid);
        surface.set_offset(offset)
    }

    /// Sets requested size for given surface.
    pub fn set_surface_requested_size(&mut self, sid: SurfaceId, size: Size) {
        let surface = try_get_surface!(self, sid);
        surface.set_requested_size(size)
    }

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {
        unimplemented!()
    }

    // FIXME: Finish implementation of Coordinator
    pub fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        unimplemented!()
    }

    /// Inform other parts of application about request from client to change cursor surface.
    pub fn set_surface_as_cursor(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::CURSOR_SURFACE_CHANGE, Perceptron::CursorSurfaceChange(sid));
    }
}

// -------------------------------------------------------------------------------------------------

// Helper functions
impl InnerCoordinator {
    // FIXME: Finish implementation of Coordinator
    fn generate_next_surface_id(&mut self) -> SurfaceId {
        self.last_id = SurfaceId::new(self.last_id.as_number() as u64 + 1);
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
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        Coordinator { inner: Arc::new(Mutex::new(InnerCoordinator::new(signaler))) }
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn notify(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.notify()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let mut mine = self.inner.lock().unwrap();
        mine.get_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_buffer(&self, sid: SurfaceId) -> Option<Arc<Buffer>> {
        let mut mine = self.inner.lock().unwrap();
        mine.get_buffer(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mut mine = self.inner.lock().unwrap();
        mine.get_renderer_context(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn create_surface(&mut self) -> SurfaceId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_surface()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn destroy_surface(&self, sid: SurfaceId) {
        let mine = self.inner.lock().unwrap();
        mine.destroy_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn attach(&self, sid: SurfaceId, buffer: Buffer) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach(sid, buffer);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn commit_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.commit_surface(sid);
        mine.notify();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn show_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {
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
        let mine = self.inner.lock().unwrap();
        mine.set_surface_relative_position(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        let mine = self.inner.lock().unwrap();
        mine.relate_surfaces(sid, parent_sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_surface_as_cursor(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_as_cursor(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccess for Coordinator {
    fn configure(&mut self, sid: SurfaceId, i: i32) {}
}

// -------------------------------------------------------------------------------------------------
