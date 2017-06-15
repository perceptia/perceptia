// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

/// This module contains logic responsible for controlling off-screen non-device software-rendered
/// outputs created e.g. for remote desktop clients.

use qualia::{Buffer, VirtualOutputBundle, Illusion, OutputInfo, Position};
use qualia::{SurfaceContext, SurfaceViewer};
use renderer_pixmap::RendererPixmap;

use output::Output;

// -------------------------------------------------------------------------------------------------

/// Structure for off-screen non-device software-rendered outputs created e.g. for remote desktop
/// clients.
pub struct VirtualOutput {
    /// Virtual output bundle containing e.g. its size, position or framebuffer.
    bundle: VirtualOutputBundle,

    /// Id of the output. Guarantied to be unique in application.
    id: i32,

    /// Name of the output.
    name: String,

    /// Pixmap renderer.
    renderer: RendererPixmap,
}

// -------------------------------------------------------------------------------------------------

impl VirtualOutput {
    /// Constructs new `VirtualOutput`.
    pub fn new(bundle: VirtualOutputBundle, id: i32) -> Result<Box<Output>, Illusion> {
        Ok(Box::new(VirtualOutput {
            bundle: bundle,
            id: id,
            name: format!("virtual{}", id),
            renderer: RendererPixmap::new(),
        }))
    }
}

// -------------------------------------------------------------------------------------------------

// Public methods
impl Output for VirtualOutput {
    /// Draws passed scene using renderer.
    fn draw(&mut self,
            layunder: &Vec<SurfaceContext>,
            surfaces: &Vec<SurfaceContext>,
            layover: &Vec<SurfaceContext>,
            viewer: &SurfaceViewer)
            -> Result<(), Illusion> {
        let mut vfb = self.bundle.vfb.write().unwrap();
        self.renderer.draw(vfb.as_mut_slice().split_at_mut(self.bundle.offset).1,
                           self.bundle.area.size,
                           self.bundle.stride,
                           layunder,
                           surfaces,
                           layover,
                           viewer)
    }

    /// Takes screenshot. Returns `Buffer` containing image data.
    fn take_screenshot(&self) -> Result<Buffer, Illusion> {
        // TODO: Implement taking screenshots of virtual outputs.
        Err(Illusion::General(format!("Screenshot on virtual output is not supported yet")))
    }

    /// Returns info about output.
    fn get_info(&self) -> OutputInfo {
        OutputInfo::new(self.id,
                        self.bundle.area,
                        self.bundle.area.size,
                        0, // TODO: Estimate refresh rate of virtual outputs.
                        self.name.clone(),
                        self.name.clone())
    }

    /// Sets global position.
    fn set_position(&mut self, position: Position) {
        self.bundle.area.pos = position;
    }

    /// Swaps renderers and devices buffers.
    fn swap_buffers(&mut self) -> Result<u32, Illusion> {
        // No need to perform buffer swap. Virtual outputs use mutex instead of double buffer.
        Ok(0)
    }

    /// In hardware outputs this method schedules pageflip. Here we only subscribe for notification
    /// about reading the output data by its consumer (e.g. remote desktop client).
    fn schedule_pageflip(&self) -> Result<(), Illusion> {
        self.bundle.vfb.write().unwrap().subscribe_for_vblank(self.id);
        Ok(())
    }

    /// Reinitializes the output.
    fn recreate(&self) -> Result<Box<Output>, Illusion> {
        VirtualOutput::new(self.bundle.clone(), self.id)
    }
}

// -------------------------------------------------------------------------------------------------
