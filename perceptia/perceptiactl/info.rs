// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Printing useful information about system and devices.

use std;
use std::os::unix::io;
use nix::fcntl::{self, open};
use nix::unistd::close;
use nix::sys::stat::Mode;

use egl;
use gl;
use libudev;
use libdrm::drm_mode;

use qualia;
use device_manager;
use cognitive_graphics::{gbm_tools, egl_tools};

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

fn print_crtcs(fd: io::RawFd, resources: &drm_mode::Resources) {
    println!("\t\tcount ctrcs: {}", resources.get_count_crtcs());
    for id in resources.get_crtcs() {
        match drm_mode::get_crtc(fd, id) {
            Some(crtc) => println!("\t\t - {:?}", crtc),
            None => println!("\t\t - failed to get info"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn print_encoders(fd: io::RawFd, resources: &drm_mode::Resources) {
    println!("\t\tcount encoders: {}", resources.get_count_encoders());
    for id in resources.get_encoders() {
        match drm_mode::get_encoder(fd, id) {
            Some(encoder) => println!("\t\t - {:?}", encoder),
            None => println!("\t\t - failed to get info"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn print_connectors(fd: io::RawFd, resources: &drm_mode::Resources) {
    println!("\t\tcount connectors: {}", resources.get_count_connectors());
    for id in resources.get_connectors() {
        match drm_mode::get_connector(fd, id) {
            Some(connector) => println!("\t\t - {:?}", connector),
            None => println!("\t\t - failed to get info"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn print_drm_info(fd: io::RawFd) {
    println!("\tDRM info:");

    match drm_mode::get_resources(fd) {
        Some(resources) => {
            println!("\t\tcount fbs: {}", resources.get_count_fbs());
            print_crtcs(fd, &resources);
            print_encoders(fd, &resources);
            print_connectors(fd, &resources);
        }
        None => println!("\t\tNo resources"),
    }
}

// -------------------------------------------------------------------------------------------------

fn print_egl_info(fd: io::RawFd) {
    println!("\tEGL/GL info:");

    // Prepare GBM device and create surface
    let size = qualia::Size::new(16, 16);
    let gbm = match gbm_tools::GbmBucket::new(fd, size.width as u32, size.height as u32) {
        Ok(gbm) => gbm,
        Err(err) => {
            println!("\t\tError: {}", err);
            return;
        }
    };

    // Create on-screen EGL
    let egl = egl_tools::EglBucket::new(gbm.device.c_struct() as *mut _,
                                        gbm.surface.c_struct() as *mut _);
    let egl = match egl {
        Ok(egl) => egl,
        Err(err) => {
            println!("\t\tError {}", err);
            return;
        }
    };

    // Print EGL info
    println!("\t\tEGL version:  {:?}", egl::query_string(egl.display, egl::EGL_VERSION).unwrap());
    println!("\t\tEGL vendor:   {:?}", egl::query_string(egl.display, egl::EGL_VENDOR).unwrap());
    println!("\t\tEGL extensions:");
    for e in vec![egl_tools::ext::IMAGE_BASE_EXT] {
        let has = if egl_tools::has_extension(egl.display, e) {
            "yes"
        } else {
            "NO"
        };
        println!("\t\t\t{}: {}", e, has);
    }

    // Make EGL context current
    let _ctx = match egl.make_current() {
        Ok(ctx) => ctx,
        Err(err) => {
            println!("\t\tFailed to make EGL context current: {}", err);
            return;
        }
    };

    // Print GL info
    gl::load_with(|s| egl::get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        println!("\t\tGL vendor:    {:?}", ptr_to_string(gl::GetString(gl::VENDOR)));
        println!("\t\tGL renderer:  {:?}", ptr_to_string(gl::GetString(gl::RENDERER)));
        println!("\t\tGL version:   {:?}", ptr_to_string(gl::GetString(gl::VERSION)));
        println!("\t\tGLSL version: {:?}",
                 ptr_to_string(gl::GetString(gl::SHADING_LANGUAGE_VERSION)));
    }

    egl.destroy();
}

// -------------------------------------------------------------------------------------------------

fn print_devices() {
    let udev = device_manager::udev::Udev::new();
    udev.iterate_event_devices(|devnode, device_kind, device| {
                                   println!("{:?}: ({:?})", device_kind, devnode);
                                   print_properties_and_attributes(&device);
                                   println!("");
                               });

    udev.iterate_drm_devices(|devnode, device| {
        println!("display: ({:?})", devnode);
        print_properties_and_attributes(&device);

        // Open device
        match open(devnode, fcntl::O_RDWR | fcntl::O_CLOEXEC, Mode::empty()) {
            Ok(fd) => {
                print_drm_info(fd);
                print_egl_info(fd);
                close(fd).unwrap();
            }
            Err(err) => {
                println!("\t\tCould not open device {}", err);
            }
        };

        println!("");
    });
}

// -------------------------------------------------------------------------------------------------
