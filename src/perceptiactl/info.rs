// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

use std;
use nix::fcntl::{self, open};
use nix::sys::stat::Mode;

use egl;
use gl;
use libudev;

use qualia;
use device_manager;
use renderer_gl;
use output;

// -------------------------------------------------------------------------------------------------

fn ptr_to_string(ptr: *const u8) -> String {
    if ptr != std::ptr::null() {
        let cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) };
        match std::str::from_utf8(cstr.to_bytes()) {
            Ok(s) => s.to_owned(),
            Err(_) => "(invalid)".to_owned(),
        }
    } else {
        "(null)".to_owned()
    }
}

// -------------------------------------------------------------------------------------------------

pub fn process() {
    print_devices();
}

// -------------------------------------------------------------------------------------------------

fn print_properties_and_attributes(device: &libudev::Device) {
    println!("\tProperties:");
    for p in device.properties() {
        println!("\t\t{:?}: {:?}", p.name(), p.value())
    }
    println!("\tAttributes:");
    for a in device.attributes() {
        if let Some(value) = a.value() {
            println!("\t\t{:?}: {:?}", a.name(), value);
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn print_egl_info(devnode: &std::path::Path) {
    println!("\tEGL/GL info:");

    // Open device
    let fd = match open(devnode, fcntl::O_RDONLY, Mode::empty()) {
        Ok(fd) => fd,
        Err(err) => {
            println!("\t\tCould not open device {}", err);
            return;
        }
    };

    // Prepare GBM device and create surface
    let size = qualia::Size::new(16, 16);
    let gbm = match output::gbm_tools::GbmBundle::new(fd, size) {
        Ok(gbm) => gbm,
        Err(err) => {
            println!("\t\t{}", err);
            return;
        }
    };

    // Create on-screen EGL
    let egl = match renderer_gl::egl_tools::EglBundle::new(gbm.device.c_struct() as *mut _,
                                                           gbm.surface.c_struct() as *mut _) {
        Ok(egl) => egl,
        Err(err) => {
            println!("\t\t{}", err);
            return;
        }
    };

    // Print EGL info
    println!("\t\tEGL version:  {:?}",
             egl::query_string(egl.display, egl::EGL_VERSION).unwrap());
    println!("\t\tEGL vendor:   {:?}",
             egl::query_string(egl.display, egl::EGL_VENDOR).unwrap());

    // Make EGL context current
    let ctx = match egl.make_current() {
        Ok(ctx) => ctx,
        Err(err) => {
            println!("\t\tFailed to make EGL context current: {}", err);
            return;
        }
    };

    // Print GL info
    gl::load_with(|s| egl::get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        println!("\t\tGL vendor:    {:?}",
                 ptr_to_string(gl::GetString(gl::VENDOR)));
        println!("\t\tGL renderer:  {:?}",
                 ptr_to_string(gl::GetString(gl::RENDERER)));
        println!("\t\tGL version:   {:?}",
                 ptr_to_string(gl::GetString(gl::VERSION)));
        println!("\t\tGLSL version: {:?}",
                 ptr_to_string(gl::GetString(gl::SHADING_LANGUAGE_VERSION)));
    }
}

// -------------------------------------------------------------------------------------------------

fn print_devices() {
    let udev = device_manager::udev::Udev::new();
    udev.iterate_event_devices(|devnode, device| {
        println!("{:?}: ({:?})",
                 device_manager::udev::determine_device_kind(&device),
                 devnode);
        print_properties_and_attributes(&device);
        println!("");
    });

    udev.iterate_drm_devices(|devnode, device| {
        println!("display: ({:?})", devnode);
        print_properties_and_attributes(&device);
        print_egl_info(&devnode);
        println!("");
    });
}

// -------------------------------------------------------------------------------------------------
