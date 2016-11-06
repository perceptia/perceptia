// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides interface from client request handlers for the rest of frontend.

// -------------------------------------------------------------------------------------------------

use skylane as wl;

use qualia::{MappedMemory, Size, SurfaceId, Vector, MemoryPoolId, MemoryViewId, show_reason};

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum ShellSurfaceOid {
    Shell(wl::common::ObjectId),
    ZxdgToplevelV6(wl::common::ObjectId, wl::common::ObjectId),
}

// -------------------------------------------------------------------------------------------------

pub trait Facade {
    /// Requests creation of memory pool. Return ID of newly created pool.
    fn create_memory_pool(&mut self, memory: MappedMemory) -> MemoryPoolId;

    /// Requests destruction of memory pool. The pool will be destroyed by application after the
    /// last view goes out of the scope.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId);

    /// Requests creation of memory view.
    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          buffer_oid: wl::common::ObjectId,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId>;

    /// Requests creation of surface. Return ID of newly created surface.
    fn create_surface(&mut self, surface_oid: wl::common::ObjectId) -> SurfaceId;

    /// Requests destruction of surface.
    fn destroy_surface(&self, sid: SurfaceId);

    /// Attaches memory view to surface. This will take effect after `commit`.
    fn attach(&mut self, buffer_oid: wl::common::ObjectId, sid: SurfaceId, x: i32, y: i32);

    /// Commits all requests to surface.
    fn commit(&self, sid: SurfaceId);

    /// Requests (one-shot) notification about redrawing of given surface.
    fn set_frame(&mut self, sid: SurfaceId, frame_oid: wl::common::ObjectId);

    /// Presents a reason to show given surface on screen.
    fn show(&mut self,
            surface_oid: wl::common::ObjectId,
            shell_surface_oid: ShellSurfaceOid,
            reason: show_reason::ShowReason);

    /// Defines offset between origin of buffer and real area of surface. Client for example may
    /// want to draw shadow, which should not be threated by compositor as internal part of
    /// surface.
    fn set_offset(&self, sid: SurfaceId, offset: Vector);

    /// Request setting size of surface.
    fn set_requested_size(&self, sid: SurfaceId, size: Size);

    /// Requests setting relation (child-parent) between two surfaces.
    fn relate(&self, sid: SurfaceId, parent_sid: SurfaceId);

    /// Requests to set offset between related surfaces.
    fn set_relative_position(&self, sid: SurfaceId, offset: Vector);

    /// Requests to use given surface for drawing cursor.
    fn set_as_cursor(&self, sid: SurfaceId);
}

// -------------------------------------------------------------------------------------------------
