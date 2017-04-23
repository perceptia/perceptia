// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Traits implementing interfaces to `Coordinator` functionality used to decouple crates using
//! `Coordinator` from its implementation (mainly useful for mocking in unit test).

use defs::{MemoryPoolId, MemoryViewId, SignalId};
use memory::{Buffer, MappedMemory};
use perceptron::Perceptron;
use surface::{SurfaceManagement, SurfaceControl, SurfaceViewer};
use surface::{SurfaceAccess, SurfaceListing, SurfaceFocusing};

// -------------------------------------------------------------------------------------------------

/// Generic communication with the rest of application.
pub trait Emiter {
    /// Emits given signal.
    fn emit(&mut self, id: SignalId, package: Perceptron);

    /// Notifies application about event that requires screen to be refreshed.
    fn notify(&mut self);
}

// -------------------------------------------------------------------------------------------------

/// Managing memory pools and views.
pub trait MemoryManagement {
    /// Creates new memory pool from mapped memory. Returns ID of newly created pool.
    fn create_pool_from_memory(&mut self, memory: MappedMemory) -> MemoryPoolId;

    /// Creates new memory pool from buffer. Returns ID of newly created pool.
    fn create_pool_from_buffer(&mut self, buffer: Buffer) -> MemoryPoolId;

    /// Schedules destruction of memory pool identified by given ID. The pool will be destructed
    /// when all its views go out of the scope.
    ///
    /// If the poll was created from mapped memory, returns this memory.
    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) -> Option<MappedMemory>;

    /// Replaces mapped memory with other memory reusing its ID. This method may be used when
    /// client requests memory map resize.
    fn replace_memory_pool(&mut self, mpid: MemoryPoolId, memory: MappedMemory);

    /// Creates new memory view from mapped memory.
    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId>;

    /// Destroys memory view.
    fn destroy_memory_view(&mut self, mpid: MemoryViewId);
}

// -------------------------------------------------------------------------------------------------

/// Screenshooting related functionality.
pub trait Screenshooting {
    /// Makes screenshot request.
    fn take_screenshot(&mut self, id: i32);

    /// Sets given buffer as results of screenshot.
    fn set_screenshot_buffer(&mut self, buffer: Buffer);

    /// Returns and forgets screenshot buffer.
    fn take_screenshot_buffer(&mut self) -> Option<Buffer>;
}

// -------------------------------------------------------------------------------------------------

/// Helper trait gathering traits used by `Exhibitor`. Keeping list of all traits in all
/// implementations is too verbose so this trait was introduced as best for now solution.
pub trait ExhibitorCoordinationTrait: SurfaceManagement +
                                      SurfaceControl +
                                      SurfaceViewer +
                                      SurfaceAccess +
                                      SurfaceListing +
                                      SurfaceFocusing +
                                      Emiter +
                                      MemoryManagement +
                                      Screenshooting {}

// -------------------------------------------------------------------------------------------------
