// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions related to outputs.

use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use defs::{Area, Size};

// -------------------------------------------------------------------------------------------------

/// Set of informations about output.
#[derive(Clone, Debug)]
pub struct OutputInfo {
    pub id: i32, // TODO: Define new type for output ID.
    pub area: Area,
    pub physical_size: Size,
    pub refresh_rate: usize,
    pub make: String,
    pub model: String,
}

// -------------------------------------------------------------------------------------------------

impl OutputInfo {
    /// Constructs new `OutputInfo`.
    pub fn new(id: i32,
               area: Area,
               physical_size: Size,
               refresh_rate: usize,
               make: String,
               model: String)
               -> Self {
        OutputInfo {
            id: id,
            area: area,
            physical_size: physical_size,
            refresh_rate: refresh_rate,
            make: make,
            model: model,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure containing all data needed to initialize DRM output.
#[derive(Clone, Debug)]
pub struct DrmBundle {
    pub path: PathBuf,
    pub fd: RawFd,
    pub crtc_id: u32,
    pub connector_id: u32,
}

// -------------------------------------------------------------------------------------------------

/// This structure defines data shared between virtual outputs and their manager.
#[derive(Clone, Debug)]
pub struct VirtualFramebuffer {
    /// Contains data of whole virtual framebuffer shared between outputs. For remote desktop
    /// solutions it is more efficient to allow all outputs draw to the same buffer. When sending
    /// data to client the data will not have to be glued together.
    ///
    /// This data is not double buffered. `VirtualFramebuffer` should be used as
    /// `Arc<Mutex<VirtualFramebuffer>>`.
    pub data: Vec<u8>,

    /// List of displays to be notified about sending data to its consumer.
    pub vblank_subscribers: Vec<i32>,
}

// -------------------------------------------------------------------------------------------------

impl VirtualFramebuffer {
    /// Constructs new `VirtualFramebuffer`.
    pub fn new(data: Vec<u8>) -> Self {
        VirtualFramebuffer {
            data: data,
            vblank_subscribers: Vec::new(),
        }
    }

    /// Returns contents for the framebuffer as slice.
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Returns contents for the framebuffer as mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Subscribes given display for notification about sending data to its consumer.
    pub fn subscribe_for_vblank(&mut self, display_id: i32) {
        self.vblank_subscribers.push(display_id);
    }

    /// Takes the list of subscribers leaving internal list empty.
    pub fn take_subscribers(&mut self) -> Vec<i32> {
        let result = self.vblank_subscribers.split_off(0);
        result
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure containing all data needed to initialize virtual output.
#[derive(Clone, Debug)]
pub struct VirtualOutputBundle {
    pub vfb: Arc<RwLock<VirtualFramebuffer>>,
    pub offset: usize,
    pub stride: usize,
    pub area: Area,
}

// -------------------------------------------------------------------------------------------------

impl VirtualOutputBundle {
    /// Constructs new `VirtualOutputBundle`.
    pub fn new(vfb: Arc<RwLock<VirtualFramebuffer>>,
               offset: usize,
               stride: usize,
               area: Area) -> Self {
        VirtualOutputBundle {
            vfb: vfb,
            offset: offset,
            stride: stride,
            area: area,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Enumeration for all possible output types containing data need for their construction.
#[derive(Debug, Clone)]
pub enum OutputType {
    Virtual(VirtualOutputBundle),
    Drm(DrmBundle),
}

// -------------------------------------------------------------------------------------------------
