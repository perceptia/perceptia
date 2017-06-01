// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Creating simple panels.
//!
//! NOTE: Ideally it would be to have this functionality in external application.
//!
//! TODO: Make appearance of the panels configurable: fonts, size, colors, ...
//!
//! TODO: Add support for displaying date, time and workspace state.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

use font_loader::system_fonts::{self, FontPropertyBuilder};
use rusttype::{Font, FontCollection, PositionedGlyph, Scale, point};

use qualia::{Buffer, Image, OutputInfo, PixelFormat, Pixmap, Size};
use qualia::{MemoryPoolId, MemoryViewId, SurfaceId, AestheticsCoordinationTrait};

// -------------------------------------------------------------------------------------------------

/// Informations about buffer.
struct BufferInfo {
    buffer: Buffer,
    mpid: MemoryPoolId,
    mvid: MemoryViewId,
}

// -------------------------------------------------------------------------------------------------

impl BufferInfo {
    /// Constructs new `BufferInfo`.
    pub fn new(buffer: Buffer) -> Self {
        BufferInfo {
            buffer: buffer,
            mpid: MemoryPoolId::initial(),
            mvid: MemoryViewId::initial(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Instance of one panel shown on one display.
struct Panel<C>
    where C: AestheticsCoordinationTrait
{
    /// Coordinator.
    coordinator: C,

    /// ID of panels surface.
    sid: SurfaceId,

    /// Buffers for drawing panel.
    buffers: Vec<BufferInfo>,

    /// Index of buffer currently used for drawing next frame.
    draw_buffer: usize,

    /// Index of buffer currently used for displaying .
    front_buffer: usize,
}

// -------------------------------------------------------------------------------------------------

impl<C> Panel<C>
    where C: AestheticsCoordinationTrait
{
    /// Constructs new `Panel`.
    fn new(size: Size, display_id: i32, font: &Option<Font>, coordinator: C) -> Self {
        let mut mine = Panel {
            coordinator: coordinator,
            sid: SurfaceId::invalid(),
            buffers: Vec::new(),
            draw_buffer: 0,
            front_buffer: 0,
        };

        mine.add_buffer(size);
        mine.add_buffer(size);
        mine.draw(font);
        mine.dock(size, display_id);
        mine.swap_buffers();
        mine
    }

    /// Add next buffer.
    fn add_buffer(&mut self, size: Size) {
        let w = size.width;
        let h = size.height;
        let format = PixelFormat::ABGR8888;
        let pixel_size = format.get_size();
        let stride = w * pixel_size;
        let data = vec![150; stride * h];
        self.draw_buffer = self.buffers.len();
        self.buffers.push(BufferInfo::new(Buffer::new(format, w, h, stride, data)));
    }

    /// Swaps buffers locally and in coordinator.
    fn swap_buffers(&mut self) {
        // Update buffer indices.
        self.front_buffer = self.draw_buffer;
        self.draw_buffer = (self.draw_buffer + 1) % self.buffers.len();

        // Tell the `Coordinator` which buffer use for drawing.
        self.coordinator.attach_shm(self.buffers[self.front_buffer].mvid, self.sid);
        self.coordinator.commit_surface(self.sid);
    }

    /// Requests showing the panel as docked on given display.
    fn dock(&mut self, size: Size, display_id: i32) {
        // Give the `Coordinator` the buffers later used for drawing panel.
        for ref mut info in self.buffers.iter_mut() {
            info.mpid = self.coordinator.create_pool_from_buffer(info.buffer.clone());
            if let Some(mvid) = self.coordinator.create_memory_view(info.mpid,
                                                                    info.buffer.get_format(),
                                                                    0,
                                                                    info.buffer.get_width(),
                                                                    info.buffer.get_height(),
                                                                    info.buffer.get_stride()) {
                info.mvid = mvid;
            }
        }

        // Create surface and request showing it as docked.
        self.sid = self.coordinator.create_surface();
        self.coordinator.dock_surface(self.sid, size, display_id);
    }
}

// -------------------------------------------------------------------------------------------------

// Methods related to drawing the panel.
impl<C> Panel<C>
    where C: AestheticsCoordinationTrait
{
    /// Redraws entire panel.
    fn draw(&mut self, font: &Option<Font>) {
        let buffer = &mut self.buffers[self.draw_buffer].buffer;
        let width = buffer.get_width();
        let height = buffer.get_height();
        let mut data = buffer.as_mut_slice();

        // Paint the background
        for y in 0..height {
            for x in 0..width {
                data[0 + 4 * (x + y * width)] = 200;
                data[1 + 4 * (x + y * width)] = 200;
                data[2 + 4 * (x + y * width)] = 200;
                data[3 + 4 * (x + y * width)] = 150;
            }
        }

        // Paint the text
        if let Some(ref font) = *font {
            let scale = Scale { x: 12.0, y: 12.0 };
            let v_metrics = font.v_metrics(scale);
            let offset = point(0.0, v_metrics.ascent);
            let glyphs: Vec<PositionedGlyph> = font.layout("Perceptia", scale, offset).collect();

            for g in glyphs {
                if let Some(bb) = g.pixel_bounding_box() {
                    g.draw(|x, y, v| {
                        let x = (x as i32 + bb.min.x + 5) as usize;
                        let y = (y as i32 + bb.min.y + 2) as usize;
                        if x < width && y < height {
                            let value = 200 - (200.0 * v) as u8;
                            let alpha = 150 + (105.0 * v) as u8;
                            data[0 + 4 * (x + y * width)] = value;
                            data[1 + 4 * (x + y * width)] = value;
                            data[2 + 4 * (x + y * width)] = value;
                            data[3 + 4 * (x + y * width)] = alpha;
                        }
                    })
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Creates and manages all the panels.
pub struct PanelManager<'a, C>
    where C: AestheticsCoordinationTrait
{
    /// Coordinator.
    coordinator: C,

    /// Collection of all panels.
    panels: HashMap<i32, Panel<C>>,

    /// Font used to draw text.
    font: Option<Font<'a>>,
}

// -------------------------------------------------------------------------------------------------

impl<'a, C> PanelManager<'a, C>
    where C: AestheticsCoordinationTrait
{
    /// Constructs new `PanelManager`.
    pub fn new(coordinator: C) -> Self {
        PanelManager {
            coordinator: coordinator,
            panels: HashMap::new(),
            font: Self::load_font(),
        }
    }

    /// Loads font from file.
    pub fn load_font() -> Option<Font<'a>> {
        let builder = FontPropertyBuilder::new();
        let property = builder.family("Inconsolata").bold().monospace().build();
        if let Some((bytes, _)) = system_fonts::get(&property) {
            let collection = FontCollection::from_bytes(bytes);
            match collection.into_font() {
                Some(font) => {
                    Some(font)
                }
                None => {
                    log_warn1!("Failed create font");
                    None
                }
            }
        } else {
            log_error!("Failed to find font");
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Handling signals
impl<'a, C> PanelManager<'a, C>
    where C: AestheticsCoordinationTrait + Clone
{
    /// Handles creation of display by creating new panel.
    ///
    /// FIXME: Handle destruction of display to destroy panel.
    pub fn on_display_created(&mut self, output: &OutputInfo) {
        let panel = Panel::new(Size::new(output.area.size.width, 16),
                               output.id,
                               &self.font,
                               self.coordinator.clone());
        self.panels.insert(output.id, panel);
    }
}

// -------------------------------------------------------------------------------------------------
