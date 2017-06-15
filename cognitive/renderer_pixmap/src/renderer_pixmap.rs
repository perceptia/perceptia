// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains pixmap renderer which allows drawing frame scenes with using sorftware
//! rendering.

use qualia::{Area, DataSource, Image, Pixmap, Position, Size};
use qualia::{Illusion, SurfaceContext, SurfaceViewer};

// -------------------------------------------------------------------------------------------------

/// Pixmap renderer.
pub struct RendererPixmap {}

// -------------------------------------------------------------------------------------------------

impl RendererPixmap {
    /// Constructs new `RendererPixmap`.
    pub fn new() -> Self {
        RendererPixmap {}
    }

    /// Draws passed frame scene.
    pub fn draw(&mut self,
                data: &mut[u8],
                size: Size,
                stride: usize,
                layunder: &[SurfaceContext],
                surfaces: &[SurfaceContext],
                layover: &[SurfaceContext],
                viewer: &SurfaceViewer)
                -> Result<(), Illusion> {
        self.draw_surfaces(data, size, stride, layunder, viewer);
        self.draw_surfaces(data, size, stride, surfaces, viewer);
        self.draw_surfaces(data, size, stride, layover, viewer);
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

impl RendererPixmap {
    /// Helper method for drawing surfaces.
    fn draw_surfaces(&mut self,
                     data: &mut[u8],
                     size: Size,
                     stride: usize,
                     surfaces: &[SurfaceContext],
                     viewer: &SurfaceViewer) {
        for context in surfaces {
            if let Some(ref surface) = viewer.get_surface(context.id) {
                if let DataSource::Shm { ref source, time_stamp: _ } = surface.data_source {
                    let target_area = Area::new(Position::new(0, 0), size);
                    let source_area = Area::new(context.pos - surface.offset, source.get_size());
                    self.draw_surface(data,
                                      target_area,
                                      stride,
                                      source.as_slice(),
                                      source_area,
                                      source.get_stride());
                }
            }
        }
    }

    /// Helper method for drawing one surface.
    fn draw_surface(&mut self,
                    target: &mut[u8],
                    target_area: Area,
                    target_stride: usize,
                    source: &[u8],
                    source_area: Area,
                    source_stride: usize) {
        if let Some(inter) = target_area.intersected(&source_area) {
            let source_offset = (4 * source_area.pos.x) +
                                (source_stride as isize * source_area.pos.y);

            for y in inter.pos.y .. (inter.pos.y + inter.size.height as isize) {
                for x in inter.pos.x .. (inter.pos.x + inter.size.width as isize) {
                    let p1 = (4 * x as usize) + (target_stride * y as usize);
                    let p2 = ((4 * x) + (source_stride as isize * y) - source_offset) as usize;
                    let a = source[p2 + 3] as f32 / 255.0;
                    target[p1 + 0] = (((1.0 - a) * target[p1 + 0] as f32) +
                                     (a * source[p2 + 0] as f32)) as u8;
                    target[p1 + 1] = (((1.0 - a) * target[p1 + 1] as f32) +
                                     (a * source[p2 + 1] as f32)) as u8;
                    target[p1 + 2] = (((1.0 - a) * target[p1 + 2] as f32) +
                                     (a * source[p2 + 2] as f32)) as u8;
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
