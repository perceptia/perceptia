// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std;
use std::sync::{Arc, Mutex};

use dharma;

use defs::{Position, Size, Vector, MemoryPoolId, MemoryViewId};
use memory::{Buffer, MappedMemory, MemoryPool, MemoryView};
use perceptron::{self, Perceptron};
use surface::{Surface, SurfaceAccess, SurfaceContext, SurfaceId, SurfaceInfo};
use surface::{show_reason, surface_state};

// -------------------------------------------------------------------------------------------------

type SurfaceMap = std::collections::HashMap<SurfaceId, Surface>;
type MemoryViewMap = std::collections::HashMap<MemoryViewId, MemoryView>;
type MemoryPoolMap = std::collections::HashMap<MemoryPoolId, MemoryPool>;

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

macro_rules! try_get_memory_view {
    ($coordinator:expr, $bid:ident) => {
        match $coordinator.memory_views.get_mut(&$bid) {
            Some(buffer) => buffer,
            None => {
                log_warn2!("Buffer {:?} not found!", $bid);
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
    /// Global signaler
    signaler: dharma::Signaler<Perceptron>,

    /// Storage of all surfaces
    surfaces: SurfaceMap,

    /// Storage for all memory views.
    memory_views: MemoryViewMap,

    /// Storage for all memory pools.
    memory_pools: MemoryPoolMap,

    /// Counter of surface IDs
    last_surface_id: SurfaceId,

    /// Counter of memory view IDs
    last_memory_view_id: MemoryViewId,

    /// Counter of memory pool IDs
    last_memory_pool_id: MemoryPoolId,

    /// Currently keyboard-focused surface ID
    kfsid: SurfaceId,

    /// Currently pointer-focused surface ID
    pfsid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinator {
    /// `InnerCoordinator` constructor.
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        InnerCoordinator {
            signaler: signaler,
            surfaces: SurfaceMap::new(),
            memory_views: MemoryViewMap::new(),
            memory_pools: MemoryPoolMap::new(),
            last_surface_id: SurfaceId::invalid(),
            last_memory_view_id: MemoryViewId::initial(),
            last_memory_pool_id: MemoryPoolId::initial(),
            kfsid: SurfaceId::invalid(),
            pfsid: SurfaceId::invalid(),
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
    pub fn get_buffer(&self, sid: SurfaceId) -> Option<MemoryView> {
        let surface = try_get_surface_or_none!(self, sid);
        surface.get_buffer()
    }

    /// Returns surface context.
    pub fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mut result = Vec::new();
        let surface = try_get_surface_or_none!(self, sid);
        result.push(surface.get_renderer_context());
        Some(result)
    }

    /// Returns ID of currently keyboard-focussed surface.
    pub fn get_keyboard_focused_sid(&self) -> SurfaceId {
        self.kfsid
    }

    /// Informs rest of the application exhibitor set keyboard focus to given surface.
    pub fn set_keyboard_focus(&mut self, sid: SurfaceId) {
        if self.kfsid != sid {
            self.signaler.emit(perceptron::KEYBOARD_FOCUS_CHANGED,
                               Perceptron::KeyboardFocusChanged(self.kfsid, sid));
            self.kfsid = sid;
        }
    }

    /// Returns ID of currently pointer-focussed surface.
    pub fn get_pointer_focused_sid(&self) -> SurfaceId {
        self.pfsid
    }

    /// Informs rest of the application exhibitor set pointer focus to given surface.
    pub fn set_pointer_focus(&mut self, sid: SurfaceId, position: Position) {
        if self.pfsid != sid {
            self.signaler.emit(perceptron::POINTER_FOCUS_CHANGED,
                               Perceptron::PointerFocusChanged(self.pfsid, sid, position));
            self.pfsid = sid;
        }
    }

    /// Creates new memory pool from mapped memory. Returns ID of newly created pool.
    pub fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId {
        let id = self.generate_next_memory_pool_id();
        self.memory_pools.insert(id, MemoryPool::new_from_mapped_memory(memory));
        id
    }

    /// Creates new memory pool from buffer. Returns ID of newly created pool.
    pub fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        let id = self.generate_next_memory_pool_id();
        self.memory_pools.insert(id, MemoryPool::new_from_buffer(buffer));
        id
    }

    /// Schedules destruction of memory pool identified by given ID. The pool will be destructed
    /// when all its views go out of the scope.
    pub fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) {
        self.memory_pools.remove(&mpid);
    }

    /// Creates new memory view from mapped memory.
    pub fn create_memory_view(&mut self,
                              mpid: MemoryPoolId,
                              offset: usize,
                              width: usize,
                              height: usize,
                              stride: usize)
                              -> Option<MemoryViewId> {
        let id = self.generate_next_memory_view_id();
        if let Some(memory_pool) = self.memory_pools.get(&mpid) {
            let memory_view = memory_pool.get_memory_view(offset, width, height, stride);
            self.memory_views.insert(id, memory_view);
            Some(id)
        } else {
            log_error!("No memory pool with ID {:?}", mpid);
            None
        }
    }

    /// Creates new surface with newly generated unique ID.
    pub fn create_surface(&mut self) -> SurfaceId {
        let id = self.generate_next_surface_id();
        self.surfaces.insert(id, Surface::new(&id));
        id
    }

    /// Informs other parts of application about request from client to destroy surface.
    pub fn destroy_surface(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::SURFACE_DESTROYED,
                           Perceptron::SurfaceDestroyed(sid));
    }

    /// Sets given buffer as pending for given surface.
    pub fn attach(&mut self, mvid: MemoryViewId, sid: SurfaceId) {
        let surface = try_get_surface!(self, sid);
        let view = try_get_memory_view!(self, mvid);
        surface.attach(view.clone());
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
    pub fn show_surface(&mut self, sid: SurfaceId, reason: show_reason::ShowReason) {
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
    pub fn set_surface_relative_position(&self, _sid: SurfaceId, _offset: Vector) {
        unimplemented!()
    }

    // FIXME: Finish implementation of Coordinator
    pub fn relate_surfaces(&self, _sid: SurfaceId, _parent_sid: SurfaceId) {
        unimplemented!()
    }

    /// Informs other parts of application about request from client to change cursor surface.
    pub fn set_surface_as_cursor(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::CURSOR_SURFACE_CHANGE,
                           Perceptron::CursorSurfaceChange(sid));
    }

    /// Reconfigure surface and send notification about this event.
    pub fn reconfigure(&mut self,
                       sid: SurfaceId,
                       size: Size,
                       state_flags: surface_state::SurfaceState) {
        let surface = try_get_surface!(self, sid);
        if (surface.get_desired_size() != size) || (surface.get_state_flags() != state_flags) {
            surface.set_desired_size(size);
            surface.set_state_flags(state_flags);
            self.signaler.emit(perceptron::SURFACE_RECONFIGURED,
                               Perceptron::SurfaceReconfigured(sid));
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Helper functions
impl InnerCoordinator {
    // FIXME: Finish implementation of Coordinator (counter)
    fn generate_next_surface_id(&mut self) -> SurfaceId {
        self.last_surface_id = SurfaceId::new(self.last_surface_id.as_number() as u64 + 1);
        self.last_surface_id
    }
    // FIXME: Finish implementation of Coordinator
    fn generate_next_memory_pool_id(&mut self) -> MemoryPoolId {
        self.last_memory_pool_id.increment()
    }
    // FIXME: Finish implementation of Coordinator
    fn generate_next_memory_view_id(&mut self) -> MemoryViewId {
        self.last_memory_view_id.increment()
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
        let mine = self.inner.lock().unwrap();
        mine.get_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_buffer(&self, sid: SurfaceId) -> Option<MemoryView> {
        let mine = self.inner.lock().unwrap();
        mine.get_buffer(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mine = self.inner.lock().unwrap();
        mine.get_renderer_context(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_keyboard_focused_sid(&self) -> SurfaceId {
        let mine = self.inner.lock().unwrap();
        mine.get_keyboard_focused_sid()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_keyboard_focus(&mut self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_keyboard_focus(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_pointer_focused_sid(&self) -> SurfaceId {
        let mine = self.inner.lock().unwrap();
        mine.get_pointer_focused_sid()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn set_pointer_focus(&mut self, sid: SurfaceId, position: Position) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_pointer_focus(sid, position)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_pool_from_memory(memory)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_pool_from_buffer(buffer)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_memory_pool(mpid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn create_memory_view(&mut self,
                              mpid: MemoryPoolId,
                              offset: usize,
                              width: usize,
                              height: usize,
                              stride: usize)
                              -> Option<MemoryViewId> {
        let mut mine = self.inner.lock().unwrap();
        mine.create_memory_view(mpid, offset, width, height, stride)
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
    pub fn attach(&self, mvid: MemoryViewId, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach(mvid, sid);
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
        mine.set_surface_as_cursor(sid);
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccess for Coordinator {
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState) {
        let mut mine = self.inner.lock().unwrap();
        mine.reconfigure(sid, size, state_flags);
    }
}

// -------------------------------------------------------------------------------------------------
