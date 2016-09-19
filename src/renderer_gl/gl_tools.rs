// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common GL-related tools.

// -------------------------------------------------------------------------------------------------

use std;
use gl;

use qualia::Error;

// -------------------------------------------------------------------------------------------------

pub enum GlslVersion {
    Unknown,
    Glsl100,
    Glsl300,
}

// -------------------------------------------------------------------------------------------------

/// Log GL error.
pub fn log_status() {
    let framebuffer_complete =
        unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE };

    log_info1!("Status - GL: 0x{:x}, framebuffer: {}complete",
               unsafe { gl::GetError() },
               if framebuffer_complete { "" } else { "NOT " });
}

// -------------------------------------------------------------------------------------------------

/// Get GL info log.
pub fn get_info_log(object: gl::types::GLuint) -> String {
    unsafe {
        let mut log_length: i32 = 0;
        if gl::IsShader(object) == gl::TRUE {
            gl::GetShaderiv(object, gl::INFO_LOG_LENGTH, &mut log_length);
        } else if gl::IsProgram(object) == gl::TRUE {
            gl::GetProgramiv(object, gl::INFO_LOG_LENGTH, &mut log_length);
        } else {
            return "GL: Not a shader or a program".to_owned();
        }

        let mut length: i32 = 0;
        let mut buffer = [0u8; 512];
        if gl::IsShader(object) == gl::TRUE {
            gl::GetShaderInfoLog(object,
                                 buffer.len() as i32,
                                 &mut length,
                                 buffer.as_mut_ptr() as *mut i8);
        } else if gl::IsProgram(object) == gl::TRUE {
            gl::GetProgramInfoLog(object,
                                  buffer.len() as i32,
                                  &mut length,
                                  buffer.as_mut_ptr() as *mut i8);
        }

        let cstr = std::ffi::CStr::from_ptr(std::mem::transmute(&buffer));
        format!("GL: {}",
                std::str::from_utf8(cstr.to_bytes()).expect("Info log is invalid"))
    }
}

// -------------------------------------------------------------------------------------------------

/// Get latests supported version of GL ES shading language.
pub fn get_shading_lang_version() -> GlslVersion {
    let version = unsafe {
        let ptr = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
        let cstr: &std::ffi::CStr = std::ffi::CStr::from_ptr(ptr as *const i8);
        std::str::from_utf8(cstr.to_bytes()).expect("Shading lang string is invalid")
    };

    if version.find("ES 3.").is_some() {
        GlslVersion::Glsl300
    } else if version.find("ES 1.").is_some() {
        GlslVersion::Glsl100
    } else {
        GlslVersion::Unknown
    }
}

// -------------------------------------------------------------------------------------------------

/// Create and compile shader.
fn create_shader(source: String,
                 shader_type: gl::types::GLenum)
                 -> Result<gl::types::GLuint, Error> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let cstr = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as gl::types::GLint) {
            let info_log = get_info_log(shader);
            gl::DeleteShader(shader);
            Err(Error::General(info_log))
        } else {
            Ok(shader)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Create and link shader program.
fn create_program(vertex_shader: gl::types::GLenum,
                  fragment_shader: gl::types::GLenum)
                  -> Result<gl::types::GLuint, Error> {
    unsafe {
        // Create program
        let shader_program = gl::CreateProgram();

        // Link with shaders
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Handle errors
        let mut link_ok = gl::FALSE as i32;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut link_ok);
        if link_ok == gl::TRUE as i32 {
            Ok(shader_program)
        } else {
            let info_log = get_info_log(shader_program);
            gl::DeleteProgram(shader_program);
            Err(Error::General(info_log))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Create program and link with shaders.
pub fn prepare_shader_program(vertex_source: String,
                              fragment_source: String)
                              -> Result<gl::types::GLuint, Error> {
    // Create vertex shader
    let vertex_shader = try!(create_shader(vertex_source, gl::VERTEX_SHADER));

    // Create fragment shader
    let fragment_shader = try!(create_shader(fragment_source, gl::FRAGMENT_SHADER));

    // Create and link shader program
    create_program(vertex_shader, fragment_shader)
}

// -------------------------------------------------------------------------------------------------

/// Get location attribute variable in linked program.
pub fn get_attrib_location(program: gl::types::GLuint,
                           name: String)
                           -> Result<gl::types::GLint, Error> {
    let cstr = std::ffi::CString::new(name.as_bytes()).unwrap();
    let location =
        unsafe { gl::GetAttribLocation(program, cstr.as_bytes_with_nul().as_ptr() as *const i8) };

    if location < 0 {
        Err(Error::General(format!("Could not get location for attribute '{}'", name)))
    } else {
        Ok(location)
    }
}

// -------------------------------------------------------------------------------------------------

/// Get location of uniform variable in linked program.
pub fn get_uniform_location(program: gl::types::GLuint,
                            name: String)
                            -> Result<gl::types::GLint, Error> {
    let cstr = std::ffi::CString::new(name.as_bytes()).unwrap();
    let location =
        unsafe { gl::GetUniformLocation(program, cstr.as_bytes_with_nul().as_ptr() as *const i8) };

    if location < 0 {
        Err(Error::General(format!("Could not get location for uniform '{}'", name)))
    } else {
        Ok(location)
    }
}

// -------------------------------------------------------------------------------------------------
