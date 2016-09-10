// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This is main module of Wayland Frontend crate. Currently it serves as bind between C and Rust
//! code. C code should be rewritten in Rust in future.

// -------------------------------------------------------------------------------------------------

use libc::c_char;
use std::ffi::CStr;
use std::str;

use timber;
use qualia::{Buffer, Coordinator, Size, SurfaceId, Vector};

// -------------------------------------------------------------------------------------------------

extern {
    fn noia_wayland_initialize(coordinator: *mut Coordinator);
}

// -------------------------------------------------------------------------------------------------

#[no_mangle]
pub extern fn noia_surface_create(coordinator: *mut Coordinator) -> SurfaceId {
    unsafe { (*coordinator).create_surface() }
}

#[no_mangle]
pub extern fn noia_surface_destroy(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).destroy_surface(sid) }
}

#[no_mangle]
pub extern fn noia_surface_attach(coordinator: *mut Coordinator,
                                  sid: SurfaceId,
                                  width: u32,
                                  height: u32,
                                  stride: u32,
                                  data: *mut u8,
                                  resource: *const u64) {
    unsafe {
        let capacity = (stride * height) as usize;
        let data = Vec::from_raw_parts(data, capacity, capacity);
        (*coordinator).attach(sid, Buffer::new(width, height, stride, data))
    }
}

#[no_mangle]
pub extern fn noia_surface_commit(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).commit_surface(sid) }
}

#[no_mangle]
pub extern fn noia_surface_show(coordinator: *mut Coordinator, sid: SurfaceId, reason: i32) {
    unsafe { (*coordinator).show_surface(sid, reason) }
}

#[no_mangle]
pub extern fn noia_surface_set_offset(coordinator: *mut Coordinator,
                                      sid: SurfaceId,
                                      offset: Vector) {
    unsafe { (*coordinator).set_surface_offset(sid, offset) }
}

#[no_mangle]
pub extern fn noia_surface_set_requested_size(coordinator: *mut Coordinator,
                                              sid: SurfaceId,
                                              size: Size) {
    unsafe { (*coordinator).set_surface_requested_size(sid, size) }
}

#[no_mangle]
pub extern fn noia_surface_reset_offset_and_requested_size(coordinator: *mut Coordinator,
                                                           sid: SurfaceId) {
}

#[no_mangle]
pub extern fn noia_surface_set_relative_position(coordinator: *mut Coordinator,
                                                 sid: SurfaceId,
                                                 offset: Vector) {
    unsafe { (*coordinator).set_surface_relative_position(sid, offset) }
}

#[no_mangle]
pub extern fn noia_surface_relate(coordinator: *mut Coordinator,
                                  sid: SurfaceId,
                                  parent_sid: SurfaceId) {
    unsafe { (*coordinator).relate_surfaces(sid, parent_sid) }
}

#[no_mangle]
pub extern fn noia_surface_set_as_cursor(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).set_surface_as_cursor(sid) }
}


#[no_mangle]
pub extern fn noia_print_log(log_level: *const c_char,
                             line_number: u32,
                             file_name: *const c_char,
                             buff: *const c_char) -> i32 {
    let log_level_bytes = unsafe { CStr::from_ptr(log_level) }.to_bytes();
    let file_name_bytes = unsafe { CStr::from_ptr(file_name) }.to_bytes();
    let buff_bytes = unsafe { CStr::from_ptr(buff) }.to_bytes();

    let log_level_str = str::from_utf8(log_level_bytes).unwrap();
    let file_name_str = str::from_utf8(file_name_bytes).unwrap();
    let buff_str = str::from_utf8(buff_bytes).unwrap();

    timber::timber(log_level_str, file_name_str, line_number, format_args!("{}", buff_str));
    0
}

// -------------------------------------------------------------------------------------------------

pub struct WaylandFrontend {
    i: i32,
}

impl WaylandFrontend {
    pub fn init(coordinator: &mut Coordinator) {
        unsafe {
            coordinator.create_surface();
            (*(coordinator as *mut Coordinator)).create_surface();
            noia_wayland_initialize(coordinator as *mut Coordinator);
        }
    }
}

// -------------------------------------------------------------------------------------------------
