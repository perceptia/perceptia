// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std::sync::{Arc, Mutex};

use dharma;

use qualia::{Position, Size, Vector, DmabufId, EglImageId, MemoryPoolId, MemoryViewId};
use qualia::{Buffer, MappedMemory, PixelFormat};
use qualia::{EglAttributes, DmabufAttributes, GraphicsManagement};
use qualia::{perceptron, Perceptron};
use qualia::{SurfaceContext, SurfaceId, SurfaceInfo, DataSource};
use qualia::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use qualia::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
use qualia::{AppearanceManagement, Emiter, MemoryManagement, HwGraphics, Screenshooting};
use qualia::{AestheticsCoordinationTrait, ExhibitorCoordinationTrait};
use qualia::{show_reason, surface_state};

use resource_storage::ResourceStorage;

// -------------------------------------------------------------------------------------------------

/// This structure contains logic related to maintaining shared application state other than
/// surfaces.
struct InnerCoordinator {
    /// Global signaler
    signaler: dharma::Signaler<Perceptron>,

    /// Screenshot buffer to be shared between threads.
    screenshot_buffer: Option<Buffer>,

    /// Currently keyboard-focused surface ID
    kfsid: SurfaceId,

    /// Currently pointer-focused surface ID
    pfsid: SurfaceId,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinator {
    /// Constructs new `InnerCoordinator`.
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        InnerCoordinator {
            signaler: signaler,
            screenshot_buffer: None,
            kfsid: SurfaceId::invalid(),
            pfsid: SurfaceId::invalid(),
        }
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

    /// Makes screenshot request.
    pub fn take_screenshot(&mut self, id: i32) {
        self.signaler.emit(perceptron::TAKE_SCREENSHOT, Perceptron::TakeScreenshot(id));
    }

    /// Sets given buffer as results of screenshot.
    pub fn set_screenshot_buffer(&mut self, buffer: Buffer) {
        self.screenshot_buffer = Some(buffer);
        self.signaler.emit(perceptron::SCREENSHOT_DONE, Perceptron::ScreenshotDone);
    }

    /// Returns and forgets screenshot buffer.
    pub fn take_screenshot_buffer(&mut self) -> Option<Buffer> {
        self.screenshot_buffer.take()
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure providing inter-thread access to `ResourceStorage` and `InnerCoordinator`.
#[derive(Clone)]
pub struct Coordinator {
    resources: Arc<Mutex<ResourceStorage>>,
    inner: Arc<Mutex<InnerCoordinator>>,
}

// -------------------------------------------------------------------------------------------------

impl Coordinator {
    /// `Coordinator` constructor.
    pub fn new(signaler: dharma::Signaler<Perceptron>) -> Self {
        Coordinator {
            resources: Arc::new(Mutex::new(ResourceStorage::new(signaler.clone()))),
            inner: Arc::new(Mutex::new(InnerCoordinator::new(signaler))),
        }
    }
}

// -------------------------------------------------------------------------------------------------

// TODO: Finish refactoring `Coordinator`: all method should be provided by some trait.
impl Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    pub fn get_data_source(&self, sid: SurfaceId) -> DataSource {
        let mine = self.resources.lock().unwrap();
        mine.get_data_source(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_surface(&mut self) -> SurfaceId {
        let mut mine = self.resources.lock().unwrap();
        mine.create_surface()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_shm(&self, mvid: MemoryViewId, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.attach_shm(mvid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_egl_image(&self, ebid: EglImageId, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.attach_egl_image(ebid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn attach_dmabuf(&self, dbid: DmabufId, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.attach_dmabuf(dbid, sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn detach_surface(&self, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.detach_surface(sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn commit_surface(&self, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.commit_surface(sid);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_surface(&self, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.destroy_surface(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceControl for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn show_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let mut mine = self.resources.lock().unwrap();
        mine.show_surface(sid, reason)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn hide_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {
        let mut mine = self.resources.lock().unwrap();
        mine.hide_surface(sid, reason)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_offset(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.resources.lock().unwrap();
        mine.set_surface_offset(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_requested_size(&self, sid: SurfaceId, size: Size) {
        let mut mine = self.resources.lock().unwrap();
        mine.set_surface_requested_size(sid, size)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {
        let mut mine = self.resources.lock().unwrap();
        mine.set_surface_relative_position(sid, offset)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.relate_surfaces(sid, parent_sid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn unrelate_surface(&self, sid: SurfaceId) {
        let mut mine = self.resources.lock().unwrap();
        mine.unrelate_surface(sid)
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceViewer for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let mine = self.resources.lock().unwrap();
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
        let mut mine = self.resources.lock().unwrap();
        mine.reconfigure(sid, size, state_flags);
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceListing for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mine = self.resources.lock().unwrap();
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
        let mut mine = self.resources.lock().unwrap();
        mine.create_pool_from_memory(memory)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        let mut mine = self.resources.lock().unwrap();
        mine.create_pool_from_buffer(buffer)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<MappedMemory> {
        let mut mine = self.resources.lock().unwrap();
        mine.destroy_memory_pool(mpid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory) {
        let mut mine = self.resources.lock().unwrap();
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
        let mut mine = self.resources.lock().unwrap();
        mine.create_memory_view(mpid, format, offset, width, height, stride)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_memory_view(&mut self, mpid: MemoryViewId) {
        let mut mine = self.resources.lock().unwrap();
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
        let mut mine = self.resources.lock().unwrap();
        mine.set_graphics_manager(Some(graphics_manager));
    }

    /// Checks if hardware acceleration support is available.
    fn has_hardware_acceleration_support(&self) -> bool {
        let mine = self.resources.lock().unwrap();
        mine.has_hardware_acceleration_support()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_egl_image(&mut self, attrs: EglAttributes) -> Option<EglImageId> {
        let mut mine = self.resources.lock().unwrap();
        mine.create_egl_image(attrs)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_egl_image(&mut self, ebid: EglImageId) {
        let mut mine = self.resources.lock().unwrap();
        mine.destroy_egl_image(ebid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn import_dmabuf(&mut self, attrs: DmabufAttributes) -> Option<DmabufId> {
        let mut mine = self.resources.lock().unwrap();
        mine.import_dmabuf(attrs)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_dmabuf(&mut self, dbid: DmabufId) {
        let mut mine = self.resources.lock().unwrap();
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
