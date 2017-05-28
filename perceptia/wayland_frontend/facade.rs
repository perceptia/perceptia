// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides interface from client request handlers for the rest of frontend.

// -------------------------------------------------------------------------------------------------

use std::path::PathBuf;
use std::os::unix::io::RawFd;

use skylane::server as wl;

use cognitive_graphics::attributes::{EglAttributes, DmabufAttributes};
use qualia::{Area, MappedMemory, PixelFormat, Size, SurfaceId, Transfer, Vector, show_reason};
use qualia::{DmabufId, EglImageId, MemoryPoolId, MemoryViewId};

// -------------------------------------------------------------------------------------------------

/// Enum describing type of shell and related object IDs.
#[derive(Clone, Copy, Debug)]
pub enum ShellSurfaceOid {
    Shell(wl::ObjectId),
    ZxdgToplevelV6(wl::ObjectId, wl::ObjectId),
}

// -------------------------------------------------------------------------------------------------

/// Data related to positioner object.
#[derive(Clone, Copy)]
pub struct PositionerInfo {
    pub offset: Vector,
    pub size: Size,
    pub anchor: Area,
}

// -------------------------------------------------------------------------------------------------

impl PositionerInfo {
    pub fn new() -> Self {
        PositionerInfo {
            offset: Vector::default(),
            size: Size::default(),
            anchor: Area::default(),
        }
    }

    pub fn get_area(&self) -> Area {
        Area::new(self.offset + self.anchor.pos, self.size)
    }
}

// -------------------------------------------------------------------------------------------------

pub trait Facade {
    /// Requests creation of memory pool. Return ID of newly created pool.
    fn create_memory_pool(&mut self, memory: MappedMemory) -> MemoryPoolId;

    /// Requests destruction of memory pool. The pool will be destroyed by application after the
    /// last view goes out of the scope.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId);

    /// Requests replacement of mapped memory after resize request from client.
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory);

    /// Requests creation of memory view.
    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          buffer_oid: wl::ObjectId,
                          format: PixelFormat,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId>;

    /// Requests destruction of memory view.
    fn destroy_memory_view(&mut self, mvid: MemoryViewId);

    /// Requests creation of EGL image.
    fn create_egl_image(&mut self,
                        buffer_oid: wl::ObjectId,
                        attrs: EglAttributes)
                        -> Option<EglImageId>;

    /// Requests destruction of EGL image.
    fn destroy_egl_image(&mut self, eiid: EglImageId);

    /// Requests creation of dmabuf.
    fn import_dmabuf(&mut self,
                     buffer_oid: wl::ObjectId,
                     attrs: DmabufAttributes)
                     -> Option<DmabufId>;

    /// Requests destruction of dmabuf.
    fn destroy_dmabuf(&mut self, dbid: DmabufId);

    /// Defines region. Regions may be used to define input area of surface.
    fn define_region(&mut self, region_oid: wl::ObjectId, region: Area);

    /// Undefines region.
    fn undefine_region(&mut self, region_oid: wl::ObjectId);

    /// Adds pointer OID.
    fn add_pointer_oid(&mut self, pointer_oid: wl::ObjectId);

    /// Removes pointer OID.
    fn remove_pointer_oid(&mut self, pointer_oid: wl::ObjectId);

    /// Adds keyboard OID.
    fn add_keyboard_oid(&mut self, keyboard_oid: wl::ObjectId);

    /// Removes keyboard OID.
    fn remove_keyboard_oid(&mut self, keyboard_oid: wl::ObjectId);

    /// Add data device OID.
    fn add_data_device_oid(&mut self, data_device_oid: wl::ObjectId);

    /// Removes data device OID.
    fn remove_data_device_oid(&mut self, data_device_oid: wl::ObjectId);

    /// Sets positioner info.
    fn set_positioner(&mut self, wl::ObjectId, positioner: PositionerInfo);

    /// Gets positioner info.
    fn get_positioner(&mut self, oid: wl::ObjectId) -> Option<PositionerInfo>;

    /// Removes positioner info.
    fn remove_positioner(&mut self, oid: wl::ObjectId);

    /// Sets transfer info.
    fn set_transfer(&mut self, wl::ObjectId, transfer: Transfer);

    /// Gets transfer info.
    fn get_transfer(&mut self, oid: wl::ObjectId) -> Option<Transfer>;

    /// Selects given transfer info as the offered one.
    fn select_transfer(&mut self, oid: wl::ObjectId);

    /// Removes transfer info.
    fn remove_transfer(&mut self, oid: wl::ObjectId);

    /// Request start of data transfer to requesting client.
    fn request_transfer(&mut self, mime_type: String, fd: RawFd);

    /// Sets given region as input region of surface.
    fn set_input_region(&self, sid: SurfaceId, region_oid: wl::ObjectId);

    /// Requests creation of surface. Return ID of newly created surface.
    fn create_surface(&mut self, surface_oid: wl::ObjectId) -> SurfaceId;

    /// Requests destruction of surface.
    fn destroy_surface(&self, sid: SurfaceId);

    /// Attaches memory view to surface. This will take effect after `commit`.
    fn attach(&mut self, buffer_oid: wl::ObjectId, sid: SurfaceId, x: i32, y: i32);

    /// Commits all requests to surface.
    fn commit(&self, sid: SurfaceId);

    /// Requests (one-shot) notification about redrawing of given surface.
    fn set_frame(&mut self, sid: SurfaceId, frame_oid: wl::ObjectId);

    /// Adds a reason to show given surface on screen.
    fn show(&mut self,
            surface_oid: wl::ObjectId,
            shell_surface_oid: ShellSurfaceOid,
            reason: show_reason::ShowReason);

    /// Removes a reason to show given surface on screen.
    fn hide(&mut self, surface_oid: wl::ObjectId, reason: show_reason::ShowReason);

    /// Defines offset between origin of buffer and real area of surface. Client for example may
    /// want to draw shadow, which should not be threated by compositor as internal part of
    /// surface.
    fn set_offset(&self, sid: SurfaceId, offset: Vector);

    /// Request setting size of surface.
    fn set_requested_size(&self, sid: SurfaceId, size: Size);

    /// Requests setting relation (child-parent) between two surfaces.
    fn relate(&self, surface_oid: wl::ObjectId, parent_surface_oid: wl::ObjectId);

    /// Requests cancellation of relation between given surface and its parent.
    fn unrelate(&self, surface_oid: wl::ObjectId);

    /// Requests to set offset between related surfaces.
    fn set_relative_position(&self, surface_oid: wl::ObjectId, x: isize, y: isize);

    /// Requests to use given surface for drawing cursor.
    fn set_as_cursor(&self, surface_oid: wl::ObjectId, hotspot_x: isize, hotspot_x: isize);

    /// Relates output object ID with output ID.
    fn relate_output_oid_with_id(&mut self, oid: wl::ObjectId, id: i32);

    /// Requests taking screenshot.
    fn take_screenshot(&mut self,
                       screenshoter_oid: wl::ObjectId,
                       output_oid: wl::ObjectId,
                       output_oid: wl::ObjectId);

    /// Authenticates DRM device.
    fn authenticate_drm_device(&mut self, magic: u32);

    /// Returns path of current DRM device.
    fn get_drm_device_path(&self) -> Option<PathBuf>;
}

// -------------------------------------------------------------------------------------------------
