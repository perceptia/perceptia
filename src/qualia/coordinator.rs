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
pub use defs::SurfaceId;

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

/// These flags describe readiness of `Surface` to be displayed.
pub mod show_reason {
    pub type ShowReason = i32;
    pub const UNINITIALIZED: ShowReason = 0b0000;
    pub const DRAWABLE:      ShowReason = 0b0001;
    pub const IN_SHELL:      ShowReason = 0b0010;
    pub const READY:         ShowReason = DRAWABLE | IN_SHELL;
}

// -------------------------------------------------------------------------------------------------

/// This structure represents surface.
struct Surface {
    /// ID of the surface.
    id: SurfaceId,

    /// Offset used to move coordinate system of surface.
    offset: Vector,

    /// Size desired by compositor.
    desired_size: Size,

    /// Size requested by client.
    requested_size: Size,

    /// ID of parent surface.
    parent_sid: SurfaceId,

    /// Data required for draw.
    buffer: Buffer,

    /// Data to be used after commit.
    pending_buffer: Buffer,

    /// Flags indicating if surface is ready to be shown.
    show_reasons: show_reason::ShowReason,
}

// -------------------------------------------------------------------------------------------------

impl Surface {
    /// `Surface` constructor.
    pub fn new(id: &SurfaceId) -> Self {
        Surface {
            id: *id,
            offset: Vector::default(),
            desired_size: Size::default(),
            requested_size: Size::default(),
            parent_sid: SurfaceId::invalid(),
            buffer: Buffer::empty(),
            pending_buffer: Buffer::empty(),
            show_reasons: show_reason::UNINITIALIZED,
        }
    }

    /// Sets position offset.
    pub fn set_offset(&mut self, offset: Vector) {
        self.offset.x = if offset.x > 0 { offset.x } else { 0 };
        self.offset.y = if offset.y > 0 { offset.y } else { 0 };
    }

    /// Sets size requested by client.
    #[inline]
    pub fn set_requested_size(&mut self, size: Size) {
        self.requested_size = size
    }

    /// Adds given reason to show reasons. Returns updates set of reasons.
    #[inline]
    pub fn show(&mut self, reason: show_reason::ShowReason) -> show_reason::ShowReason {
        self.show_reasons |= reason;
        self.show_reasons
    }

    /// Sets given buffer as pending.
    #[inline]
    pub fn attach(&mut self, buffer: Buffer) {
        self.pending_buffer = buffer
    }

    /// Sets pending buffer as current. If surface was committed for the first time and sizes are
    /// not set, assign size of buffer as requested size. Return `true` if surface was committed for
    /// the first time, `false` otherwise.
    pub fn commit(&mut self) -> bool {
        let is_first_time_commited = self.buffer.is_empty();
        self.buffer.assign_from(&self.pending_buffer);

        // If surface was just created...
        if is_first_time_commited {
            // ... size was not yet requested by surface ...
            if !((self.requested_size.width == 0) || (self.requested_size.height == 0)) {
                // ... use its buffer size as requested size ...
                self.requested_size = self.buffer.get_size();
            }
            // ... and if it is subsurface ...
            if self.parent_sid.is_valid() {
                // ... set its desired size.
                self.desired_size = self.buffer.get_size();
            }
        }

        is_first_time_commited
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

    // FIXME: Finish implementation of Coordinator
    pub fn set_surface_as_cursor(&self, sid: SurfaceId) {
        unimplemented!()
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
        mine.attach(sid, buffer)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn commit_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.commit_surface(sid)
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
        let mine = self.inner.lock().unwrap();
        mine.set_surface_as_cursor(sid)
    }
}

// -------------------------------------------------------------------------------------------------
