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
    pub shm_oid: Option<wl::ObjectId>,
    pub screenshooter_oid: Option<wl::ObjectId>,
    pub screenshot: Option<ScreenshotStore>,
}

// -------------------------------------------------------------------------------------------------

impl Store {
    pub fn new() -> Self {
        Store {
            shm_oid: None,
            screenshooter_oid: None,
            screenshot: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

define_ref!(struct Store as StoreRef);

// -------------------------------------------------------------------------------------------------
