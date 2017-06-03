// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra settling functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::{Position, Size, Vector};
use qualia::{SurfaceAccess, surface_state};

use frame::{Frame, Geometry, Mobility};
use settling::Settling;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more packing functionality.
pub trait Packing {
    /// TODO: Implement relaxing. Currently relaxing is equivalent to homogenizing.
    fn relax(&mut self, sa: &mut SurfaceAccess);

    /// Make all subsurfaces have the same size and proper layout.
    /// Homogenizing works only on directed frames.
    fn homogenize(&mut self, sa: &mut SurfaceAccess);

    /// Set size of the frame and resize its subframe accordingly.
    fn set_size(&mut self, size: Size, sa: &mut SurfaceAccess);

    /// Remove given frame and relax old parent.
    fn remove_self(&mut self, sa: &mut SurfaceAccess);
}

// -------------------------------------------------------------------------------------------------

impl Packing for Frame {
    fn relax(&mut self, sa: &mut SurfaceAccess) {
        self.homogenize(sa);
    }

    fn homogenize(&mut self, sa: &mut SurfaceAccess) {
        let len = self.count_anchored_children();
        if len < 1 {
            return;
        }

        // Decide how to resize and move twigs
        let mut size = Size::new(0, 0);
        let mut increment = Vector::new(0, 0);
        match self.get_geometry() {
            Geometry::Stacked => {
                size = self.get_size();
            }
            Geometry::Vertical => {
                let mut docked_height = 0;
                for frame in self.space_iter() {
                    if frame.get_mobility().is_docked() {
                        docked_height += frame.get_size().height;
                    }
                }
                size.width = self.get_size().width;
                size.height = (self.get_size().height - docked_height) / len;
                increment.y = size.height as isize;
            }
            Geometry::Horizontal => {
                let mut docked_width = 0;
                for frame in self.space_iter() {
                    if frame.get_mobility().is_docked() {
                        docked_width += frame.get_size().width;
                    }
                }
                size.height = self.get_size().height;
                size.width = (self.get_size().width - docked_width) / len;
                increment.x = size.width as isize;
            }
        }

        // Resize and reposition all subframes recursively
        let mut pos = Position::default();
        for mut frame in self.space_iter() {
            match frame.get_mobility() {
                Mobility::Anchored => {
                    frame.set_size(size, sa);
                    frame.set_position(pos);
                    pos = pos + increment;
                }
                Mobility::Docked => {
                    match self.get_geometry() {
                        Geometry::Stacked => {}
                        Geometry::Vertical => pos.y += frame.get_size().height as isize,
                        Geometry::Horizontal => pos.x += frame.get_size().width as isize,
                    }
                }
                Mobility::Floating => {}
            }
        }
    }

    fn set_size(&mut self, size: Size, sa: &mut SurfaceAccess) {
        // Set size for given frame.
        let old_size = self.get_size();
        self.set_plumbing_size(size.clone());
        sa.reconfigure(self.get_sid(), size.clone(), surface_state::MAXIMIZED);

        // Set size to frames children.
        match self.get_geometry() {
            Geometry::Horizontal => {
                if old_size.width == size.width {
                    for mut frame in self.space_iter() {
                        let mut frame_size = frame.get_size();
                        frame_size.height = size.height;
                        frame.set_size(frame_size, sa);
                    }
                } else {
                    self.relax(sa);
                }
            }
            Geometry::Vertical => {
                if old_size.height == size.height {
                    for mut frame in self.space_iter() {
                        let mut frame_size = frame.get_size();
                        frame_size.width = size.width;
                        frame.set_size(frame_size, sa);
                    }
                } else {
                    self.relax(sa);
                }
            }
            Geometry::Stacked => {
                for mut frame in self.space_iter() {
                    if !frame.get_mobility().is_floating() {
                        frame.set_size(size.clone(), sa);
                    }
                }
            }
        }
    }

    fn remove_self(&mut self, sa: &mut SurfaceAccess) {
        if let Some(ref mut parent) = self.get_parent() {
            self.remove();
            let len = parent.count_children();
            if len == 0 && !parent.is_top() {
                parent.remove_self(sa);
            } else {
                parent.relax(sa);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
