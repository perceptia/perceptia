// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extra settling functionality for `frames::Frame`.

// -------------------------------------------------------------------------------------------------

use qualia::{Area, Position, Vector, Size, SurfaceAccess, SurfaceId};

use frame::{Frame, Geometry, Mobility, Mode, Side};
use searching::Searching;
use packing::Packing;

// -------------------------------------------------------------------------------------------------

/// Extension trait for `Frame` adding more settling functionality.
pub trait Settling {
    /// Settle self in buildable of target and relax it.
    ///
    /// If `area` is provided settle the surface as floating with given position and size.
    fn settle(&mut self, target: &mut Frame, area: Option<Area>, sa: &mut SurfaceAccess);

    /// Remove given frame, relax old parent and settle the frame on given target.
    fn resettle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Pop the surface `pop` and its parents inside surface `self`.
    ///
    /// After calling this function `pop` will be most recently used frame inside `self`.
    fn pop_recursively(&mut self, pop: &mut Frame);

    /// Changes frames geometry and resizes all subframe accordingly.
    fn change_geometry(&mut self, geometry: Geometry, sa: &mut SurfaceAccess);

    /// Adds another container into given place in frame layout if needed.
    ///
    /// This method is used when jumping into leaf frame to create container to handle the leaf
    /// and jumped frame.
    ///
    /// Returns
    ///  - `self` if it is container with one child,
    ///  - parent if parent has one child
    ///  - newly created container frame otherwise
    ///
    /// Returned frame is guarantied to have exactly one child.
    fn ramify(&mut self, geometry: Geometry) -> Frame;

    /// Removes unnecessary layers of container frames containing only one container or leaf frame.
    fn deramify(&mut self);

    /// Places frame `self` on given `side` of `target` frame.
    fn jumpin(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Removes frame `self` from frame layout and then places it using `jumpin` method.
    fn jump(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess);

    /// Places frame `self` in `target` frame as dock.
    fn dock(&mut self, target: &mut Frame, size: Size, sa: &mut SurfaceAccess);

    /// Anchorizes floating frame.
    fn anchorize(&mut self, sa: &mut SurfaceAccess);

    /// Deanchorizes frame. Floating frame must be attached to workspace so it will be resettled if
    /// necessary.
    fn deanchorize(&mut self, area: Area, sa: &mut SurfaceAccess);

    /// Set new position for given frame and move it subframes accordingly.
    fn set_position(&mut self, pos: Position);

    /// Move the frame and all subframes by given vector.
    fn move_with_contents(&mut self, vector: Vector);

    /// Removes frame `self`, relaxes old parent and destroys the frame.
    fn destroy_self(&mut self, sa: &mut SurfaceAccess);
}

// -------------------------------------------------------------------------------------------------

impl Settling for Frame {
    fn settle(&mut self, target: &mut Frame, area: Option<Area>, sa: &mut SurfaceAccess) {
        if let Some(ref mut buildable) = target.find_buildable() {
            if buildable.get_geometry() == Geometry::Stacked {
                buildable.prepend(self);
                if let Some(area) = area {
                    self.set_plumbing_mobility(Mobility::Floating);
                    self.set_size(area.size, sa);
                    self.set_position(area.pos);
                } else {
                    self.set_plumbing_mobility(Mobility::Anchored);
                }
            } else {
                buildable.append(self);
                self.set_plumbing_mobility(Mobility::Anchored);
            }
            buildable.relax(sa);
        }
    }

    fn resettle(&mut self, target: &mut Frame, sa: &mut SurfaceAccess) {
        let area = {
            // Preserve area if resettling to another workspace
            if self.get_mobility().is_floating() && target.get_mode().is_workspace() {
                Some(self.get_area())
            } else {
                None
            }
        };
        self.remove_self(sa);
        self.settle(target, area, sa);
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
        if !self.is_top() {
            let parent = self.get_parent().expect("should have parent");
            if self.count_children() == 1 {
                return self.clone();
            }
            if parent.count_children() == 1 {
                return parent;
            }
        }

        let (distancer_mobility, distancer_mode) = if self.is_top() {
            (self.get_mobility(), self.get_mode())
        } else {
            (Mobility::Anchored, Mode::Container)
        };

        let frame_mode = if self.get_mode().is_leaf() {
            self.get_mode()
        } else {
            Mode::Container
        };

        let mut distancer = Frame::new(SurfaceId::invalid(),
                                       geometry,
                                       distancer_mobility,
                                       distancer_mode,
                                       self.get_position(),
                                       self.get_size(),
                                       self.get_title());
        self.prejoin(&mut distancer);
        self.remove();
        self.set_plumbing_mobility(Mobility::Anchored);
        self.set_plumbing_mode(frame_mode);
        let opposite = self.get_position().opposite();
        self.move_with_contents(opposite);
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
                self.set_plumbing_mode(first.get_mode());
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
                    let mut new_target = {
                        if !target_parent.is_top() &&
                           target_parent.count_children() == 1 {
                            target_parent.clone()
                        } else if target.get_mode().is_leaf() {
                            target.ramify(Geometry::Stacked)
                        } else {
                            target.clone()
                        }
                    };

                    self.settle(&mut new_target, None, sa);
                }
            }
        }
    }

    fn jump(&mut self, side: Side, target: &mut Frame, sa: &mut SurfaceAccess) {
        self.remove_self(sa);
        self.jumpin(side, target, sa);
    }

    fn dock(&mut self, target: &mut Frame, size: Size, sa: &mut SurfaceAccess) {
        target.set_plumbing_geometry(Geometry::Vertical);
        self.set_plumbing_mobility(Mobility::Docked);
        self.set_plumbing_size(size);
        self.set_plumbing_position(Position::default());
        target.prepend(self);
        target.relax(sa);
    }

    fn anchorize(&mut self, sa: &mut SurfaceAccess) {
        if self.is_reanchorizable() && self.get_mobility().is_floating() {
            // NOTE: Floating surface must be direct child of workspace.
            let parent = self.get_parent().expect("should have parent");
            self.set_size(parent.get_size(), sa);
            self.set_position(Position::default());
            self.set_plumbing_mobility(Mobility::Anchored);
        }
    }

    fn deanchorize(&mut self, area: Area, sa: &mut SurfaceAccess) {
        if self.is_reanchorizable() && self.get_mobility().is_anchored() {
            let mut workspace = self.find_top().expect("should have toplevel");
            let parent = self.get_parent().expect("should have parent");
            if !parent.equals_exact(&workspace) {
                self.remove_self(sa);
                workspace.prepend(self);
            }
            self.set_size(area.size, sa);
            self.set_position(area.pos);
            self.set_plumbing_mobility(Mobility::Floating);
        }
    }

    fn set_position(&mut self, pos: Position) {
        let vector = pos - self.get_position();
        self.move_with_contents(vector);
    }

    fn move_with_contents(&mut self, vector: Vector) {
        // Update frames position
        let new_position = self.get_position() + vector.clone();
        self.set_plumbing_position(new_position);
    }

    fn destroy_self(&mut self, sa: &mut SurfaceAccess) {
        self.remove_self(sa);
        self.destroy();
    }
}

// -------------------------------------------------------------------------------------------------
