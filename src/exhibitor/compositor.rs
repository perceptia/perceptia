// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Compositor is manager of surfaces. Cares about placing and manipulating them according to
//! user-defined strategies.

// -------------------------------------------------------------------------------------------------

use timber;
use qualia::{Coordinator, Area, SurfaceId, SurfaceInfo};

use surface_history::SurfaceHistory;
use frames::{self, Frame};
use frames::searching::Searching;
use frames::settling::Settling;

// -------------------------------------------------------------------------------------------------

macro_rules! try_get_surface {
    ($compositor:expr, $sid:ident) => {
        match $compositor.coordinator.get_surface($sid) {
            Some(surface) => surface,
            None => {
                log_warn2!("Surface {} not found!", $sid);
                return
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure describing strategic decision about how to handle new surface.
struct ManageDecision {
    /// Target frame where new surface should be settled.
    target: Frame,

    /// Geometry of new frame.
    geometry: frames::Geometry,

    /// True if new frame should be selected. False otherwise.
    selection: bool,
}

// -------------------------------------------------------------------------------------------------

/// Compositor main structure.
pub struct Compositor {
    history: SurfaceHistory,
    coordinator: Coordinator,
    root: Frame,
    selection: Frame,
}

// -------------------------------------------------------------------------------------------------

impl Compositor {
    /// `Compositor` constructor.
    pub fn new(coordinator: Coordinator) -> Self {
        let root = Frame::new_root();
        Compositor {
            history: SurfaceHistory::new(),
            coordinator: coordinator,
            root: root.clone(),
            selection: root,
        }
    }

    /// Creates new display with default workspace.
    pub fn create_display(&mut self, area: Area, name: String) -> Frame {
        let mut display = Frame::new_display(area, name);
        let mut workspace = Frame::new_workspace();
        self.root.append(&mut display);
        workspace.settle(&mut display, &mut self.coordinator);
        self.select(workspace);
        display
    }

    /// Handles new surface by settling it in frame tree, adding to history and notifying
    /// coordinator.
    pub fn manage_surface(&mut self, sid: SurfaceId) {
        // Get surface
        let surface = try_get_surface!(self, sid);

        // Consult about placement strategy
        let mut decision = self.choose_target(&surface);

        // Settle and optionally select new frame
        let mut frame = Frame::new_leaf(sid, decision.geometry);
        frame.settle(&mut decision.target, &mut self.coordinator);
        if decision.selection {
            self.select(frame);
        }

        // Finalize
        self.history.add(sid);
        self.coordinator.notify();
        self.log_frames();
    }

    /// Handles destruction of surface. Removes it from history and frame free.
    pub fn unmanage_surface(&mut self, sid: SurfaceId) {
        if let Some(ref mut frame) = self.root.find_with_sid(sid) {
            self.history.remove(sid);
            if frame.get_sid() == self.selection.get_sid() {
                // TODO: This should be configurable
                let new_selection = self.selection.find_buildable().unwrap();
                self.select(new_selection);
            }

            frame.remove();
            self.coordinator.notify();
            self.log_frames();
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Private methods
impl Compositor {
    /// Set given frame as selected.
    fn select(&mut self, frame: Frame) {
        self.selection = frame;
        if self.selection.get_sid().is_valid() {
            self.coordinator.set_focus(self.selection.get_sid());
        }
    }

    /// Get selected frame.
    fn get_selection(&self) -> Frame {
        self.selection.clone()
    }

    /// Decide how to handle new surface.
    fn choose_target(&self, surface: &SurfaceInfo) -> ManageDecision {
        if surface.parent_sid.is_valid() {
            // FIXME: Choosing surface target should be configurable.
            ManageDecision {
                target: self.get_selection().find_buildable().unwrap(),
                geometry: frames::Geometry::Stacked,
                selection: true,
            }
        } else {
            ManageDecision {
                target: self.get_selection().find_top().unwrap(),
                geometry: frames::Geometry::Vertical,
                selection: true,
            }
        }
    }

    /// Print frame layout for log file.
    fn log_frames(&self) {
        let mut timber = timber::lock().unwrap();
        timber.log(format_args!("===============================================\
                                 ===============================================\n"));
        self.log_frames_helper(&self.root, 0, &mut timber);
        timber.log(format_args!("===============================================\
                                 ===============================================\n"));
    }

    /// Helper for frame layout printer.
    fn log_frames_helper(&self, frame: &Frame, depth: i32, timber: &mut timber::Timber) {
        for ref subframe in frame.space_iter() {
            for _ in 0..depth {
                timber.log(format_args!("\t"));
            }
            timber.log(format_args!("{:?}\n", subframe));
            self.log_frames_helper(subframe, depth + 1, timber);
        }
    }
}

// -------------------------------------------------------------------------------------------------
