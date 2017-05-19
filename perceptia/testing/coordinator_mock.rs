// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains wanna-be-mock of `Coordinator`. Currently it is more a stub.

use std::cell::RefCell;
use std::rc::Rc;

use qualia::{Perceptron, PixelFormat, Position, SignalId, Size, Vector};
use qualia::{SurfaceContext, SurfaceId, SurfaceInfo, surface_state, show_reason};
use qualia::{Buffer, DataSource, DmabufId, EglImageId, MappedMemory, MemoryPoolId, MemoryViewId};
use qualia::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use qualia::{SurfaceAccess, SurfaceListing, SurfaceFocusing};
use qualia::{Emiter, Screenshooting, MemoryManagement, ExhibitorCoordinationTrait};

// -------------------------------------------------------------------------------------------------

/// Mock of `Coordinator`.
pub struct InnerCoordinatorMock {}

// -------------------------------------------------------------------------------------------------

impl InnerCoordinatorMock {
    pub fn new() -> Self {
        InnerCoordinatorMock {}
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
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceManagement for CoordinatorMock {
    fn create_surface(&mut self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn attach_shm(&self, mvid: MemoryViewId, sid: SurfaceId) {}
    fn attach_egl_image(&self, eiid: EglImageId, sid: SurfaceId) {}
    fn attach_dmabuf(&self, dbid: DmabufId, sid: SurfaceId) {}
    fn detach_surface(&self, sid: SurfaceId) {}
    fn commit_surface(&self, sid: SurfaceId) {}
    fn destroy_surface(&self, sid: SurfaceId) {}
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceControl for CoordinatorMock {
    fn show_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {}
    fn dock_surface(&self, sid: SurfaceId, size: Size, display_id: i32) {}
    fn hide_surface(&self, sid: SurfaceId, reason: show_reason::ShowReason) {}
    fn set_surface_offset(&self, sid: SurfaceId, offset: Vector) {}
    fn set_surface_requested_size(&self, sid: SurfaceId, size: Size) {}
    fn set_surface_relative_position(&self, sid: SurfaceId, offset: Vector) {}
    fn relate_surfaces(&self, sid: SurfaceId, parent_sid: SurfaceId) {}
    fn unrelate_surface(&self, sid: SurfaceId) {}
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceViewer for CoordinatorMock {
    fn get_surface(&self, sid: SurfaceId) -> Option<SurfaceInfo> {
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

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceAccess for CoordinatorMock {
    fn reconfigure(&mut self,
                   sid: SurfaceId,
                   size: Size,
                   state_flags: surface_state::SurfaceState) {
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceListing for CoordinatorMock {
    fn get_renderer_context(&self, sid: SurfaceId) -> Option<Vec<SurfaceContext>> {
        None
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl SurfaceFocusing for CoordinatorMock {
    fn get_keyboard_focused_sid(&self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn set_keyboard_focus(&mut self, sid: SurfaceId) {}
    fn get_pointer_focused_sid(&self) -> SurfaceId {
        SurfaceId::new(0)
    }
    fn set_pointer_focus(&mut self, sid: SurfaceId, position: Position) {}
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl Emiter for CoordinatorMock {
    fn emit(&mut self, id: SignalId, package: Perceptron) {}
    fn notify(&mut self) {}
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl MemoryManagement for CoordinatorMock {
    fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId {
        MemoryPoolId::initial()
    }
    fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId {
        MemoryPoolId::initial()
    }
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<MappedMemory> {
        None
    }
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory) {}
    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          format: PixelFormat,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId> {
        None
    }
    fn destroy_memory_view(&mut self, mpid: MemoryViewId) {}
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl Screenshooting for CoordinatorMock {
    fn take_screenshot(&mut self, id: i32) {}
    fn set_screenshot_buffer(&mut self, buffer: Buffer) {}
    fn take_screenshot_buffer(&mut self) -> Option<Buffer> {
        None
    }
}

// -------------------------------------------------------------------------------------------------

impl ExhibitorCoordinationTrait for CoordinatorMock {}

// -------------------------------------------------------------------------------------------------
