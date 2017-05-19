// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Compositor is manager of surfaces. Cares about placing and manipulating them according to
//! user-defined strategies.

// -------------------------------------------------------------------------------------------------

use std;

use timber;
use qualia::{Action, Area, Command, Direction, Size, Vector};
use qualia::{SurfaceId, CompositorConfig, ExhibitorCoordinationTrait};

use surface_history::SurfaceHistory;
use frames::{Frame, Geometry, Mode, Side};
use frames::searching::Searching;
use frames::settling::Settling;

use strategist::Strategist;

// -------------------------------------------------------------------------------------------------

const MAX_WORKSPACES: u32 = 1000;

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

/// Result of executing command.
enum CommandResult {
    /// Everything went OK.
    Ok,

    /// Command not handled most probably because it is not yet implemented.
    NotHandled,

    /// Wrong frame was used for operation. This indicates error in compositor logic.
    WrongFrame,
}

// -------------------------------------------------------------------------------------------------

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CommandResult::Ok => write!(f, "ok"),
            CommandResult::NotHandled => write!(f, "not handled"),
            CommandResult::WrongFrame => write!(f, "wrong frame"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Compositor main structure.
pub struct Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    history: SurfaceHistory,
    coordinator: C,
    root: Frame,
    selection: Frame,
    strategist: Strategist,
    config: CompositorConfig,
}

// -------------------------------------------------------------------------------------------------

/// Public methods.
impl<C> Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    /// `Compositor` constructor.
    pub fn new(coordinator: C, strategist: Strategist, config: CompositorConfig) -> Self {
        let root = Frame::new_root();
        Compositor {
            history: SurfaceHistory::new(),
            coordinator: coordinator,
            root: root.clone(),
            selection: root,
            strategist: strategist,
            config: config,
        }
    }

    /// Creates new display with default workspace.
    pub fn create_display(&mut self, area: Area, name: String) -> Frame {
        let mut display = Frame::new_display(area, name);
        let mut workspace = self.create_next_workspace()
            .expect("Could not create workspace. This probably indicates compositor logic error");
        self.root.append(&mut display);
        workspace.settle(&mut display, None, &mut self.coordinator);
        self.select(workspace);
        display
    }

    /// Executes given command.
    pub fn execute_command(&mut self, command: Command) {
        // Execute command
        let mut frame = self.selection.clone();
        let result = match command.action {
            Action::Configure => self.configure(&mut frame, command.direction),
            Action::Focus => {
                match command.direction {
                    Direction::Workspace => {
                        self.focus_workspace(&command.string);
                        CommandResult::Ok
                    }
                    _ => self.focus(&mut frame, command.direction, command.magnitude),
                }
            }
            Action::Jump => {
                match command.direction {
                    Direction::Workspace => {
                        self.jump_to_workspace(&mut frame, &command.string);
                        CommandResult::Ok
                    }
                    Direction::End => {
                        self.ramify(frame);
                        CommandResult::Ok
                    }
                    Direction::Begin => {
                        self.exalt(&mut frame);
                        CommandResult::Ok
                    }
                    _ => self.jump(&mut frame, command.direction, command.magnitude),
                }
            }
            Action::Dive => {
                match command.direction {
                    Direction::Workspace => {
                        self.dive_to_workspace(frame, &command.string);
                        CommandResult::Ok
                    }
                    Direction::Begin => {
                        self.exalt(&mut frame);
                        CommandResult::Ok
                    }
                    _ => self.dive(&mut frame, command.direction, command.magnitude),
                }
            }
            Action::Move => self.move_frame(&mut frame, command.direction, command.magnitude),
            Action::Anchor => self.anchorize(frame),
            _ => CommandResult::NotHandled,
        };

        // Check result and print appropriate log
        match result {
            CommandResult::Ok => {
                self.coordinator.notify();
                self.log_frames();
            }
            _ => log_error!("Command failed: {} ({:?})", result, command),
        }
    }

    /// Handles new surface by settling it in frame tree, adding to history and notifying
    /// coordinator.
    pub fn manage_surface(&mut self, sid: SurfaceId) {
        if self.root.find_with_sid(sid).is_none() {
            // Get surface
            let surface = try_get_surface!(self, sid);

            // Consult about placement strategy
            let mut decision = self.strategist.choose_target(&self.get_selection(), &surface);
            let area = if let Some(floating) = decision.floating {
                Some(floating.area)
            } else {
                None
            };

            // Settle and optionally select new frame
            let mut frame = Frame::new_leaf(sid, decision.geometry);
            frame.settle(&mut decision.target, area, &mut self.coordinator);
            if decision.selection {
                self.select(frame);
            }

            // Finalize
            self.history.add(sid);
            self.coordinator.notify();
            self.log_frames();
        }
    }

    /// Dock given surface.
    pub fn dock_surface(&mut self, sid: SurfaceId, size: Size, mut display_frame: Frame) -> Frame {
        if self.root.find_with_sid(sid).is_none() {
            // Dock the surface
            let mut dock = Frame::new_leaf(sid, Geometry::Stacked);
            let mut new_display_frame = display_frame.ramify(Geometry::Vertical);
            dock.dock(&mut new_display_frame, size, &mut self.coordinator);

            // Finalize
            self.coordinator.notify();
            self.log_frames();

            // Display frame might changed - return the new one
            new_display_frame
        } else {
            display_frame
        }
    }

    /// Handles destruction of surface. Removes it from history and frame free.
    pub fn unmanage_surface(&mut self, sid: SurfaceId) {
        if let Some(ref mut frame) = self.root.find_with_sid(sid) {
            self.history.remove(sid);
            if self.selection.get_sid() == sid {
                let new_selection = {
                    if let Some(previous_sid) = self.history.get_nth(0) {
                        self.root.find_with_sid(previous_sid).expect("Find previous frame")
                    } else {
                        self.selection.find_buildable().expect("Find buildable")
                    }
                };
                self.select(new_selection);
            }

            frame.destroy_self(&mut self.coordinator);
            self.coordinator.notify();
            self.log_frames();
        }
    }

    /// Pop given surface in history.
    pub fn pop_surface(&mut self, sid: SurfaceId) {
        if sid.is_valid() {
            if let Some(mut frame) = self.root.find_with_sid(sid) {
                // Pop in frame hierarchy.
                self.root.pop_recursively(&mut frame);

                // Update selection.
                self.select(frame);
            }

            // Pop in history.
            self.history.pop(sid);
        }
    }

    /// Returns root frame.
    pub fn get_root(&self) -> Frame {
        self.root.clone()
    }

    /// Returns selected frame.
    pub fn get_selection(&self) -> Frame {
        self.selection.clone()
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods related to handling commands.
impl<C> Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    /// Reconfigure frame to have different geometry.
    ///
    /// Only `Container`, `Leaf` or `Workspace` can be reconfigured (from this follows that
    /// reconfigured frame must have parent.)
    ///
    /// For convenience if target is `Leaf` its parent is reconfigured.
    fn configure(&mut self, frame: &mut Frame, direction: Direction) -> CommandResult {
        // Check validity of frame
        if !frame.is_reorientable() {
            log_warn1!("Can not change geometry of this frame: {:?}", frame);
            return CommandResult::WrongFrame;
        }

        let mut parent = frame.get_parent().expect("reconfigured frame should have parent");

        // Choose geometry
        let geometry = match direction {
            Direction::North | Direction::South => Geometry::Vertical,
            Direction::East | Direction::West => Geometry::Horizontal,
            Direction::Begin | Direction::End => Geometry::Stacked,
            Direction::Up => parent.get_geometry(),
            Direction::None | Direction::Backward | Direction::Forward | Direction::Workspace => {
                return CommandResult::NotHandled;
            }
        };

        log_info2!("Compositor: Change frame geometry to '{:?}'", geometry);

        // Change frame geometry
        if frame.has_children() {
            frame.change_geometry(geometry, &mut self.coordinator);
        } else {
            parent.change_geometry(geometry, &mut self.coordinator);
        }
        CommandResult::Ok
    }

    /// Focus frame found in given direction relatively to given `frame`.
    fn focus(&mut self,
             frame: &mut Frame,
             mut direction: Direction,
             mut position: i32)
             -> CommandResult {
        match direction {
            Direction::Workspace => CommandResult::NotHandled,
            Direction::Backward | Direction::Forward => {
                if direction == Direction::Forward {
                    position = -1 * position;
                }

                if let Some(sid) = self.history.get_nth(position as isize) {
                    self.pop_surface(sid);
                }
                CommandResult::Ok
            }
            _ => {
                let position = if position < 0 {
                    direction = direction.reversed();
                    -position
                } else {
                    position
                } as u32;

                if let Some(new_selection) = frame.find_adjacent(direction, position) {
                    self.select(new_selection);
                }
                CommandResult::Ok
            }
        }
    }

    /// Moves frame in frame layout in given direction by given distance. Moved frame jumps over
    /// other frames.
    fn jump(&mut self,
            reference: &mut Frame,
            mut direction: Direction,
            distance: i32)
            -> CommandResult {
        log_info2!("Compositor: jump");

        // Modify direction if needed
        let distance = if distance < 0 {
            direction = direction.reversed();
            -distance
        } else {
            distance
        } as u32;

        // Choose side
        let side = match direction {
            Direction::North | Direction::West => Side::Before,
            Direction::South | Direction::East => Side::After,
            _ => {
                return CommandResult::NotHandled;
            }
        };

        // Perform jump
        if let Some(mut target) = reference.find_adjacent(direction, distance) {
            let mut source = reference.get_parent().expect("jump reference must have parent");
            reference.jump(side, &mut target, &mut self.coordinator);
            source.deramify();
        }
        CommandResult::Ok
    }

    /// Jumps given frame to workspace with given title. If workspace does not exist new one is
    /// created. Old workspace stays focused.
    fn jump_to_workspace(&mut self, frame: &mut Frame, title: &String) {
        log_info2!("Compositor: jump to workspace '{}'", title);
        let old_workspace = self.find_current_workspace();
        let mut new_workspace = self.bring_workspace(title, false);
        if !old_workspace.equals_exact(&new_workspace) {
            frame.resettle(&mut new_workspace, &mut self.coordinator);
            let most_recent = self.find_most_recent(old_workspace);
            self.select(most_recent);
        }
    }

    /// Moves frame in frame layout in given direction by given distance. Moved frame dives into
    /// other frames.
    fn dive(&mut self,
            reference: &mut Frame,
            mut direction: Direction,
            distance: i32)
            -> CommandResult {
        // Modify direction if needed
        let distance = if distance < 0 {
            direction = direction.reversed();
            -distance
        } else {
            distance
        } as u32;

        // Perform dive
        if let Some(mut target) = reference.find_adjacent(direction, distance) {
            let mut source = reference.get_parent().expect("dive reference must have parent");
            reference.jump(Side::On, &mut target, &mut self.coordinator);
            source.deramify();
        }
        CommandResult::Ok
    }

    /// Dives given frame to workspace with given title. If workspace does not exist new one is
    /// created. Chosen workspace becomes focused.
    fn dive_to_workspace(&mut self, mut frame: Frame, title: &String) {
        log_info2!("Compositor: dive to workspace '{}'", title);
        let old_workspace = self.find_current_workspace();
        let mut new_workspace = self.bring_workspace(title, false);
        if !old_workspace.equals_exact(&new_workspace) {
            frame.jump(Side::On, &mut new_workspace, &mut self.coordinator);
            self.select(frame.clone());
            self.root.pop_recursively(&mut frame);
        }
    }

    /// Moves the frame in given direction.
    ///
    /// TODO: If selected frame is anchored but inside floating frame move whole floating frame.
    fn move_frame(&mut self,
                  frame: &mut Frame,
                  direction: Direction,
                  distance: i32)
                  -> CommandResult {
        if frame.get_mobility().is_floating() {
            let distance = distance as isize;
            let vector = {
                    if direction == Direction::North {
                        Vector::new(0, -distance)
                    } else if direction == Direction::East {
                        Vector::new(distance, 0)
                    } else if direction == Direction::South {
                        Vector::new(0, distance)
                    } else if direction == Direction::West {
                        Vector::new(-distance, 0)
                    } else {
                        Vector::default()
                    }
                }
                .scaled(self.config.move_step as f32);

            if !vector.is_zero() {
                frame.move_with_contents(vector);
            }
        }
        CommandResult::Ok
    }

    /// Handles anchorization command.
    ///
    /// TODO: Extract preferred size from frame.
    fn anchorize(&mut self, mut frame: Frame) -> CommandResult {
        if frame.get_mobility().is_anchored() {
            let workspace = self.find_current_workspace();
            let decision = self.strategist.choose_floating(workspace.get_size(), None);
            frame.deanchorize(decision.area, &mut self.coordinator);
        } else {
            frame.anchorize(&mut self.coordinator);
        }
        CommandResult::Ok
    }

    /// Adds new container just above selection.
    fn ramify(&mut self, mut frame: Frame) {
        // TODO: Geometry should be configurable.
        frame.ramify(Geometry::Stacked);
        self.select(frame);
    }

    /// Jumps frame one level higher.
    fn exalt(&mut self, frame: &mut Frame) {
        // Choose target
        let mut above = frame.get_parent().expect("exalted frame must have parent");
        if above.is_reanchorizable() {
            let mut target = if above.get_geometry() == Geometry::Stacked {
                above = above.get_parent().expect("exalted frame must have grand parent");
                if above.get_geometry() == Geometry::Stacked {
                    above
                } else {
                    above.ramify(Geometry::Stacked)
                }
            } else {
                above.ramify(Geometry::Stacked)
            };

            // Resettle to target
            frame.resettle(&mut target, &mut self.coordinator);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Miscellaneous private methods.
impl<C> Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    /// Find most recently focused frame inside given frame. This function is used to find most
    /// recently used frame when focusing to workspace or when currently focussed frame jumps from
    /// workspace.
    ///
    /// Returns most recently focused frame, or `reference` frame if nothing found.
    ///
    /// Searching for new selection is done by iterating through surface history and checking if
    /// surface with given ID is somewhere in workspace three. Not the most efficient... Any ideas
    /// for improvement?
    fn find_most_recent(&self, reference: Frame) -> Frame {
        for sid in self.history.iter() {
            if let Some(frame) = reference.find_with_sid(sid) {
                return frame.clone();
            }
        }
        reference
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods related to workspaces.
impl<C> Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    /// Search for existing workspace with given title.
    fn find_workspace(&self, title: &String) -> Option<Frame> {
        self.root.find(Some(Mode::Workspace), Some(title))
    }

    /// Search for existing workspace with given title.
    fn find_current_workspace(&self) -> Frame {
        self.selection.find_top().expect("selection should have `top`")
    }

    /// Creates new frame, places it in proper place in frame tree and initializes it as a
    /// workspace.
    fn create_new_workspace(&mut self,
                            mut display: &mut Frame,
                            title: &String,
                            focus: bool)
                            -> Frame {
        log_info2!("Compositor: create new workspace (title: {}, focus: {})", title, focus);

        // Create and configure workspace
        // TODO: Make default workspace geometry configurable.
        let mut workspace = Frame::new_workspace(title.clone(), Geometry::Stacked);
        workspace.settle(&mut display, None, &mut self.coordinator);

        // Focus if requested or make sure current selection stays focused
        if focus {
            self.select(workspace.clone());
            self.root.pop_recursively(&mut workspace);
        } else {
            self.root.pop_recursively(&mut self.selection);
        }
        workspace
    }

    /// Creates next workspace.
    ///
    /// This method will check if workspaces title "1", "2", "3" and so on up to "1000" exist and
    /// create next workspace titled will first available name. 1000 frames is probably to much for
    /// any use. We should not need to create more.
    fn create_next_workspace(&mut self) -> Option<Frame> {
        for i in 1..MAX_WORKSPACES {
            let title = i.to_string();
            if self.find_workspace(&title).is_none() {
                return Some(Frame::new_workspace(title, Geometry::Stacked));
            }
        }
        log_error!("Don't you think {} workspaces isn't enough?", MAX_WORKSPACES);
        None
    }

    /// Search for existing workspace or create new with given title.
    fn bring_workspace(&mut self, title: &String, focus: bool) -> Frame {
        if let Some(workspace) = self.find_workspace(&title) {
            workspace.clone()
        } else {
            // TODO: For many output setup this should be configurable on which output the
            // workspace will be created.
            let mut display_frame = self.find_current_workspace()
                .get_parent()
                .expect("workspace must be contained in display frame");

            self.create_new_workspace(&mut display_frame, title, focus)
        }
    }

    /// Focus workspace with given title.
    fn focus_workspace(&mut self, title: &String) {
        log_info1!("Compositor: Change workspace to '{}'", title);
        let workspace = self.bring_workspace(title, true);
        let mut most_recent = self.find_most_recent(workspace);
        self.select(most_recent.clone());
        self.root.pop_recursively(&mut most_recent);
    }
}

// -------------------------------------------------------------------------------------------------

/// Miscellaneous private methods.
impl<C> Compositor<C>
    where C: ExhibitorCoordinationTrait
{
    /// Set given frame as selected.
    fn select(&mut self, mut frame: Frame) {
        self.root.pop_recursively(&mut frame);
        self.selection = frame;
        if self.selection.get_sid().is_valid() {
            self.coordinator.set_keyboard_focus(self.selection.get_sid());
        }
    }

    /// Print frame layout for log file.
    fn log_frames(&self) {
        let mut timber = timber::lock().expect("Lock logger");
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
            timber.log(format_args!("{:?}", subframe));
            if subframe.equals_exact(&self.selection) {
                timber.log(format_args!(" <--\n"));
            } else {
                timber.log(format_args!("\n"));
            }
            self.log_frames_helper(subframe, depth + 1, timber);
        }
    }
}

// -------------------------------------------------------------------------------------------------
