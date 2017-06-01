// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces and
//! their data sources.

// -------------------------------------------------------------------------------------------------

use std;

use dharma;

use cognitive_graphics::attributes::{EglAttributes, DmabufAttributes};
use qualia::{Position, Size, Vector, DmabufId, EglImageId, MemoryPoolId, MemoryViewId};
use qualia::{Memory, MemoryPool, MemoryView, PixelFormat, GraphicsManagement};
use qualia::{perceptron, Perceptron};
use qualia::{SurfaceContext, SurfaceId, SurfaceInfo, DataSource};
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
                log_warn3!("Surface {} not found!", $sid);
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

/// This structure contains logic related to maintaining shared application state about surfaces and
/// their data sources.
pub struct ResourceStorage {
    /// Global signaler
    signaler: dharma::Signaler<Perceptron>,

    /// Graphics manager.
    graphics_manager: Option<Box<GraphicsManagement + Send>>,

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
}

// -------------------------------------------------------------------------------------------------

impl ResourceStorage {
    /// Constructs new `ResourceStorage`.
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        ResourceStorage {
            signaler: signaler,
            graphics_manager: None,
            surfaces: SurfaceMap::new(),
            memory_views: MemoryViewMap::new(),
            memory_pools: MemoryPoolMap::new(),
            egl_images: EglImagesMap::new(),
            dmabufs: DmabufsMap::new(),
            last_surface_id: SurfaceId::invalid(),
            last_memory_view_id: MemoryViewId::initial(),
            last_memory_pool_id: MemoryPoolId::initial(),
            last_egl_image_id: EglImageId::initial(),
            last_dmabuf_id: DmabufId::initial(),
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
        let is_first_time_commited = {
            let surface = try_get_surface!(self, sid);
            surface.commit()
        };
        if is_first_time_commited {
            self.show_surface(sid, show_reason::DRAWABLE);
        }
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
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

    /// Requests showing given surface as dock with given size on given display.
    pub fn dock_surface(&mut self, sid: SurfaceId, size: Size, display_id: i32) {
        self.signaler.emit(perceptron::DOCK_SURFACE,
                           Perceptron::DockSurface(sid, size, display_id));
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
        }
        {
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
}

// -------------------------------------------------------------------------------------------------

impl ResourceStorage {
    /// Creates new memory pool. Returns ID of newly created pool.
    pub fn create_memory_pool(&mut self, memory: Memory) -> MemoryPoolId {
        let mpid = self.generate_next_memory_pool_id();
        let bundle = MemoryPoolBundle::new(MemoryPool::new(memory));
        self.memory_pools.insert(mpid, bundle);
        mpid
    }

    /// Schedules destruction of memory pool identified by given ID. The pool will be destructed
    /// when all its views go out of the scope.
    ///
    /// If the poll was created from mapped memory, returns this memory.
    pub fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<Memory> {
        match self.memory_pools.remove(&mpid) {
            Some(memory_pool) => {
                // Remove all related views
                for mvid in memory_pool.views.iter() {
                    self.memory_views.remove(mvid);
                }

                // Remove the pool
                memory_pool.pool.take_memory()
            }
            None => None,
        }
    }

    /// Replaces mapped memory with other memory reusing its ID. This method may be used when
    /// client requests memory map resize.
    pub fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: Memory) {
        self.memory_pools.remove(&mpid);
        let bundle = MemoryPoolBundle::new(MemoryPool::new(memory));
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
}

// -------------------------------------------------------------------------------------------------

impl ResourceStorage {
    /// Set the graphics manager.
    pub fn set_graphics_manager(&mut self,
                                graphics_manager: Option<Box<GraphicsManagement + Send>>) {
        self.graphics_manager = graphics_manager;
    }

    /// Checks if hardware acceleration support is available.
    pub fn has_hardware_acceleration_support(&self) -> bool {
        self.graphics_manager.is_some()
    }

    /// Checks if it is possible to create EGL image with given attributes. If so, then stores
    /// attributes and returns ID assigned to them.
    pub fn create_egl_image(&mut self, attrs: EglAttributes) -> Option<EglImageId> {
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

    /// Removes EGL attributes.
    pub fn destroy_egl_image(&mut self, ebid: EglImageId) {
        self.egl_images.remove(&ebid);
    }

    /// Checks if it is possible to import dmabuf with given attributes. If so, then stores
    /// attributes and returns ID assigned to them.
    pub fn import_dmabuf(&mut self, attrs: DmabufAttributes) -> Option<DmabufId> {
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

    /// Removes dmabuf attributes.
    pub fn destroy_dmabuf(&mut self, dbid: DmabufId) {
        self.dmabufs.remove(&dbid);
    }
}

// -------------------------------------------------------------------------------------------------

// Helper functions
impl ResourceStorage {
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

#[cfg(test)]
mod tests {
    use qualia;

    fn assert_storage_size(resources: &super::ResourceStorage) {
        assert!(resources.surfaces.len() == 0);
        assert!(resources.memory_views.len() == 0);
        assert!(resources.memory_pools.len() == 0);
        assert!(resources.egl_images.len() == 0);
        assert!(resources.dmabufs.len() == 0);
    }

    /// Check if newly created `ResourceStorage` is empty.
    #[test]
    fn test_empty_resources() {
        let signaler = super::dharma::Signaler::new();
        let resources = super::ResourceStorage::new(signaler);
        assert_storage_size(&resources);
    }

    /// Check if pool and view are removed when removed when destroyed explicitly and when only
    /// pool is destroyed.
    #[test]
    fn test_leaks_after_creating_pool_and_view() {
        let signaler = super::dharma::Signaler::new();
        let mut resources = super::ResourceStorage::new(signaler);

        let width = 10;
        let height = 10;
        let format = super::PixelFormat::XRGB8888;
        let stride = format.get_size() * width;

        let data1 = vec![0; stride * height];
        let mut buffer1 = qualia::Buffer::new(format, width, height, stride, data1);
        let data2 = vec![0; stride * height];
        let mut buffer2 = qualia::Buffer::new(format, width, height, stride, data2);

        let mpid1 = resources.create_memory_pool(unsafe { buffer1.as_memory() });
        let mvid1 = resources.create_memory_view(mpid1, format, 0, width, height, stride).unwrap();

        let mpid2 = resources.create_memory_pool(unsafe { buffer2.as_memory() });
        resources.create_memory_view(mpid2, format, 0, width, height, stride).unwrap();

        resources.destroy_memory_view(mvid1);
        resources.destroy_memory_pool(mpid1);
        resources.destroy_memory_pool(mpid2);

        assert_storage_size(&resources);
    }
}

// -------------------------------------------------------------------------------------------------
