// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains logic related to maintaining shared application state about surfaces.
//! Every update for application state should be done via call to one on `Coordinator`s methods
//! which update the state and signal an event if needed.

// -------------------------------------------------------------------------------------------------

use std::os::unix::io::RawFd;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dharma;

use cognitive_graphics::attributes::{EglAttributes, DmabufAttributes};
use qualia::{Position, Size, Vector, DmabufId, EglImageId, MemoryPoolId, MemoryViewId};
use qualia::{Buffer, Memory, PixelFormat, GraphicsManagement, WorkspaceState};
use qualia::{perceptron, Perceptron, Transfer, DrmBundle};
use qualia::{SurfaceContext, SurfaceId, SurfaceInfo, DataSource};
use qualia::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use qualia::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
use qualia::{AppearanceManagement, DataTransferring, EventHandling, StatePublishing};
use qualia::{MemoryManagement, HwGraphics, WindowManagement, Screenshooting};
use qualia::{AestheticsCoordinationTrait, ExhibitorCoordinationTrait};
use qualia::FrontendsCoordinationTrait;
use qualia::{show_reason, surface_state};

use resource_storage::ResourceStorage;

// -------------------------------------------------------------------------------------------------

/// This structure contains logic related to maintaining shared application state other than
/// surfaces.
struct InnerCoordinator {
    /// Global signaler.
    signaler: dharma::Signaler<Perceptron>,

    /// `Dispatcher` controller.
    dispatcher: dharma::DispatcherController,

    /// Screenshot buffer to be shared between threads.
    screenshot_buffer: Option<Buffer>,

    /// Currently keyboard-focused surface ID
    kfsid: SurfaceId,

    /// Currently pointer-focused surface ID
    pfsid: SurfaceId,

    /// Represents possible data transfer between clients (e.g. copy-paste)
    transfer: Option<Transfer>,

    /// State of workspaces.
    workspace_state: WorkspaceState,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinator {
    /// Constructs new `InnerCoordinator`.
    pub fn new(signaler: dharma::Signaler<Perceptron>,
               dispatcher: dharma::DispatcherController)
               -> Self {
        InnerCoordinator {
            signaler: signaler,
            dispatcher: dispatcher,
            screenshot_buffer: None,
            kfsid: SurfaceId::invalid(),
            pfsid: SurfaceId::invalid(),
            transfer: None,
            workspace_state: WorkspaceState::empty(),
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

    /// Notifies about suspending drawing on screen. Probably virtual terminal was switched and GPU
    /// is not available to us.
    pub fn suspend(&mut self) {
        self.signaler.emit(perceptron::SUSPEND, Perceptron::Suspend);
    }

    /// Send request to revoke application from suspension.
    pub fn wakeup(&mut self) {
        self.signaler.emit(perceptron::WAKEUP, Perceptron::WakeUp);
    }

    /// Sends notification about changing of state of input devices.
    pub fn input_devices_changed(&mut self) {
        self.signaler.emit(perceptron::INPUTS_CHANGED, Perceptron::InputsChanged);
    }

    /// Sends notification about changing of state of output devices.
    pub fn output_devices_changed(&mut self) {
        self.signaler.emit(perceptron::OUTPUTS_CHANGED, Perceptron::OutputsChanged);
    }

    /// Notifies application about event that requires screen to be refreshed.
    pub fn notify(&mut self) {
        self.signaler.emit(perceptron::NOTIFY, Perceptron::Notify);
    }

    /// Publishes newly found output.
    fn publish_output(&mut self, drm_bundle: DrmBundle) {
        self.signaler.emit(perceptron::OUTPUT_FOUND, Perceptron::OutputFound(drm_bundle));
    }

    /// Notifies about V-blank.
    fn emit_vblank(&mut self, display_id: i32) {
        self.signaler.emit(perceptron::VERTICAL_BLANK, Perceptron::VerticalBlank(display_id));
    }

    /// Notifies about page flip.
    fn emit_page_flip(&mut self, display_id: i32) {
        self.signaler.emit(perceptron::PAGE_FLIP, Perceptron::PageFlip(display_id));
    }

    /// Sets data transfer information.
    pub fn set_transfer(&mut self, transfer: Option<Transfer>) {
        // TODO: Only currently focussed client should be able to set transfer.
        self.transfer = transfer;
        self.signaler.emit(perceptron::TRANSFER_OFFERED, Perceptron::TransferOffered);
    }

    /// Returns data transfer information.
    pub fn get_transfer(&self) -> Option<Transfer> {
        self.transfer.clone()
    }

    /// Requests begin of data transfer to requesting client.
    pub fn request_transfer(&mut self, mime_type: String, fd: RawFd) {
        self.signaler.emit(perceptron::TRANSFER_REQUESTED,
                           Perceptron::TransferRequested(mime_type, fd));
    }

    /// Adds new event handler.
    pub fn add_event_handler(&mut self,
                             event_handler: Box<dharma::EventHandler + Send>,
                             event_kind: dharma::EventKind)
                             -> dharma::EventHandlerId {
        self.dispatcher.add_source(event_handler, event_kind)
    }

    /// Adds new event handler.
    pub fn remove_event_handler(&mut self, id: dharma::EventHandlerId) {
        self.dispatcher.delete_source(id);
    }

    /// Stores new workspace state and informs other parts of application it has changed.
    fn set_workspace_state(&mut self, state: WorkspaceState) {
        self.workspace_state = state;

        // NOTE: To avoid having to many signals this one should be the only one related to
        // workspace state. If more fine informations is needed it should be provided inside
        // `Perceptron::WorkspaceStateChanged`.
        self.signaler.emit(perceptron::WORKSPACE_STATE_CHANGED,
                           Perceptron::WorkspaceStateChanged {});
    }

    /// Returns current workspace state.
    fn get_workspace_state(&self) -> WorkspaceState {
        self.workspace_state.clone()
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
    pub fn new(signaler: dharma::Signaler<Perceptron>,
               dispatcher: dharma::DispatcherController)
               -> Self {
        let mut mine = Coordinator {
            resources: Arc::new(Mutex::new(ResourceStorage::new(signaler.clone()))),
            inner: Arc::new(Mutex::new(InnerCoordinator::new(signaler.clone(),
                                                             dispatcher.clone()))),
        };

        mine.setup(signaler, dispatcher);
        mine
    }

    /// Sets up coordinations related stuff.
    ///
    /// - adds system signal event handler
    /// - adds 500 millisecond timer
    fn setup(&mut self,
             signaler: dharma::Signaler<Perceptron>,
             mut dispatcher: dharma::DispatcherController) {
        // Set up signal handler
        let signal_source = Box::new(dharma::SignalEventHandler::new(dispatcher.clone(),
                                                                     signaler.clone()));
        dispatcher.add_source(signal_source, dharma::event_kind::READ);

        // Set up 500 milliseconds timer.
        let mut timer_signaler = signaler.clone();
        let timer = dharma::Timer::new(Duration::new(0, 500_000_000), move || {
            timer_signaler.emit(perceptron::TIMER_500, Perceptron::Timer500);
        }).expect("creating 500 millisecond timer");
        dispatcher.add_source(Box::new(timer), dharma::event_kind::READ);
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
    fn dock_surface(&self, sid: SurfaceId, size: Size, display_id: i32) {
        let mut mine = self.resources.lock().unwrap();
        mine.dock_surface(sid, size, display_id)
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

impl DataTransferring for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_transfer(&mut self, transfer: Option<Transfer>) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_transfer(transfer);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_transfer(&self) -> Option<Transfer> {
        let mine = self.inner.lock().unwrap();
        mine.get_transfer()
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn request_transfer(&mut self, mime_type: String, fd: RawFd) {
        let mut mine = self.inner.lock().unwrap();
        mine.request_transfer(mime_type, fd);
    }
}

// -------------------------------------------------------------------------------------------------

impl EventHandling for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn add_event_handler(&mut self,
                         event_handler: Box<dharma::EventHandler + Send>,
                         event_kind: dharma::EventKind)
                         -> dharma::EventHandlerId {
        let mut mine = self.inner.lock().unwrap();
        mine.add_event_handler(event_handler, event_kind)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn remove_event_handler(&mut self, id: dharma::EventHandlerId) {
        let mut mine = self.inner.lock().unwrap();
        mine.remove_event_handler(id);
    }
}

// -------------------------------------------------------------------------------------------------

impl StatePublishing for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn emit(&mut self, id: dharma::SignalId, package: Perceptron) {
        let mut mine = self.inner.lock().unwrap();
        mine.emit(id, package);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn suspend(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.suspend();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn wakeup(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.wakeup();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn input_devices_changed(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.input_devices_changed();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn output_devices_changed(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.output_devices_changed();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn notify(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.notify();
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn publish_output(&mut self, drm_budle: DrmBundle) {
        let mut mine = self.inner.lock().unwrap();
        mine.publish_output(drm_budle);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn emit_vblank(&mut self, display_id: i32) {
        let mut mine = self.inner.lock().unwrap();
        mine.emit_vblank(display_id);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn emit_page_flip(&mut self, display_id: i32) {
        let mut mine = self.inner.lock().unwrap();
        mine.emit_page_flip(display_id);
    }
}

// -------------------------------------------------------------------------------------------------

impl MemoryManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn create_memory_pool(&mut self, memory: Memory) -> MemoryPoolId {
        let mut mine = self.resources.lock().unwrap();
        mine.create_memory_pool(memory)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<Memory> {
        let mut mine = self.resources.lock().unwrap();
        mine.destroy_memory_pool(mpid)
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: Memory) {
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

impl WindowManagement for Coordinator {
    /// Lock and call corresponding method from `InnerCoordinator`.
    fn set_workspace_state(&mut self, state: WorkspaceState) {
        let mut mine = self.inner.lock().unwrap();
        mine.set_workspace_state(state);
    }

    /// Lock and call corresponding method from `InnerCoordinator`.
    fn get_workspace_state(&self) -> WorkspaceState {
        let mine = self.inner.lock().unwrap();
        mine.get_workspace_state()
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
impl FrontendsCoordinationTrait for Coordinator {}

// -------------------------------------------------------------------------------------------------
