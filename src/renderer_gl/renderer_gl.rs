// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains GL renderer which allows drawing frame scenes with GL.

// -------------------------------------------------------------------------------------------------

use std;
use gl;
use egl;

use graphics::egl_tools;
use graphics::gl_tools;

use qualia::{SurfaceViewer, SurfaceContext, Illusion, Size};
use qualia::{Buffer, Pixmap, PixelFormat, DataSource, Image};

// -------------------------------------------------------------------------------------------------

const MAX_TEXTURES: u32 = 32;

/// Vertex shader source code for OpenGL ES 2.0 (GLSL ES 100)
const VERTEX_SHADER_100: &'static str = include_str!("vertex.100.glsl");

/// Fragment shader source code for OpenGL ES 2.0 (GLSL ES 100)
const FRAGMENT_SHADER_100: &'static str = include_str!("fragment.100.glsl");

/// Vertex shader source code for OpenGL ES 3.0 (GLSL ES 300)
const VERTEX_SHADER_300: &'static str = include_str!("vertex.300.glsl");

/// Fragment shader source code for OpenGL ES 3.0 (GLSL ES 300)
const FRAGMENT_SHADER_300: &'static str = include_str!("fragment.300.glsl");

// -------------------------------------------------------------------------------------------------

/// GL renderer.
pub struct RendererGl {
    egl: egl_tools::EglBucket,
    size: Size,

    // GL rendering
    program: gl::types::GLuint,
    loc_vertices: gl::types::GLint,
    loc_texcoords: gl::types::GLint,
    loc_texture: gl::types::GLint,
    loc_screen_size: gl::types::GLint,
    vbo_vertices: gl::types::GLuint,
    vbo_texcoords: gl::types::GLuint,
    vbo_texture: [gl::types::GLuint; MAX_TEXTURES as usize],

    // Pointers to extension functions
    image_target_texture: Option<egl_tools::ImageTargetTexture2DOesFunc>,
}

// -------------------------------------------------------------------------------------------------

impl RendererGl {
    /// `RendererGl` constructor.
    pub fn new(egl: egl_tools::EglBucket, size: Size) -> Self {
        RendererGl {
            egl: egl,
            size: size,
            program: gl::types::GLuint::default(),
            loc_vertices: gl::types::GLint::default(),
            loc_texcoords: gl::types::GLint::default(),
            loc_texture: gl::types::GLint::default(),
            loc_screen_size: gl::types::GLint::default(),
            vbo_vertices: gl::types::GLuint::default(),
            vbo_texcoords: gl::types::GLuint::default(),
            vbo_texture: [0; MAX_TEXTURES as usize],
            image_target_texture: None,
        }
    }

    /// Initialize renderer.
    ///  - prepare shaders and program,
    ///  - bind locations,
    ///  - generate buffers,
    ///  - configure textures,
    pub fn initialize(&mut self) -> Result<(), Illusion> {
        gl::load_with(|s| egl::get_proc_address(s) as *const std::os::raw::c_void);

        let _context = self.egl.make_current()?;

        // Get GLSL version
        let (vshader_src, fshader_src) = match gl_tools::get_shading_lang_version() {
            gl_tools::GlslVersion::Glsl100 => {
                (VERTEX_SHADER_100.to_owned(), FRAGMENT_SHADER_100.to_owned())
            }
            gl_tools::GlslVersion::Glsl300 => {
                (VERTEX_SHADER_300.to_owned(), FRAGMENT_SHADER_300.to_owned())
            }
            gl_tools::GlslVersion::Unknown => {
                return Err(Illusion::General(format!("Could not figure out GLSL version")));
            }
        };

        // Compile shades, link program and get locations
        self.program = gl_tools::prepare_shader_program(vshader_src, fshader_src)?;
        self.loc_vertices = gl_tools::get_attrib_location(self.program, "vertices".to_owned())?;
        self.loc_texcoords = gl_tools::get_attrib_location(self.program, "texcoords".to_owned())?;
        self.loc_texture = gl_tools::get_uniform_location(self.program, "texture".to_owned())?;
        self.loc_screen_size = gl_tools::get_uniform_location(self.program,
                                                              "screen_size".to_owned())?;

        // Generate vertex buffer object
        unsafe {
            gl::GenBuffers(1, &mut self.vbo_vertices);
            gl::GenBuffers(1, &mut self.vbo_texcoords);
        }

        // Create texture buffer
        // FIXME: Implement support for more textures.
        unsafe {
            gl::GenTextures(MAX_TEXTURES as i32, (&mut self.vbo_texture).as_mut_ptr());
            for i in 0..MAX_TEXTURES {
                gl::ActiveTexture(gl::TEXTURE0 + 1);
                gl::BindTexture(gl::TEXTURE_2D, self.vbo_texture[i as usize]);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            }
        }

        self.image_target_texture = egl_tools::get_proc_address_of_image_target_texture_2d_oes();

        Ok(())
    }

    /// Draw passed frame scene.
    pub fn draw(&mut self,
                layunder: &Vec<SurfaceContext>,
                surfaces: &Vec<SurfaceContext>,
                layover: &Vec<SurfaceContext>,
                viewer: &SurfaceViewer)
                -> Result<(), Illusion> {
        let _context = self.egl.make_current()?;
        self.prepare_view();
        self.draw_surfaces(layunder, viewer);
        self.draw_surfaces(surfaces, viewer);
        self.draw_surfaces(layover, viewer);
        self.release_view();
        Ok(())
    }

    /// Swap buffers.
    pub fn swap_buffers(&mut self) -> Result<(), Illusion> {
        let context = self.egl.make_current()?;
        context.swap_buffers()
    }

    /// Reads pixels for whole screen and returns image data as `Buffer`.
    pub fn take_screenshot(&self) -> Result<Buffer, Illusion> {
        let _context = self.egl.make_current()?;

        let format = PixelFormat::ARGB8888;
        let stride = format.get_size() * self.size.width;
        let size = stride * self.size.height;
        let mut dst: Vec<u8> = Vec::with_capacity(size);
        unsafe { dst.set_len(size) };

        unsafe {
            gl::ReadBuffer(gl::BACK);
            gl::ReadPixels(0,
                           0,
                           self.size.width as i32,
                           self.size.height as i32,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           dst.as_mut_ptr() as *mut std::os::raw::c_void);
        }

        // GL returns data starting from bottom. We have to reverse the order.
        let mut data = Vec::new();
        for chunk in dst.chunks(stride).rev() {
            data.extend(chunk);
        }

        Ok(Buffer::new(format, self.size.width, self.size.height, stride, data))
    }
}

// -------------------------------------------------------------------------------------------------

/// Drawing helpers.
impl RendererGl {
    /// Prepare view for drawing.
    fn prepare_view(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::UseProgram(self.program);
            gl::Uniform2i(self.loc_screen_size, self.size.width as i32, self.size.height as i32);
        }
    }

    /// Load textures and prepare vertices.
    fn load_texture_and_prepare_vertices(&self,
                                         viewer: &SurfaceViewer,
                                         context: &SurfaceContext,
                                         vertices: &mut [gl::types::GLfloat],
                                         texcoords: &mut [gl::types::GLfloat],
                                         index: usize) {
        if let Some(ref surface) = viewer.get_surface(context.id) {
            let mut size = None;
            if let DataSource::Shm(ref buffer) = surface.data_source {
                let format = {
                    match buffer.get_format() {
                        // NOTE: Mixing channels is intentional. In `PixelFormat` one reads it from
                        // right to left, and in `gl` from left to right.
                        PixelFormat::XBGR8888 => gl::RGBA,
                        PixelFormat::ABGR8888 => gl::RGBA,
                        PixelFormat::XRGB8888 => gl::BGRA,
                        PixelFormat::ARGB8888 => gl::BGRA,
                    }
                };

                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                    gl::BindTexture(gl::TEXTURE_2D, self.vbo_texture[index]);

                    gl::TexImage2D(gl::TEXTURE_2D, // target
                                   0, // level, 0 = no mipmap
                                   gl::RGBA as gl::types::GLint, // internal format
                                   buffer.get_width() as gl::types::GLint, // width
                                   buffer.get_height() as gl::types::GLint, // height
                                   0, // always 0 in OpenGL ES
                                   format, // format
                                   gl::UNSIGNED_BYTE, // type
                                   buffer.as_ptr() as *const _);
                }

                size = Some(buffer.get_size());
            } else if let DataSource::HwImage(ref image, ref attrs) = surface.data_source {
                if let Some(image_target_texture) = self.image_target_texture {
                    // TODO: Reimport images only when it is really needed.
                    // FIXME: Destroy images created here.
                    let img = egl_tools::create_image(self.egl.display, attrs);
                    if let Some(img) = img {
                        unsafe {
                            let target = gl::TEXTURE_2D;

                            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                            gl::BindTexture(target, self.vbo_texture[index]);

                            image_target_texture(target, img.as_raw());
                            size = Some(image.get_size());
                        }
                    }
                }
            } else if let DataSource::Dmabuf(ref image, ref attrs) = surface.data_source {
                if let Some(image_target_texture) = self.image_target_texture {
                    // TODO: Reimport images only when it is really needed.
                    // FIXME: Destroy images created here.
                    let img = egl_tools::import_dmabuf(self.egl.display, attrs);
                    if let Some(img) = img {
                        unsafe {
                            let target = gl::TEXTURE_2D;

                            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
                            gl::BindTexture(target, self.vbo_texture[index]);

                            image_target_texture(target, img.as_raw());
                            size = Some(image.get_size());
                        }
                    }
                }
            }

            if let Some(size) = size {
                let left = (context.pos.x - surface.offset.x) as gl::types::GLfloat;
                let top = (context.pos.y - surface.offset.y) as gl::types::GLfloat;
                let right = left + size.width as gl::types::GLfloat;
                let bottom = top + size.height as gl::types::GLfloat;

                vertices[0] = left;
                vertices[1] = top;
                vertices[2] = right;
                vertices[3] = top;
                vertices[4] = left;
                vertices[5] = bottom;
                vertices[6] = right;
                vertices[7] = top;
                vertices[8] = right;
                vertices[9] = bottom;
                vertices[10] = left;
                vertices[11] = bottom;

                texcoords[0] = 0.0;
                texcoords[1] = 0.0;
                texcoords[2] = 1.0;
                texcoords[3] = 0.0;
                texcoords[4] = 0.0;
                texcoords[5] = 1.0;
                texcoords[6] = 1.0;
                texcoords[7] = 0.0;
                texcoords[8] = 1.0;
                texcoords[9] = 1.0;
                texcoords[10] = 0.0;
                texcoords[11] = 1.0;
            } else {
                log_warn3!("Renderer: No buffer for surface {}", context.id);
            }
        } else {
            log_warn3!("Renderer: No info for surface {}", context.id);
        }
    }

    /// Draw surfaces.
    fn draw_surfaces(&self, surfaces: &Vec<SurfaceContext>, viewer: &SurfaceViewer) {
        if surfaces.len() == 0 {
            return;
        }

        // Prepare vertices positions and upload textures
        let vertices_len = 12 * surfaces.len();
        let vertices_size = vertices_len * std::mem::size_of::<gl::types::GLfloat>();
        let mut vertices = vec![0.0; vertices_len];
        let mut texcoords = vec![0.0; vertices_len];

        for i in 0..surfaces.len() {
            self.load_texture_and_prepare_vertices(viewer,
                                                   &surfaces[i],
                                                   &mut vertices[12 * i..12 * i + 12],
                                                   &mut texcoords[12 * i..12 * i + 12],
                                                   i);
        }

        unsafe {
            // Upload positions to vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_vertices);
            gl::EnableVertexAttribArray(self.loc_vertices as gl::types::GLuint);
            gl::VertexAttribPointer(self.loc_vertices as gl::types::GLuint,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    2 *
                                    std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLint,
                                    std::ptr::null());
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertices_size as isize,
                           vertices.as_ptr() as *const _,
                           gl::DYNAMIC_DRAW);

            // Upload positions to vertex buffer object
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_texcoords);
            gl::EnableVertexAttribArray(self.loc_texcoords as gl::types::GLuint);
            gl::VertexAttribPointer(self.loc_texcoords as gl::types::GLuint,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    2 *
                                    std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLint,
                                    std::ptr::null());
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertices_size as isize,
                           texcoords.as_ptr() as *const _,
                           gl::DYNAMIC_DRAW);

            // Redraw everything
            for i in 0..surfaces.len() as i32 {
                gl::Uniform1i(self.loc_texture, i);
                gl::DrawArrays(gl::TRIANGLES, 6 * i, 6);
            }

            // Release resources
            gl::DisableVertexAttribArray(self.loc_texcoords as gl::types::GLuint);
            gl::DisableVertexAttribArray(self.loc_vertices as gl::types::GLuint);
        }
    }

    /// Unbind framebuffer and program.
    fn release_view(&self) {
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
            gl::UseProgram(0);
        }
    }
}

// -------------------------------------------------------------------------------------------------
