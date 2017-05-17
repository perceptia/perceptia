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

//! Simple example application demonstrating EGL use with `skylane`.
//!
//! FIXME: This example shows only one frame. More work needs to be done.

extern crate nix;
extern crate gl;
extern crate egl;
extern crate gbm_rs as libgbm;
extern crate drm as libdrm;

extern crate graphics;
extern crate skylane_simple_framework;

use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::collections::HashSet;

use graphics::egl_tools;
use graphics::gl_tools;
use skylane_simple_framework::{Application, Controller};
use skylane_simple_framework::{Listener, ListenerConstructor};

// -------------------------------------------------------------------------------------------------

/// Vertex shader source code.
const VERTEX_SHADER_CODE: &'static str = "
uniform mat4 rotation;
attribute vec4 position;
attribute vec4 color;
varying vec4 v_color;
void main() {
    gl_Position = position;
    v_color = color;
}";

/// Fragment shader source code.
const FRAGMENT_SHADER_CODE: &'static str = "
varying vec4 v_color;
void main() {
    gl_FragColor = v_color;
}";

// -------------------------------------------------------------------------------------------------

const GLOBAL_COMPOSITOR: &'static str = "wl_compositor";
const GLOBAL_DRM: &'static str = "wl_drm";
const GLOBAL_SHELL: &'static str = "wl_shell";

fn main() {
    println!("skylane simple EGL demo");
    Application::new().run(SimpleEglConstructor::new())
}

// -------------------------------------------------------------------------------------------------

/// Constructor required by the `skylane` framework.
struct SimpleEglConstructor {}

impl SimpleEglConstructor {
    fn new() -> Self {
        SimpleEglConstructor{}
    }
}

impl ListenerConstructor for SimpleEglConstructor {
    type Listener = SimpleEgl;

    fn construct(&self, controller: Controller) -> Box<Self::Listener> {
        Box::new(SimpleEgl::new(controller))
    }
}

// -------------------------------------------------------------------------------------------------

/// Bundle of window related stuff.
pub struct EglWindow {
    pub device: libgbm::Device,
    pub display: egl::EGLDisplay,
    pub context: egl::EGLContext,

    pub image: *const std::os::raw::c_void,
    pub name: u32,

    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

// -------------------------------------------------------------------------------------------------

/// Main structure.
struct SimpleEgl {
    controller: Controller,
    fd: Option<RawFd>,
    window: Option<EglWindow>,
}

// -------------------------------------------------------------------------------------------------

impl SimpleEgl {
    /// Constructs new `SimpleEgl`.
    pub fn new(controller: Controller) -> Self {
        SimpleEgl {
            controller: controller,
            fd: None,
            window: None,
        }
    }

    /// Open default DRM device.
    fn open_device(&mut self) -> Result<RawFd, String> {
        // Open the device
        match nix::fcntl::open(&PathBuf::from("/dev/dri/card0"),
                               nix::fcntl::O_RDWR | nix::fcntl::O_CLOEXEC,
                               nix::sys::stat::Mode::empty()) {
            Ok(fd) => {
                Ok(fd)
            }
            Err(err) => {
                return Err(format!("Failed to open device: {}", err));
            }
        }
    }

    /// Calls "glEGLImageTargetRenderbufferStorageOES" extensions function wich allow to bind out
    /// frambuffer with image we will send to server.
    fn target_storage(&mut self,
                      target: u32,
                      image: *const std::os::raw::c_void)
                      -> Result<(), String> {
        if let Some(target_texture) =
                egl_tools::get_proc_address_of_image_target_renderbuffer_storage_oes() {
            target_texture(target, image);
            Ok(())
        } else {
            Err(format!("Failed to create target texture"))
        }
    }


    /// Creates new window.
    fn create_window(&mut self) -> Result<EglWindow, String> {
        let width: u32 = 800;
        let height: u32 = 600;

        let fd = if let Some(fd) = self.fd {
            fd
        } else {
            return Err(format!("DRM device unvailable"));
        };

        // Initialize EGL
        let device = if let Some(device) = libgbm::Device::from_fd(fd) {
            device
        } else {
            return Err(format!("Failed to create GBM device"));
        };

        let display = egl_tools::get_gbm_display(device.c_struct() as egl::EGLNativeDisplayType);
        let display = if let Ok(display) = display {
            display
        } else {
            return Err(format!("Failed to create display"));
        };

        let mut major = 0;
        let mut minor = 0;
        if !egl::initialize(display, &mut major, &mut minor) {
            return Err(format!("Failed to initialize EGL"));
        };

        if !egl::bind_api(egl::EGL_OPENGL_API) {
            return Err(format!("Failed to bind API"));
        };

        // Check extensions
        if !egl_tools::has_extension(egl::EGL_NO_DISPLAY, "EGL_MESA_platform_gbm") {
            return Err(format!("EGL does not provide GBM extension"));
        }

        // Choose config
        const CONFIG_ATTRIB_LIST: [egl::EGLint; 13] = [
                egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_BIT,
                egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
                egl::EGL_RED_SIZE,        1,
                egl::EGL_GREEN_SIZE,      1,
                egl::EGL_BLUE_SIZE,       1,
                egl::EGL_ALPHA_SIZE,      1,
                egl::EGL_NONE
            ];

        let config = if let Some(config) = egl::choose_config(display, &CONFIG_ATTRIB_LIST, 1) {
            config
        } else {
            return Err(format!("Failed to choose EGL config"));
        };

        const CONTEXT_ATTRIB_LIST: [egl::EGLint; 0] = [];
        let ctx = egl::create_context(display,
                                      std::ptr::null_mut(),
                                      egl::EGL_NO_CONTEXT,
                                      &CONTEXT_ATTRIB_LIST);
        let context = if let Some(context) = ctx {
            context
        } else {
            return Err(format!("Failed to create EGL context"));
        };

        // Create surface
        let gbm_surface = if let Some(gbm_surface) = libgbm::Surface::new(&device,
                                                                          width,
                                                                          height,
                                                                          libgbm::format::XRGB8888,
                                                                          libgbm::USE_RENDERING) {
            gbm_surface
        } else {
            return Err(format!("Failed to create GBM surface"));
        };

        // Create window surface
        let create_surface = egl_tools::get_proc_address_of_create_platform_surface();
        let surface = if let Some(create_surface) = create_surface {
            let surface = create_surface(display,
                                         config,
                                         gbm_surface.c_struct() as *mut _,
                                         std::ptr::null());
            if !surface.is_null() {
                surface
            } else {
                return Err(format!("Failed to create EGL surface"));
            }
        } else {
            return Err(format!("Failed to call create EGL surface"));
        };

        // Make context current
        if !egl::make_current(display, surface, surface, context) {
            return Err(format!("Failed to make current"));
        }

        // Create EGL DRM image
        let img = if let Some(create_img) = egl_tools::get_proc_address_of_create_drm_image_mesa() {
            let mut attribs = [egl::EGL_NONE; 9];

            attribs[0] = egl::EGL_WIDTH;
            attribs[1] = width as i32;
            attribs[2] = egl::EGL_HEIGHT;
            attribs[3] = height as i32;
            attribs[4] = egl_tools::ext::DRM_BUFFER_FORMAT_MESA;
            attribs[5] = egl_tools::ext::DRM_BUFFER_FORMAT_ARGB32_MESA;
            attribs[6] = egl_tools::ext::DRM_BUFFER_USE_MESA;
            attribs[7] = egl_tools::ext::DRM_BUFFER_USE_SHARE_MESA;
            attribs[8] = egl::EGL_NONE;

            create_img(display, &attribs as *const _)
        } else {
            return Err(format!("Failed to create image"));
        };

        // Initialize GL
        gl::load_with(|s| egl::get_proc_address(s) as *const std::os::raw::c_void);

        // Prepare all frame-, render- and vertex-buffers
        let mut vbo_vertices = 0;
        let mut vbo_colors = 0;
        let status = unsafe {
            let mut fbo = 0;
            let mut rbo_color = 0;
            let mut rbo_depth = 0;

            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::GenRenderbuffers(1, &mut rbo_color);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_color);
            self.target_storage(gl::RENDERBUFFER, img)?;
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER,
                                        gl::COLOR_ATTACHMENT0,
                                        gl::RENDERBUFFER,
                                        rbo_color);

            gl::GenRenderbuffers(1, &mut rbo_depth);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth);
            gl::RenderbufferStorage(gl::RENDERBUFFER,
                                    gl::DEPTH_COMPONENT,
                                    width as i32,
                                    height as i32);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER,
                                        gl::DEPTH_ATTACHMENT,
                                        gl::RENDERBUFFER,
                                        rbo_depth);

            gl::GenBuffers(1, &mut vbo_vertices);
            gl::GenBuffers(1, &mut vbo_colors);

            gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
        };

        if status != gl::FRAMEBUFFER_COMPLETE {
            return Err(format!("framebuffer not complete 2: {}", status));
        }

        // Prepare shader program
        let program = gl_tools::prepare_shader_program(VERTEX_SHADER_CODE.to_string(),
                                                       FRAGMENT_SHADER_CODE.to_string())?;
        let loc_position = gl_tools::get_attrib_location(program, "position".to_owned())?;
        let loc_color = gl_tools::get_attrib_location(program, "color".to_owned())?;

        // Render view
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.3, 0.3, 0.3, 0.5);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::UseProgram(program);

            // Setup data
            let vertices: [gl::types::GLfloat; 6] = [ -0.5, -0.5, 0.5, -0.5, 0.0, 0.5];
            let colors: [gl::types::GLfloat; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

            let float_size = std::mem::size_of::<gl::types::GLfloat>();
            let vertices_size = float_size * vertices.len();
            let colors_size = float_size * colors.len();

            // Load verticas
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
            gl::EnableVertexAttribArray(loc_position as _);
            gl::VertexAttribPointer(loc_position as _,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    2 * float_size as egl::EGLint,
                                    std::ptr::null());
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertices_size as isize,
                           vertices.as_ptr() as *const _,
                           gl::DYNAMIC_DRAW);

            // Load colors
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
            gl::EnableVertexAttribArray(loc_color as _);
            gl::VertexAttribPointer(loc_color as _,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    3 * float_size as egl::EGLint,
                                    std::ptr::null());
            gl::BufferData(gl::ARRAY_BUFFER,
                           colors_size as isize,
                           colors.as_ptr() as *const _,
                           gl::DYNAMIC_DRAW);

            // Draw
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::Finish();
        }

        // Export image
        let mut name: i32 = 0;
        let mut stride: i32 = 0;
        let mut handle: i32 = 0;
        if let Some(export_img) = egl_tools::get_proc_address_of_export_drm_image_mesa() {
            let result = export_img(display,
                                    img,
                                    &mut name as *mut _,
                                    &mut handle as *mut _,
                                    &mut stride as *mut _);
            if result == egl::EGL_FALSE {
                return Err(format!("Failed to export image"));
            }
        } else {
            return Err(format!("Failed to call export image"));
        };

        // Create the window
        let window = EglWindow {
            device: device,
            display: display,
            context: context,
            image: img,
            name: name as u32,
            width: width as usize,
            height: height as usize,
            stride: stride as usize,
        };

        Ok(window)
    }
}

// -------------------------------------------------------------------------------------------------

impl Listener for SimpleEgl {
    /// Server finished sending globals. Check if it supports everything we need.
    fn globals_done(&mut self, globals: HashSet<String>) {
        println!("Globals done");
        for global in vec![GLOBAL_COMPOSITOR, GLOBAL_DRM, GLOBAL_SHELL] {
            if !globals.contains(global) {
                println!("Server does not provide '{}' global interface", global);
                self.controller.stop();
            }
        }
    }

    /// All requests from initialization faze were processed. We open device and ask for
    /// authentication.
    fn init_done(&mut self) {
        println!("Init done");
        if let Ok(fd) = self.open_device() {
            self.controller.initialize_graphics(fd);
            self.fd = Some(fd);
        }
    }

    /// Authentication of DRM device was succesfull. Now we can start drawing.
    fn graphics_done(&mut self, _device_name: String) {
        println!("Creating surface");
        let window = self.create_window().expect("Surface creation");
        self.controller.create_egl_surface(window.name, window.width, window.height, window.stride);
        self.window = Some(window);
    }

    /// Exit if authentication failed.
    fn graphics_failed(&mut self) {
        println!("Failed to initialize hardware accelerated graphics!");
        self.controller.stop();
    }
}

// -------------------------------------------------------------------------------------------------
