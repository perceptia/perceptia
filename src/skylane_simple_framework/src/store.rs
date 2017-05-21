// Copyright 2017 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Store shared between proxies and controller.

use std;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use nix;

use skylane::client as wl;

// -------------------------------------------------------------------------------------------------

/// Helper structure for storing data related to screenshot.
pub struct ScreenshotStore {
    pub fd: RawFd,
    pub path: PathBuf,
    pub memory: *mut u8,
    pub size: usize,
    pub width: usize,
    pub height: usize,
}

// -------------------------------------------------------------------------------------------------

impl Drop for ScreenshotStore {
    fn drop(&mut self) {
        nix::unistd::close(self.fd).expect("Closing screenshot file");
        nix::unistd::unlink(&self.path).expect("Removing screenshot file");
    }
}

// -------------------------------------------------------------------------------------------------

/// Store shared between proxies and controller for cases when this data can not be shared via
/// `skylane` objects.
pub struct Store {
    pub registry_oid: Option<wl::ObjectId>,
    pub compositor_oid: Option<wl::ObjectId>,
    pub shell_oid: Option<wl::ObjectId>,
    pub drm_oid: Option<wl::ObjectId>,
    pub dmabuf_oid: Option<wl::ObjectId>,
    pub shm_oid: Option<wl::ObjectId>,
    pub screenshooter_oid: Option<wl::ObjectId>,
    pub screenshooter_name: Option<u32>,
    pub drm_device_name: Option<String>,
    pub screenshot: Option<ScreenshotStore>,
}

// -------------------------------------------------------------------------------------------------

impl Store {
    pub fn new() -> Self {
        Store {
            registry_oid: None,
            compositor_oid: None,
            shell_oid: None,
            drm_oid: None,
            dmabuf_oid: None,
            shm_oid: None,
            screenshooter_oid: None,
            screenshooter_name: None,
            drm_device_name: None,
            screenshot: None,
        }
    }

    /// Returns IDs of compositor, shell and dmabuf objects if available.
    pub fn ensure_dmabuf(&self) -> Option<(wl::ObjectId, wl::ObjectId, wl::ObjectId)> {
        if let Some(compositor_oid) = self.compositor_oid {
            if let Some(shell_oid) = self.shell_oid {
                if let Some(dmabuf_oid) = self.dmabuf_oid {
                    return Some((compositor_oid, shell_oid, dmabuf_oid));
                }
            }
        }
        None
    }

    /// Returns IDs of compositor, shell and DRM objects if available.
    pub fn ensure_drm(&self) -> Option<(wl::ObjectId, wl::ObjectId, wl::ObjectId)> {
        if let Some(compositor_oid) = self.compositor_oid {
            if let Some(shell_oid) = self.shell_oid {
                if let Some(drm_oid) = self.drm_oid {
                    return Some((compositor_oid, shell_oid, drm_oid));
                }
            }
        }
        None
    }
}

// -------------------------------------------------------------------------------------------------

define_ref!(struct Store as StoreRef);

// -------------------------------------------------------------------------------------------------
