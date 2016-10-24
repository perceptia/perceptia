// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This is main module of Wayland Frontend crate. Currently it serves as bind between C and Rust
//! code. C code should be rewritten in Rust in future.

// -------------------------------------------------------------------------------------------------

use libc::c_char;
use std::ffi::CStr;
use std::{str, mem};

use timber;
use qualia::{Buffer, Coordinator, Size, SurfaceId, SurfaceIdType, Key, Position, SurfacePosition, Vector, KeymapSettings};

// -------------------------------------------------------------------------------------------------

extern "C" {
    fn noia_wayland_initialize(coordinator: *mut Coordinator, keymap_settings: *mut KeymapSettings);
    fn noia_wayland_advertise_output();
    fn noia_wayland_module_on_keyboard_event(time: u32, code: u32, value: u32);
    fn noia_wayland_module_on_surface_frame(sid: SurfaceIdType);
    fn noia_wayland_module_on_pointer_focus_changed(sid: SurfaceIdType, pos: Position);
    fn noia_wayland_module_on_pointer_relative_motion(sid: SurfaceIdType, pos: Position);
    fn noia_wayland_module_on_keyboard_focus_changed(old_sid: SurfaceIdType,
                                                     old_size: Size,
                                                     old_flags: u32,
                                                     new_sid: SurfaceIdType,
                                                     new_size: Size,
                                                     new_flags: u32);
    fn noia_wayland_module_on_surface_reconfigured(sid: SurfaceIdType, size: Size, state_flags: u32);
}

// -------------------------------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn noia_surface_create(coordinator: *mut Coordinator) -> SurfaceId {
    unsafe { (*coordinator).create_surface() }
}

#[no_mangle]
pub extern "C" fn noia_surface_destroy(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).destroy_surface(sid) }
}

#[no_mangle]
pub extern "C" fn noia_surface_attach(coordinator: *mut Coordinator,
                                      sid: SurfaceId,
                                      width: u32,
                                      height: u32,
                                      stride: u32,
                                      data: *mut u8,
                                      resource: *const u64) {
    unsafe {
        let capacity = (stride * height) as usize;
        let data = Vec::from_raw_parts(data, capacity, capacity);
        let buffer = Buffer::new(width as usize, height as usize, stride as usize, data.clone());
        (*coordinator).attach(sid, buffer);
        mem::forget(data);
    }
}

#[no_mangle]
pub extern "C" fn noia_surface_commit(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).commit_surface(sid) }
}

#[no_mangle]
pub extern "C" fn noia_surface_show(coordinator: *mut Coordinator, sid: SurfaceId, reason: i32) {
    unsafe { (*coordinator).show_surface(sid, reason) }
}

#[no_mangle]
pub extern "C" fn noia_surface_set_offset(coordinator: *mut Coordinator,
                                          sid: SurfaceId,
                                          offset: Vector) {
    unsafe { (*coordinator).set_surface_offset(sid, offset) }
}

#[no_mangle]
pub extern "C" fn noia_surface_set_requested_size(coordinator: *mut Coordinator,
                                                  sid: SurfaceId,
                                                  size: Size) {
    unsafe { (*coordinator).set_surface_requested_size(sid, size) }
}

#[no_mangle]
pub extern "C" fn noia_surface_reset_offset_and_requested_size(coordinator: *mut Coordinator,
                                                               sid: SurfaceId) {
}

#[no_mangle]
pub extern "C" fn noia_surface_set_relative_position(coordinator: *mut Coordinator,
                                                     sid: SurfaceId,
                                                     offset: Vector) {
    unsafe { (*coordinator).set_surface_relative_position(sid, offset) }
}

#[no_mangle]
pub extern "C" fn noia_surface_relate(coordinator: *mut Coordinator,
                                      sid: SurfaceId,
                                      parent_sid: SurfaceId) {
    unsafe { (*coordinator).relate_surfaces(sid, parent_sid) }
}

#[no_mangle]
pub extern "C" fn noia_surface_set_as_cursor(coordinator: *mut Coordinator, sid: SurfaceId) {
    unsafe { (*coordinator).set_surface_as_cursor(sid) }
}


#[no_mangle]
pub extern "C" fn noia_print_log(log_level: *const c_char,
                                 line_number: u32,
                                 file_name: *const c_char,
                                 buff: *const c_char)
                                 -> i32 {
    let log_level_bytes = unsafe { CStr::from_ptr(log_level) }.to_bytes();
    let file_name_bytes = unsafe { CStr::from_ptr(file_name) }.to_bytes();
    let buff_bytes = unsafe { CStr::from_ptr(buff) }.to_bytes();

    let log_level_str = str::from_utf8(log_level_bytes).unwrap();
    let file_name_str = str::from_utf8(file_name_bytes).unwrap();
    let buff_str = str::from_utf8(buff_bytes).unwrap();

    timber::timber(log_level_str,
                   file_name_str,
                   line_number,
                   format_args!("{}", buff_str));
    0
}

// -------------------------------------------------------------------------------------------------

pub struct WaylandFrontend {
    i: i32,
}

impl WaylandFrontend {
    pub fn init(coordinator: &mut Coordinator, settings: &mut KeymapSettings) {
        unsafe {
            coordinator.create_surface();
            (*(coordinator as *mut Coordinator)).create_surface();
            noia_wayland_initialize(coordinator as *mut Coordinator,
                                    settings as *mut KeymapSettings);
        }
    }

    pub fn on_output_found() {
        unsafe {
            noia_wayland_advertise_output();
        }
    }

    pub fn on_keyboard_input(key: Key) {
        unsafe {
            noia_wayland_module_on_keyboard_event(0, key.code as u32, key.value as u32);
        }
    }

    pub fn on_surface_frame(sid: SurfaceId) {
        unsafe {
            noia_wayland_module_on_surface_frame(sid.as_number());
        }
    }

    pub fn on_pointer_focus_changed(surface_position: SurfacePosition) {
        unsafe {
            noia_wayland_module_on_pointer_focus_changed(surface_position.sid.as_number(),
                                                         surface_position.pos.clone());
        }
    }

    pub fn on_pointer_relative_motion(surface_position: SurfacePosition) {
        unsafe {
            noia_wayland_module_on_pointer_relative_motion(surface_position.sid.as_number(),
                                                           surface_position.pos.clone());
        }
    }

    pub fn on_keyboard_focus_changed(old_sid: SurfaceId,
                                     old_size: Size,
                                     old_flags: u32,
                                     new_sid: SurfaceId,
                                     new_size: Size,
                                     new_flags: u32) {
        unsafe {
            noia_wayland_module_on_keyboard_focus_changed(old_sid.as_number(),
                                                          old_size,
                                                          old_flags,
                                                          new_sid.as_number(),
                                                          new_size,
                                                          new_flags);
        }
    }

    pub fn on_surface_reconfigured(sid: SurfaceId, size: Size, state_flags: u32) {
        unsafe {
            noia_wayland_module_on_surface_reconfigured(sid.as_number(), size, state_flags);
        }
    }
}

// -------------------------------------------------------------------------------------------------
