// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains wanna-be-mock of `Coordinator`. Currently it is more a stub.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use defs::{DrmBundle, Position, SignalId, Size, Vector, WorkspaceState};
use defs::{DmabufId, EglImageId, MemoryPoolId, MemoryViewId};
use surface::{DataSource, SurfaceContext, SurfaceId, SurfaceInfo, surface_state, show_reason};
use surface::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use surface::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
use memory::{Buffer, Memory};
use image::PixelFormat;
use perceptron::Perceptron;
use traits::{StatePublishing, Screenshooting, MemoryManagement, WindowManagement};
use traits::ExhibitorCoordinationTrait;

// -------------------------------------------------------------------------------------------------

/// Mock of `Coordinator`.
pub struct InnerCoordinatorMock {
    surfaces: HashMap<SurfaceId, SurfaceInfo>,
}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinatorMock {
    pub fn new() -> Self {
        InnerCoordinatorMock { surfaces: HashMap::new() }
    }

    pub fn add_surface(&mut self, sid: SurfaceId) {
        let info = SurfaceInfo {
            id: sid,
            offset: Vector::default(),
            parent_sid: SurfaceId::invalid(),
            desired_size: Size::default(),
            requested_size: Size::default(),
            state_flags: surface_state::REGULAR,
            data_source: DataSource::None,
        };

        self.surfaces.insert(sid, info);
    }
}

// -------------------------------------------------------------------------------------------------

/// Mock of `Coordinator`.
#[derive(Clone)]
pub struct CoordinatorMock {
    mock: Rc<RefCell<InnerCoordinatorMock>>,
}

// -------------------------------------------------------------------------------------------------

impl CoordinatorMock {
    pub fn new() -> Self {
        CoordinatorMock { mock: Rc::new(RefCell::new(InnerCoordinatorMock::new())) }
    }

    pub fn add_surface(&mut self, sid: SurfaceId) {
        let mut mock = self.mock.borrow_mut();
        mock.add_surface(sid);
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceManagement for CoordinatorMock {
    fn create_surface(&mut self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn attach_shm(&self, _mvid: MemoryViewId, _sid: SurfaceId) {}
    fn attach_egl_image(&self, _eiid: EglImageId, _sid: SurfaceId) {}
    fn attach_dmabuf(&self, _dbid: DmabufId, _sid: SurfaceId) {}
    fn detach_surface(&self, _sid: SurfaceId) {}
    fn commit_surface(&self, _sid: SurfaceId) {}
    fn destroy_surface(&self, _sid: SurfaceId) {}
}

// -------------------------------------------------------------------------------------------------

impl SurfaceControl for CoordinatorMock {
    fn show_surface(&self, _sid: SurfaceId, _reason: show_reason::ShowReason) {}
    fn dock_surface(&self, _sid: SurfaceId, _size: Size, _display_id: i32) {}
    fn hide_surface(&self, _sid: SurfaceId, _reason: show_reason::ShowReason) {}
    fn set_surface_offset(&self, _sid: SurfaceId, _offset: Vector) {}
    fn set_surface_requested_size(&self, _sid: SurfaceId, _size: Size) {}
    fn set_surface_relative_position(&self, _sid: SurfaceId, _offset: Vector) {}
    fn relate_surfaces(&self, _sid: SurfaceId, _parent_sid: SurfaceId) {}
    fn unrelate_surface(&self, _sid: SurfaceId) {}
}

// -------------------------------------------------------------------------------------------------

impl SurfaceViewer for CoordinatorMock {
    // TODO: This method will always return some `SurfaceInfo`. It should panic if not existing
    // surface is requested.
    fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
        let mock = self.mock.borrow();
        if let Some(info) = mock.surfaces.get(&sid) {
            Some(info.clone())
        } else {
            Some(SurfaceInfo {
                     id: sid,
                     offset: Vector::default(),
                     parent_sid: SurfaceId::invalid(),
                     desired_size: Size::default(),
                     requested_size: Size::default(),
                     state_flags: surface_state::REGULAR,
                     data_source: DataSource::None,
                 })
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceAccess for CoordinatorMock {
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState) {
        let mut mock = self.mock.borrow_mut();
        if let Some(mut info) = mock.surfaces.get_mut(&sid) {
            info.desired_size = size;
            info.requested_size = size;
            info.state_flags = state_flags;
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceListing for CoordinatorMock {
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        let mock = self.mock.borrow();
        if let Some(info) = mock.surfaces.get(&sid) {
            Some(vec![SurfaceContext::new(sid, info.offset)])
        } else {
            panic!("Trying to get renderer context for not existing surface {:?}", sid);
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SurfaceFocusing for CoordinatorMock {
    fn get_keyboard_focused_sid(&self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn set_keyboard_focus(&mut self, _sid: SurfaceId) {}
    fn get_pointer_focused_sid(&self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn set_pointer_focus(&mut self, _sid: SurfaceId, _position: Position) {}
}

// -------------------------------------------------------------------------------------------------

impl StatePublishing for CoordinatorMock {
    fn emit(&mut self, _id: SignalId, _package: Perceptron) {}
    fn suspend(&mut self) {}
    fn wakeup(&mut self) {}
    fn input_devices_changed(&mut self) {}
    fn output_devices_changed(&mut self) {}
    fn notify(&mut self) {}
    fn publish_output(&mut self, _drm_bundle: DrmBundle) {}
    fn emit_vblank(&mut self, _display_id: i32) {}
    fn emit_page_flip(&mut self, _display_id: i32) {}
}

// -------------------------------------------------------------------------------------------------

impl MemoryManagement for CoordinatorMock {
    fn create_memory_pool(&mut self, _memory: Memory) -> MemoryPoolId {
        MemoryPoolId::initial()
    }
    fn destroy_memory_pool(&mut self, _mpid: MemoryPoolId) -> Option<Memory> {
        None
    }
    fn replace_memory_pool(&mut self, _mpid: MemoryPoolId, _memory: Memory) {}
    fn create_memory_view(&mut self,
                          _mpid: MemoryPoolId,
                          _format: PixelFormat,
                          _offset: usize,
                          _width: usize,
                          _height: usize,
                          _stride: usize)
                          -> Option<MemoryViewId> {
        None
    }
    fn destroy_memory_view(&mut self, _mpid: MemoryViewId) {}
}

// -------------------------------------------------------------------------------------------------

impl WindowManagement for CoordinatorMock {
    fn set_workspace_state(&mut self, _state: WorkspaceState) {}
    fn get_workspace_state(&self) -> WorkspaceState {
        WorkspaceState::empty()
    }
}

// -------------------------------------------------------------------------------------------------

impl Screenshooting for CoordinatorMock {
    fn take_screenshot(&mut self, _id: i32) {}
    fn set_screenshot_buffer(&mut self, _buffer: Buffer) {}
    fn take_screenshot_buffer(&mut self) -> Option<Buffer> {
        None
    }
}

// -------------------------------------------------------------------------------------------------

impl ExhibitorCoordinationTrait for CoordinatorMock {}

// -------------------------------------------------------------------------------------------------
