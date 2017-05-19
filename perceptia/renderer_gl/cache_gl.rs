// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains functionality related to caching GL state.

use std::collections::HashMap;
use std::time::Instant;

use gl;

use qualia::{HwImage, SurfaceId};

// -------------------------------------------------------------------------------------------------

/// Helper structure for storing texture related data.
#[derive(Clone)]
pub struct TextureInfo {
    /// Texture ID.
    texture: gl::types::GLuint,

    /// Current hardware image
    image: Option<HwImage>,

    /// Time when texture was updated.
    time_stamp: Option<Instant>,
}

// -------------------------------------------------------------------------------------------------

impl TextureInfo {
    /// Constructs new `TextureInfo`.
    pub fn new(texture: gl::types::GLuint) -> Self {
        TextureInfo {
            texture: texture,
            image: None,
            time_stamp: None,
        }
    }

    /// Returns texture ID.
    #[inline]
    pub fn get_texture(&self) -> gl::types::GLuint {
        self.texture
    }

    /// Returns hardware image.
    #[inline]
    pub fn get_image(&self) -> Option<HwImage> {
        self.image.clone()
    }

    /// Updates time stamp of texture. Should be called whenever new data was loaded.
    #[inline]
    pub fn update(&mut self, image: Option<HwImage>) {
        self.time_stamp = Some(Instant::now());
        self.image = image;
    }

    /// Checks if texture was loaded before given instant in time.
    pub fn is_younger(&self, other_time_stamp: Instant) -> bool {
        if let Some(my_time_stamp) = self.time_stamp {
            my_time_stamp < other_time_stamp
        } else {
            true
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Stores texture and instant in time then it was updated. Thanks to that we do not have to load
/// textures every time screen refreshes but cache them here and reuse when needed.
pub struct CacheGl {
    /// Map from surface ID to texture info.
    textures: HashMap<SurfaceId, TextureInfo>,
}

// -------------------------------------------------------------------------------------------------

// Public methods
impl CacheGl {
    /// Constructs new `CacheGl`.
    pub fn new() -> Self {
        CacheGl { textures: HashMap::new() }
    }

    /// Tries to get texture for given surface. If surface was never drawn before creates new
    /// texture.
    pub fn get_or_generate_info(&mut self, sid: SurfaceId) -> TextureInfo {
        if let Some(texture) = self.get_texture_info(sid) {
            texture
        } else {
            let mut texture: gl::types::GLuint = 0;
            unsafe {
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            }
            self.insert_texture(sid, texture)
        }
    }

    /// Updates time stamp of texture and optionally stores image. Should be called whenever new
    /// data was loaded.
    pub fn update(&mut self, sid: SurfaceId, image: Option<HwImage>) {
        if let Some(ref mut info) = self.textures.get_mut(&sid) {
            info.update(image);
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Private methods
impl CacheGl {
    /// Returns texture info.
    fn get_texture_info(&self, sid: SurfaceId) -> Option<TextureInfo> {
        self.textures.get(&sid).cloned()
    }

    /// Inserts texture info.
    fn insert_texture(&mut self, sid: SurfaceId, texture: gl::types::GLuint) -> TextureInfo {
        let info = TextureInfo::new(texture);
        self.textures.insert(sid, info.clone());
        info
    }
}

// -------------------------------------------------------------------------------------------------
