// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra settling functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::{SurfaceAccess, SurfaceId};

use frame::{Frame, Geometry, Mode, Side};
use searching::Searching;
use packing::Packing;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more settling functionality.
pub trait Settling {
    /// Settle self in buildable of target and relax it.
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Remove given frame, relax old parent and settle the frame on given target.
    fn resettle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Pop the surface `pop` and its parents inside surface `self`.
    ///
    /// After calling this function `pop` will be most recently used frame inside `self`.
    fn pop_recursively(&mut self, pop: &mut Frame);

    /// Changes frames geometry and resizes all subframe accordingly.
    fn change_geometry(&mut self, geometry: Geometry, sa: &mut SurfaceAccess);

    /// Adds another container into given place in frame layout.
    ///
    /// This method is used when jumping into leaf frame to create container to handle the leaf
    /// and jumped frame.
    ///
    /// Returns newly created container frame.
    fn ramify(&mut self, geometry: Geometry) -> Frame;

    /// Removes unnecessary layers of container frames containing only one container or leaf frame.
    fn deramify(&mut self);

    /// Places frame `self` on given `side` of `target` frame.
    fn jumpin(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Removes frame `self` from frame layout and then places it using `jumpin` method.
    fn jump(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Removes frame `self`, relaxes old parent and destroys the frame.
    fn destroy_self(&mut self, sa: &mut SurfaceAccess);
}

// -------------------------------------------------------------------------------------------------

impl Settling for Frame {
    fn settle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess) {
        if let Some(ref mut buildable) = target.find_buildable() {
            buildable.append(self);
            buildable.relax(sa);
        }
    }

    fn resettle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess) {
        self.remove_self(sa);
        self.settle(target, sa);
    }

    fn pop_recursively(&mut self, pop: &mut Frame) {
        // If we reached `self` we can finish
        if self.equals_exact(pop) {
            return;
        }

        // If there's nothing above we can finish
        if let Some(ref mut parent) = pop.get_parent() {
            // If it is `stacked` frame we have to pop it also spatially
            if parent.get_geometry() == Geometry::Stacked {
                pop.remove();
                parent.prepend(pop);
            }

            // Pop in temporal order
            pop.pop();

            // Do the same recursively on trunk
            self.pop_recursively(parent);
        }
    }

    fn change_geometry(&mut self, geometry: Geometry, sa: &mut SurfaceAccess) {
        self.set_plumbing_geometry(geometry);
        self.homogenize(sa);
    }

    fn ramify(&mut self, geometry: Geometry) -> Frame {
        let distancer_mode = if self.get_mode().is_top() {
            self.get_mode()
        } else {
            Mode::Container
        };

        let frame_mode = if self.get_mode() == Mode::Leaf {
            self.get_mode()
        } else {
            Mode::Container
        };

        let mut distancer = Frame::new(SurfaceId::invalid(),
                                       distancer_mode,
                                       geometry,
                                       self.get_position(),
                                       self.get_size(),
                                       self.get_title());
        self.prejoin(&mut distancer);
        self.remove();
        self.set_plumbing_mode(frame_mode);
        distancer.prepend(self);
        distancer
    }

    fn deramify(&mut self) {
        let len = self.count_children();
        if len == 1 {
            let mut first = self.get_first_time().expect("should have exactly one child");
            let len = first.count_children();
            if len == 1 {
                let mut second = first.get_first_time().expect("should have exactly one child");
                first.remove();
                second.remove();
                self.prepend(&mut second);
                first.destroy();
            } else if len == 0 {
                self.set_plumbing_mode(Mode::Leaf);
                self.set_plumbing_sid(first.get_sid());
                first.remove();
                first.destroy();
            }
        }
    }

    fn jumpin(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess) {
        if let Some(mut target_parent) = target.get_parent() {
            match side {
                Side::Before => {
                    target.prejoin(self);
                    target_parent.relax(sa);
                }
                Side::After => {
                    target.adjoin(self);
                    target_parent.relax(sa);
                }
                Side::On => {
                    let mut new_target = if target_parent.count_children() == 1 {
                        target_parent.clone()
                    } else if target.get_mode() == Mode::Leaf {
                        target.ramify(Geometry::Stacked)
                    } else {
                        target.clone()
                    };

                    self.settle(&mut new_target, sa);
                }
            }
        }
    }

    fn jump(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess) {
        self.remove_self(sa);
        self.jumpin(side, target, sa);
    }

    fn destroy_self(&mut self, sa: &mut SurfaceAccess) {
        self.remove_self(sa);
        self.destroy();
    }
}

// -------------------------------------------------------------------------------------------------
