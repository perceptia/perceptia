// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std;
use std::sync::{Arc, Mutex};

use dharma;

use qualia::{Position, Size, Vector, DmabufId, EglImageId, MemoryPoolId, MemoryViewId};
use qualia::{Buffer, MappedMemory, MemoryPool, MemoryView, PixelFormat};
use qualia::{EglAttributes, DmabufAttributes, GraphicsManagement};
use qualia::{perceptron, Perceptron};
use qualia::{SurfaceContext, SurfaceId, SurfaceInfo, DataSource};
use qualia::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use qualia::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
use qualia::{AppearanceManagement, Emiter, MemoryManagement, HwGraphics, Screenshooting};
use qualia::{AestheticsCoordinationTrait, ExhibitorCoordinationTrait};
use qualia::{show_reason, surface_state};

use surfaces::Surface;

// -------------------------------------------------------------------------------------------------

/// Bundles memory pool with its views.
///
/// Used to remove views when pool is removed.
struct MemoryPoolBundle {
    pub pool: MemoryPool,
    pub views: std::collections::HashSet<MemoryViewId>,
}

// -------------------------------------------------------------------------------------------------

impl MemoryPoolBundle {
    /// Constructs new `MemoryPoolBundle`.
    pub fn new(pool: MemoryPool) -> Self {
        MemoryPoolBundle {
            pool: pool,
            views: std::collections::HashSet::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Bundles memory view with its pool.
///
/// Used to unrelate view from pool when view is removed.
struct MemoryViewBundle {
    pub view: MemoryView,
    pub pool: MemoryPoolId,
}

// -------------------------------------------------------------------------------------------------

impl MemoryViewBundle {
    /// Constructs new `MemoryViewBundle`.
    pub fn new(view: MemoryView, pool: MemoryPoolId) -> Self {
        MemoryViewBundle {
            view: view,
            pool: pool,
        }
    }
}

// -------------------------------------------------------------------------------------------------

type SurfaceMap = std::collections::HashMap<SurfaceId, Surface>;
type MemoryViewMap = std::collections::HashMap<MemoryViewId, MemoryViewBundle>;
type MemoryPoolMap = std::collections::HashMap<MemoryPoolId, MemoryPoolBundle>;
type EglImagesMap = std::collections::HashMap<EglImageId, EglAttributes>;
type DmabufsMap = std::collections::HashMap<DmabufId, DmabufAttributes>;

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

macro_rules! try_get_egl_image_attrs {
    ($coordinator:expr, $ebid:ident) => {
        match $coordinator.egl_images.get_mut(&$ebid) {
            Some(image) => image,
            None => {
                log_warn2!("Image {:?} not found!", $ebid);
                return
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! try_get_dmabuf_attrs {
    ($coordinator:expr, $dbid:ident) => {
        match $coordinator.dmabufs.get_mut(&$dbid) {
            Some(image) => image,
            None => {
                log_warn2!("Dmabuf {:?} not found!", $dbid);
                return
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

    /// Storage for EGL images.
    egl_images: EglImagesMap,

    /// Storage for dmabuf images.
    dmabufs: DmabufsMap,

    /// Graphics manager.
    graphics_manager: Option<Box<GraphicsManagement + Send>>,

    /// Screenshot buffer to be shared between threads.
    screenshot_buffer: Option<Buffer>,

    /// Counter of surface IDs
    last_surface_id: SurfaceId,

    /// Counter of memory view IDs
    last_memory_view_id: MemoryViewId,

    /// Counter of memory pool IDs
    last_memory_pool_id: MemoryPoolId,

    /// Counter of hardware image IDs
    last_egl_image_id: EglImageId,

    /// Counter of dmabuf IDs
    last_dmabuf_id: DmabufId,

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
            egl_images: EglImagesMap::new(),
            dmabufs: DmabufsMap::new(),
            graphics_manager: None,
            screenshot_buffer: None,
            last_surface_id: SurfaceId::invalid(),
            last_memory_view_id: MemoryViewId::initial(),
            last_memory_pool_id: MemoryPoolId::initial(),
            last_egl_image_id: EglImageId::initial(),
            last_dmabuf_id: DmabufId::initial(),
            kfsid: SurfaceId::invalid(),
            pfsid: SurfaceId::invalid(),
        }
    }

    /// Returns buffer of the surface.
    pub fn get_data_source(&self, sid: SurfaceId) -> DataSource {
        match self.surfaces.get(&sid) {
            Some(surface) => surface.get_data_source(),
            None => DataSource::None,
        }
    }

    /// Creates new surface with newly generated unique ID.
    pub fn create_surface(&mut self) -> SurfaceId {
        let id = self.generate_next_surface_id();
        self.surfaces.insert(id, Surface::new(&id));
        id
    }

    /// Sets given buffer as pending for given surface.
    pub fn attach_shm(&mut self, mvid: MemoryViewId, sid: SurfaceId) {
        let surface = try_get_surface!(self, sid);
        let view = try_get_memory_view!(self, mvid);
        surface.attach_shm(view.view.clone());
    }

    /// Sets given hardware image as pending for given surface.
    pub fn attach_egl_image(&mut self, ebid: EglImageId, sid: SurfaceId) {
        let surface = try_get_surface!(self, sid);
        let attrs = try_get_egl_image_attrs!(self, ebid);
        surface.attach_egl_image(attrs.clone());
    }

    /// Sets given dmabuf as pending for given surface.
    pub fn attach_dmabuf(&mut self, dbid: DmabufId, sid: SurfaceId) {
        let surface = try_get_surface!(self, sid);
        let attrs = try_get_dmabuf_attrs!(self, dbid);
        surface.attach_dmabuf(attrs.clone());
    }

    /// Informs other parts of application the surface is now not visible.
    pub fn detach_surface(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::SURFACE_DESTROYED, Perceptron::SurfaceDestroyed(sid));
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

    /// Detaches and forgets given surface.
    pub fn destroy_surface(&mut self, sid: SurfaceId) {
        self.detach_surface(sid);
        self.surfaces.remove(&sid);
    }

    /// Adds given show reason flag to set of surfaces show reason. If all reasons needed for
    /// surface to be drawn are meet, emit signal `surface ready`.
    pub fn show_surface(&mut self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let surface = try_get_surface!(self, sid);
        let old_reason = surface.get_show_reason();
        if surface.show(reason) == show_reason::READY && old_reason != show_reason::READY {
            self.signaler.emit(perceptron::SURFACE_READY, Perceptron::SurfaceReady(sid));
        }
    }

    /// Subtracts given show reason flag from set of surfaces show reason. If not all reasons
    /// needed for surface to be drawn are meet, emit signal `surface destroyed`.
    pub fn hide_surface(&mut self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let surface = try_get_surface!(self, sid);
        let old_reason = surface.get_show_reason();
        if surface.hide(reason) != show_reason::READY && old_reason == show_reason::READY {
            self.signaler.emit(perceptron::SURFACE_DESTROYED, Perceptron::SurfaceDestroyed(sid));
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

    /// Sets satellite surface position relative to its parent.
    pub fn set_surface_relative_position(&mut self, sid: SurfaceId, position: Position) {
        let surface = try_get_surface!(self, sid);
        surface.set_relative_position(position)
    }

    /// Relates two surfaces.
    pub fn relate_surfaces(&mut self, sid: SurfaceId, parent_sid: SurfaceId) {
        {
            let mut surface = try_get_surface!(self, sid);
            surface.set_parent_sid(parent_sid);
            surface.set_relative_position(Vector::default());
            surface.hide(show_reason::IN_SHELL);
        } {
            let mut parent_surface = try_get_surface!(self, parent_sid);
            parent_surface.add_satellite(sid);
        }
    }

    /// Unrelates two surfaces.
    pub fn unrelate_surface(&mut self, sid: SurfaceId) {
        let parent_sid = {
            let mut surface = try_get_surface!(self, sid);
            let parent_sid = surface.get_parent_sid();
            surface.set_parent_sid(SurfaceId::invalid());
            parent_sid
        };
        let mut parent_surface = try_get_surface!(self, parent_sid);
        parent_surface.remove_satellite(sid);
    }

    /// Returns information about surface.
    pub fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let surface = try_get_surface_or_none!(self, sid);
        Some(surface.get_info())
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

    /// Returns surface context.
    pub fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let surface = try_get_surface_or_none!(self, sid);
        let mut result = Vec::new();
        for child_sid in surface.get_satellites() {
            if *child_sid == sid {
                result.push(surface.get_renderer_context());
            } else {
                if let Some(ref mut array) = self.get_renderer_context(*child_sid) {
                    result.append(array)
                }
            }
        }
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

    /// Informs other parts of application about request from client to change cursor surface.
    pub fn set_surface_as_cursor(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::CURSOR_SURFACE_CHANGE, Perceptron::CursorSurfaceChange(sid));
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Informs other parts of application about request from client to change background surface.
    pub fn set_surface_as_background(&mut self, sid: SurfaceId) {
        self.signaler.emit(perceptron::BACKGROUND_SURFACE_CHANGE,
                           Perceptron::BackgroundSurfaceChange(sid));
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Emits given signal.
    fn emit(&mut self, id: dharma::SignalId, package: Perceptron) {
        self.signaler.emit(id, package);
    }

    /// Notifies application about event that requires screen to be refreshed.
    pub fn notify(&mut self) {
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Creates new memory pool from mapped memory. Returns ID of newly created pool.
    pub fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId {
        let mpid = self.generate_next_memory_pool_id();
        let bundle = MemoryPoolBundle::new(MemoryPool::new_from_mapped_memory(memory));
        self.memory_pools.insert(mpid, bundle);
        mpid
    }

    /// Creates new memory pool from buffer. Returns ID of newly created pool.
    pub fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        let mpid = self.generate_next_memory_pool_id();
        let bundle = MemoryPoolBundle::new(MemoryPool::new_from_buffer(buffer));
        self.memory_pools.insert(mpid, bundle);
        mpid
    }

    /// Schedules destruction of memory pool identified by given ID. The pool will be destructed
    /// when all its views go out of the scope.
    ///
    /// If the poll was created from mapped memory, returns this memory.
    pub fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<MappedMemory> {
        match self.memory_pools.remove(&mpid) {
            Some(memory_pool) => {
                // Remove all related views
                for mvid in memory_pool.views.iter() {
                    self.memory_views.remove(mvid);
                }

                // Remove the pool
                memory_pool.pool.take_mapped_memory()
            }
            None => None,
        }
    }

    /// Replaces mapped memory with other memory reusing its ID. This method may be used when
    /// client requests memory map resize.
    pub fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory) {
        self.memory_pools.remove(&mpid);
        let bundle = MemoryPoolBundle::new(MemoryPool::new_from_mapped_memory(memory));
        self.memory_pools.insert(mpid, bundle);
    }

    /// Creates new memory view from mapped memory.
    pub fn create_memory_view(&mut self,
                              mpid: MemoryPoolId,
                              format: PixelFormat,
                              offset: usize,
                              width: usize,
                              height: usize,
                              stride: usize)
                              -> Option<MemoryViewId> {
        let id = self.generate_next_memory_view_id();
        if let Some(mut memory_pool) = self.memory_pools.get_mut(&mpid) {
            let view = memory_pool.pool.get_memory_view(format, offset, width, height, stride);
            self.memory_views.insert(id, MemoryViewBundle::new(view, mpid));
            memory_pool.views.insert(id);
            Some(id)
        } else {
            log_error!("No memory pool with ID {:?}", mpid);
            None
        }
    }

    /// Destroys memory view.
    pub fn destroy_memory_view(&mut self, mvid: MemoryViewId) {
        if let Some(view) = self.memory_views.remove(&mvid) {
            if let Some(mut memory_pool) = self.memory_pools.get_mut(&view.pool) {
                memory_pool.views.remove(&mvid);
            }
        }
    }

    /// Makes screenshot request.
    pub fn take_screenshot(&mut self, id: i32) {
        self.signaler.emit(perceptron::TAKE_SCREENSHOT, Perceptron::TakeScreenshot(id));
    }

    /// Sets given buffer as results of screenshot.
    pub fn set_screenshot_buffer(&mut self, buffer: Buffer) {
        self.screenshot_buffer = Some(buffer);
        self.signaler.emit(perceptron::SCREENSHOT_DONE, Perceptron::ScreenshotDone);
    }

    /// Checks if it is possible to create EGL image with given attributes. If so, then stores
    /// attributes and returns ID assigned to them.
    fn create_egl_image(&mut self, attrs: EglAttributes) -> Option<EglImageId> {
        let id = self.generate_next_egl_image_id();
        if let Some(ref mut graphics_manager) = self.graphics_manager {
            let image = graphics_manager.create_egl_image(&attrs);
            if let Some(image) = image {
                let _ = graphics_manager.destroy_hw_image(image);
                self.egl_images.insert(id, attrs);
                Some(id)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Checks if it is possible to import dmabuf with given attributes. If so, then stores
    /// attributes and returns ID assigned to them.
    fn import_dmabuf(&mut self, attrs: DmabufAttributes) -> Option<DmabufId> {
        let id = self.generate_next_dmabuf_id();
        if let Some(ref mut graphics_manager) = self.graphics_manager {
            let image = graphics_manager.import_dmabuf(&attrs);
            if let Some(image) = image {
                let _ = graphics_manager.destroy_hw_image(image);
                self.dmabufs.insert(id, attrs);
                Some(id)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Removes EGL attributes.
    fn destroy_egl_image(&mut self, ebid: EglImageId) {
        self.egl_images.remove(&ebid);
    }

    /// Removes dmabuf attributes.
    fn destroy_dmabuf(&mut self, dbid: DmabufId) {
        self.dmabufs.remove(&dbid);
    }

    /// Returns and forgets screenshot buffer.
    pub fn take_screenshot_buffer(&mut self) -> Option<Buffer> {
        self.screenshot_buffer.take()
    }
}

// -------------------------------------------------------------------------------------------------

// Helper functions
impl InnerCoordinator {
    /// Generates next surface ID.
    fn generate_next_surface_id(&mut self) -> SurfaceId {
        self.last_surface_id = SurfaceId::new(self.last_surface_id.as_number() as u64 + 1);
        self.last_surface_id
    }
    /// Generates next memory pool ID.
    fn generate_next_memory_pool_id(&mut self) -> MemoryPoolId {
        self.last_memory_pool_id.increment()
    }
    /// Generates next memory view ID.
    fn generate_next_memory_view_id(&mut self) -> MemoryViewId {
        self.last_memory_view_id.increment()
    }

    /// Generates next hardware image ID.
    fn generate_next_egl_image_id(&mut self) -> EglImageId {
        self.last_egl_image_id.increment()
    }

    /// Generates next dmabuf ID.
    fn generate_next_dmabuf_id(&mut self) -> DmabufId {
        self.last_dmabuf_id.increment()
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
}

// -------------------------------------------------------------------------------------------------

// TODO: Finish refactoring `Coordinator`: all method should be provided by some trait.
impl Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_data_source(&self, sid: SurfaceId) -> DataSource {
        let mine = self.inner.lock().unwrap();
        mine.get_data_source(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_surface(&mut self) -> SurfaceId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_surface()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_shm(&self, mvid: MemoryViewId, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach_shm(mvid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_egl_image(&self, ebid: EglImageId, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach_egl_image(ebid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_dmabuf(&self, dbid: DmabufId, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.attach_dmabuf(dbid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn detach_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.detach_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn commit_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.commit_surface(sid);
        mine.notify();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_surface(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceControl for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn show_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let mut mine = self.inner.lock().unwrap();
        mine.show_surface(sid, reason)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn hide_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let mut mine = self.inner.lock().unwrap();
        mine.hide_surface(sid, reason)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_offset(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_offset(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_requested_size(&self, sid: SurfaceId, size: Size) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_requested_size(sid, size)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_relative_position(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.relate_surfaces(sid, parent_sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn unrelate_surface(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.unrelate_surface(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceViewer for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let mine = self.inner.lock().unwrap();
        mine.get_surface(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccess for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState) {
        let mut mine = self.inner.lock().unwrap();
        mine.reconfigure(sid, size, state_flags);
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceListing for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mine = self.inner.lock().unwrap();
        mine.get_renderer_context(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceFocusing for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_keyboard_focused_sid(&self) -> SurfaceId {
        let mine = self.inner.lock().unwrap();
        mine.get_keyboard_focused_sid()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_keyboard_focus(&mut self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_keyboard_focus(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_pointer_focused_sid(&self) -> SurfaceId {
        let mine = self.inner.lock().unwrap();
        mine.get_pointer_focused_sid()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_pointer_focus(&mut self, sid: SurfaceId, position: Position) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_pointer_focus(sid, position)
    }
}

// -------------------------------------------------------------------------------------------------

impl AppearanceManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_as_cursor(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_as_cursor(sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_as_background(&self, sid: SurfaceId) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_surface_as_background(sid);
    }
}

// -------------------------------------------------------------------------------------------------

impl Emiter for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn emit(&mut self, id: dharma::SignalId, package: Perceptron) {
        let mut mine = self.inner.lock().unwrap();
        mine.emit(id, package);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn notify(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.notify()
    }
}

// -------------------------------------------------------------------------------------------------

impl MemoryManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_pool_from_memory(memory)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        let mut mine = self.inner.lock().unwrap();
        mine.create_pool_from_buffer(buffer)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<MappedMemory> {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_memory_pool(mpid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory) {
        let mut mine = self.inner.lock().unwrap();
        mine.replace_memory_pool(mpid, memory)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          format: PixelFormat,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId> {
        let mut mine = self.inner.lock().unwrap();
        mine.create_memory_view(mpid, format, offset, width, height, stride)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_memory_view(&mut self, mpid: MemoryViewId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_memory_view(mpid);
    }
}

// -------------------------------------------------------------------------------------------------

impl HwGraphics for Coordinator {
    /// Sets graphics manager.
    ///
    /// During initialization or device discovery device manager should set this manager.
    ///
    /// FIXME: It is open point how to handle graphics manager in case of multi GPU setup.
    fn set_graphics_manager(&mut self, graphics_manager: Box<GraphicsManagement + Send>) {
        let mut mine = self.inner.lock().unwrap();
        mine.graphics_manager = Some(graphics_manager)
    }

    /// Checks if hardware acceleration support is available.
    fn has_hardware_acceleration_support(&self) -> bool {
        let mine = self.inner.lock().unwrap();
        mine.graphics_manager.is_some()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_egl_image(&mut self, attrs: EglAttributes) -> Option<EglImageId> {
        let mut mine = self.inner.lock().unwrap();
        mine.create_egl_image(attrs)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_egl_image(&mut self, ebid: EglImageId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_egl_image(ebid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn import_dmabuf(&mut self, attrs: DmabufAttributes) -> Option<DmabufId> {
        let mut mine = self.inner.lock().unwrap();
        mine.import_dmabuf(attrs)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_dmabuf(&mut self, dbid: DmabufId) {
        let mut mine = self.inner.lock().unwrap();
        mine.destroy_dmabuf(dbid)
    }
}

// -------------------------------------------------------------------------------------------------

impl Screenshooting for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn take_screenshot(&mut self, id: i32) {
        let mut mine = self.inner.lock().unwrap();
        mine.take_screenshot(id);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_screenshot_buffer(&mut self, buffer: Buffer) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_screenshot_buffer(buffer);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn take_screenshot_buffer(&mut self) -> Option<Buffer> {
        let mut mine = self.inner.lock().unwrap();
        mine.take_screenshot_buffer()
    }
}

// -------------------------------------------------------------------------------------------------

impl AestheticsCoordinationTrait for Coordinator {}
impl ExhibitorCoordinationTrait for Coordinator {}

// -------------------------------------------------------------------------------------------------
