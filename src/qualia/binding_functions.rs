// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functions to be used as handlers for key bindings.

// -------------------------------------------------------------------------------------------------

use libc;

use enums::{Action, Direction};
use defs::{KeyCode, mode_name};

// -------------------------------------------------------------------------------------------------

/// Type definition for functions to be executed as key binding handler.
pub type Executor = fn(&mut InputContext);

// -------------------------------------------------------------------------------------------------

/// Trait for contexts passed for key binding handler functions.
///
/// Context allows to
/// - build command executed by compositor
/// - activate/deactivate input modes
pub trait InputContext {
    /// Sets command action.
    fn set_action(&mut self, action: Action);

    /// Sets command direction.
    fn set_direction(&mut self, direction: Direction);

    /// Sets command magnitude.
    fn set_magnitude(&mut self, magnitude: i32);

    /// Sets command string.
    fn set_string(&mut self, string: String);

    /// Gets command action.
    fn get_action(&mut self) -> Action;

    /// Gets command direction.
    fn get_direction(&mut self) -> Direction;

    /// Gets command magnitude.
    fn get_magnitude(&mut self) -> i32;

    /// Gets command string.
    fn get_string(&mut self) -> String;

    /// Tells compositor to execute built command.
    fn execute_command(&mut self);

    /// Clears command.
    fn clean_command(&mut self);

    /// Activates/deactivates input mode.
    fn activate_mode(&mut self, mode_name: &'static str, active: bool);

    /// Returns the code of key that triggered the binging executor.
    fn get_code(&self) -> KeyCode;

    /// Just like `get_code` but returns number if number key was pressed, `None` otherwise.
    fn get_code_as_number(&self) -> Option<i32>;
}

// -------------------------------------------------------------------------------------------------

/// Helper function to putting actions.
fn put_action(context: &mut InputContext, action: Action) {
    context.set_action(action);
    if context.get_direction() != Direction::None {
        if context.get_magnitude() == 0 {
            context.set_magnitude(1);
        }
        context.execute_command();
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper function to putting directions.
fn put_direction(context: &mut InputContext, direction: Direction) {
    context.set_direction(direction);
    if context.get_action() != Action::None {
        if context.get_magnitude() == 0 {
            context.set_magnitude(1);
        }
        context.execute_command();
    }
}

// -------------------------------------------------------------------------------------------------

/// Cleans compositor command.
pub fn clean_command(context: &mut InputContext) {
    context.clean_command();
}

// -------------------------------------------------------------------------------------------------

/// Shuts down the application by sending `SIGTERM` to itself.
#[allow(unused_variables)]
pub fn quit(context: &mut InputContext) {
    log_info1!("QUIT!");
    unsafe { libc::kill(libc::getpid(), libc::SIGTERM) };
}

// -------------------------------------------------------------------------------------------------

/// Sets focus action in command but do not execute.
pub fn put_focus(context: &mut InputContext) {
    put_action(context, Action::Focus);
}

// -------------------------------------------------------------------------------------------------

/// Sets swap action in command but do not execute.
pub fn put_swap(context: &mut InputContext) {
    put_action(context, Action::Swap);
}

// -------------------------------------------------------------------------------------------------

/// Sets jump action in command but do not execute.
pub fn put_jump(context: &mut InputContext) {
    put_action(context, Action::Jump);
}

// -------------------------------------------------------------------------------------------------

/// Sets dive action in command but do not execute.
pub fn put_dive(context: &mut InputContext) {
    put_action(context, Action::Dive);
}

// -------------------------------------------------------------------------------------------------

/// Sets north direction in command but do not execute.
pub fn put_north(context: &mut InputContext) {
    put_direction(context, Direction::North);
}

// -------------------------------------------------------------------------------------------------

/// Sets east direction in command but do not execute.
pub fn put_east(context: &mut InputContext) {
    put_direction(context, Direction::East);
}

// -------------------------------------------------------------------------------------------------

/// Sets south direction in command but do not execute.
pub fn put_south(context: &mut InputContext) {
    put_direction(context, Direction::South);
}

// -------------------------------------------------------------------------------------------------

/// Sets west direction in command but do not execute.
pub fn put_west(context: &mut InputContext) {
    put_direction(context, Direction::West);
}

// -------------------------------------------------------------------------------------------------

/// Sets forward direction in command but do not execute.
pub fn put_forward(context: &mut InputContext) {
    put_direction(context, Direction::Forward);
}

// -------------------------------------------------------------------------------------------------

/// Sets backward direction in command but do not execute.
pub fn put_backward(context: &mut InputContext) {
    put_direction(context, Direction::Backward);
}

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to horizontal.
pub fn horizontalize(context: &mut InputContext) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::East);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to vertical.
pub fn verticalize(context: &mut InputContext) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::North);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to stacked.
pub fn stackize(context: &mut InputContext) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::End);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command for circling surfaces forward.
pub fn cicle_history_forward(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Forward);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command for circling surfaces backward.
pub fn cicle_history_backward(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Backward);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface on the right.
pub fn focus_right(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface below.
pub fn focus_down(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface on the left.
pub fn focus_left(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface above.
pub fn focus_up(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface on the right.
pub fn jump_right(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface below.
pub fn jump_down(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface on the left.
pub fn jump_left(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface above.
pub fn jump_up(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Jumps frame one level higher.
pub fn exalt(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::Begin);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Adds new container just above selection.
pub fn ramify(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::End);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame on the right.
pub fn dive_right(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame below.
pub fn dive_down(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame on the left.
pub fn dive_left(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame above.
pub fn dive_up(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Jumps selected frame to workspace (does not focus workspace).
///
/// E.g. if key [5] was pressed, will jump into workspace titled "5".
pub fn jump_to_workspace(context: &mut InputContext) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Jump);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
}

// -------------------------------------------------------------------------------------------------

/// Jumps selected frame to workspace (focuses workspace).
///
/// E.g. if key [5] was pressed, will dive into workspace titled "5".
pub fn dive_to_workspace(context: &mut InputContext) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Dive);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
}

// -------------------------------------------------------------------------------------------------

/// Focuses the workspace basing on key code.
///
/// E.g. if key [5] was pressed, workspace titled "5" will be focused.
pub fn focus_workspace(context: &mut InputContext) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Focus);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
}

// -------------------------------------------------------------------------------------------------

/// Switches normal mode off and insert mode on.
pub fn swap_mode_normal_to_insert(context: &mut InputContext)
{
    log_info2!("Swap mode from normal to insert");
    context.activate_mode(mode_name::NORMAL, false);
    context.activate_mode(mode_name::INSERT, true);
}

// -------------------------------------------------------------------------------------------------

/// Switches insert mode off and normal mode on.
pub fn swap_mode_insert_to_normal(context: &mut InputContext)
{
    log_info2!("Swap mode from insert to normal");
    context.activate_mode(mode_name::INSERT, false);
    context.activate_mode(mode_name::NORMAL, true);
}

// -------------------------------------------------------------------------------------------------
