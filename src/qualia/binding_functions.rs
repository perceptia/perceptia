// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functions to be used as handlers for key bindings.

// -------------------------------------------------------------------------------------------------

use libc;

use enums::{Action, Direction};
use defs::mode_name;

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
    /// Set command action.
    fn set_action(&mut self, action: Action);

    /// Set command direction.
    fn set_direction(&mut self, direction: Direction);

    /// Set command magnitude.
    fn set_magnitude(&mut self, magnitude: i32);

    /// Tell compositor to execute built command.
    fn execute_command(&mut self);

    /// Clear command.
    fn clean_command(&mut self);

    /// Activate/deactivate input mode.
    fn activate_mode(&mut self, mode_name: &'static str, active: bool);
}

// -------------------------------------------------------------------------------------------------

/// Clean compositor command.
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

/// Execute command for circling surfaces forward.
pub fn cicle_history_forward(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Forward);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command for circling surfaces backward.
pub fn cicle_history_backward(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Back);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command focusing surface on the right.
pub fn focus_right(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command focusing surface below.
pub fn focus_down(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command focusing surface on the left.
pub fn focus_left(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command focusing surface above.
pub fn focus_up(context: &mut InputContext) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command jumping over surface on the right.
pub fn jump_right(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command jumping over surface below.
pub fn jump_down(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command jumping over surface on the left.
pub fn jump_left(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command jumping over surface above.
pub fn jump_up(context: &mut InputContext) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command diving into frame on the right.
pub fn dive_right(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command diving into frame below.
pub fn dive_down(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command diving into frame on the left.
pub fn dive_left(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Execute command diving into frame above.
pub fn dive_up(context: &mut InputContext) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
}

// -------------------------------------------------------------------------------------------------

/// Switch normal mode off and insert mode on.
pub fn swap_mode_normal_to_insert(context: &mut InputContext)
{
    log_info2!("Swap mode from normal to insert");
    context.activate_mode(mode_name::NORMAL, false);
    context.activate_mode(mode_name::INSERT, true);
}

// -------------------------------------------------------------------------------------------------

/// Switch insert mode off and normal mode on.
pub fn swap_mode_insert_to_normal(context: &mut InputContext)
{
    log_info2!("Swap mode from insert to normal");
    context.activate_mode(mode_name::INSERT, false);
    context.activate_mode(mode_name::NORMAL, true);
}

// -------------------------------------------------------------------------------------------------
