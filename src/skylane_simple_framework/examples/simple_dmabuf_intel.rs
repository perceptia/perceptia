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

//! Simple example application demonstrating dmabuf use with `skylane`.
//!
//! FIXME: This example shows only one frame. More work needs to be done.

extern crate nix;
extern crate drm as libdrm;

extern crate graphics;
extern crate skylane_simple_framework;

use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::collections::HashSet;
use std::ffi::CString;

use skylane_simple_framework::{Application, Controller};
use skylane_simple_framework::{Listener, ListenerConstructor};

// -------------------------------------------------------------------------------------------------

/// TODO: Move these bindings to drm crate.
mod ffi {
    use nix::libc::{c_int, c_char, c_ulong, c_void};

    pub enum DrmIntelBufmgr {}

    #[repr(C)]
    pub struct DrmIntelBo {
        pub size: c_ulong,
        pub align: c_ulong,
        pub offset: c_ulong,
        pub virt: *mut c_void,
        pub bufmgr: *mut DrmIntelBufmgr,
        pub handle: c_int,
        pub offset64: u64,
    }

    #[link(name="drm_intel")]
    extern {
        pub fn drm_intel_bufmgr_gem_init(fd: c_int, batch_size: c_int) -> *mut DrmIntelBufmgr;

        pub fn drm_intel_bo_alloc_tiled(bufmgr: *mut DrmIntelBufmgr,
                                        name: *const c_char,
                                        x: c_int,
                                        y: c_int,
                                        cpp: c_int,
                                        tiling_mode: *mut u32,
                                        pitch: *mut c_ulong,
                                        flags: c_ulong)
                                        -> *mut DrmIntelBo;

        pub fn drm_intel_gem_bo_map_gtt(bo: *mut DrmIntelBo) -> c_int;

        pub fn drm_intel_bo_gem_export_to_prime(bo: *mut DrmIntelBo,
                                                prime_fd: *mut c_int) -> c_int;
    }
}

// -------------------------------------------------------------------------------------------------

const GLOBAL_COMPOSITOR: &'static str = "wl_compositor";
const GLOBAL_DMABUF: &'static str = "zwp_linux_dmabuf_v1";
const GLOBAL_SHELL: &'static str = "wl_shell";

fn main() {
    println!("skylane simple DRM demo");
    Application::new().run(SimpleDmabufConstructor::new())
}

// -------------------------------------------------------------------------------------------------

/// Constructor required by the `skylane` framework.
struct SimpleDmabufConstructor {}

impl SimpleDmabufConstructor {
    fn new() -> Self {
        SimpleDmabufConstructor{}
    }
}

impl ListenerConstructor for SimpleDmabufConstructor {
    type Listener = SimpleDmabuf;

    fn construct(&self, controller: Controller) -> Box<Self::Listener> {
        Box::new(SimpleDmabuf::new(controller))
    }
}

// -------------------------------------------------------------------------------------------------

/// Bundle of window related stuff.
pub struct DmabufWindow {
    pub prime_fd: RawFd,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

// -------------------------------------------------------------------------------------------------

/// Main structure.
struct SimpleDmabuf {
    controller: Controller,
    window: Option<DmabufWindow>,
}

// -------------------------------------------------------------------------------------------------

impl SimpleDmabuf {
    /// Constructs new `SimpleDmabuf`.
    pub fn new(controller: Controller) -> Self {
        SimpleDmabuf {
            controller: controller,
            window: None,
        }
    }

    /// Creates new window.
    fn create_window(&mut self, device_name: &str) -> Result<DmabufWindow, String> {
        let width: u32 = 800;
        let height: u32 = 600;
        let name = CString::new("test").unwrap();

        // Open the device
        let fd = {
            match nix::fcntl::open(&PathBuf::from(device_name),
                                   nix::fcntl::O_RDWR | nix::fcntl::O_CLOEXEC,
                                   nix::sys::stat::Mode::empty()) {
                Ok(fd) => {
                    fd
                }
                Err(err) => {
                    return Err(format!("Failed to open device: {}", err));
                }
            }
        };

        let bufmgr = unsafe { ffi::drm_intel_bufmgr_gem_init(fd, 32) };
        if bufmgr == std::ptr::null_mut() {
            return Err(format!("Failed to initalize buffer manager"));
        }

        let mut tiling = 0;
        let mut stride = 0;
        let bo = unsafe {
            ffi::drm_intel_bo_alloc_tiled(bufmgr,
                                          name.as_ptr(),
                                          width as i32,
                                          height as i32,
                                          8,
                                          &mut tiling,
                                          &mut stride,
                                          0x0)
        };

        let result = unsafe { ffi::drm_intel_gem_bo_map_gtt(bo) };
        if result != 0 {
            return Err(format!("Failed to map buffer object: {}", result));
        }
        if unsafe { (*bo).virt } == std::ptr::null_mut() {
            return Err(format!("Buffer object map is invalid"));
        }

        unsafe {
            let x_scale = 255.0 / width as f32;
            let y_scale = 255.0 / height as f32;
            for y in 0..height as isize {
                let fy = y as f32;
                let row = (*bo).virt.offset(y * stride as isize);
                for x in 0..width as isize {
                    let fx = x as f32;
                    let pix = row.offset(4 * x) as *mut u8;
                    *pix.offset(0) = (fx * x_scale) as u8;
                    *pix.offset(1) = (fy * y_scale) as u8;
                    *pix.offset(2) = 0x00;
                    *pix.offset(3) = 0xff;
                }
            }
        }

        let mut prime_fd = -1;
        let result = unsafe { ffi::drm_intel_bo_gem_export_to_prime(bo, &mut prime_fd) };
        if result != 0 {
            return Err(format!("Exporting to prime failed"));
        }
        if prime_fd < 0 {
            return Err(format!("prime fd < 0"));
        }

        let window = DmabufWindow {
            prime_fd: prime_fd,
            width: width as usize,
            height: height as usize,
            stride: stride as usize,
        };

        Ok(window)
    }
}

// -------------------------------------------------------------------------------------------------

impl Listener for SimpleDmabuf {
    fn globals_done(&mut self, globals: HashSet<String>) {
        println!("Globals done");
        for global in vec![GLOBAL_COMPOSITOR, GLOBAL_DMABUF, GLOBAL_SHELL] {
            if !globals.contains(global) {
                println!("Server does not provide '{}' global interface", global);
                self.controller.stop();
            }
        }
    }

    fn init_done(&mut self) {
        println!("Init done");
        let win = self.create_window("/dev/dri/renderD128").expect("Surface creation");
        self.controller.create_dmabuf_surface(win.prime_fd, win.width, win.height, win.stride);
        self.window = Some(win);
    }

    fn graphics_failed(&mut self) {
        println!("Failed to initialize hardware accelerated graphics!");
        self.controller.stop();
    }
}

// -------------------------------------------------------------------------------------------------
