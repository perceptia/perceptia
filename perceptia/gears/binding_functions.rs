// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functions to be used as handlers for key bindings.

// -------------------------------------------------------------------------------------------------

use qualia::{Action, Direction, KeyCode};

use input_manager::mode_name;
use functions;

// -------------------------------------------------------------------------------------------------

/// Type definition for functions to be executed as key binding handler.
pub trait Executor: Send + Sync {
    fn execute(&self, &mut InputContext);
    fn duplicate(&self) -> Box<Executor>;
}

// -------------------------------------------------------------------------------------------------

/// Enum describing how the command was previously modified.
#[derive(Clone, Copy, PartialEq)]
pub enum PreviousModification {
    Action,
    Direction,
    Magnitude,
    String,
    None,
}

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

    /// Tells how the command was previously modified.
    fn previous_modification(&self) -> PreviousModification;

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

/// Helper macro definig implementation of `Executor`.
macro_rules! define_simple_executor {
    ($name:ident($context:ident) $callback:block) => {
        #[derive(Clone)]
        pub struct $name {}
        impl $name {
            pub fn new() -> Box<Self> { Box::new(Self{}) }
        }
        impl Executor for $name {
            fn duplicate(&self) -> Box<Executor> { Self::new() }
            fn execute(&self, $context: &mut InputContext) {
                $callback
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Dummy no-op executor.
define_simple_executor!(Nop(_context) {});

// -------------------------------------------------------------------------------------------------

/// Cleans compositor command.
define_simple_executor!(CleanCommand(context) {
    context.clean_command();
});

// -------------------------------------------------------------------------------------------------

/// Shuts down the application by sending `SIGTERM` to itself.
define_simple_executor!(Quit(_context) {
    functions::quit();
});

// -------------------------------------------------------------------------------------------------

/// Sets focus action in command but do not execute.
define_simple_executor!(PutFocus(context) {
    put_action(context, Action::Focus);
});

// -------------------------------------------------------------------------------------------------

/// Sets swap action in command but do not execute.
define_simple_executor!(PutSwap(context) {
    put_action(context, Action::Swap);
});

// -------------------------------------------------------------------------------------------------

/// Sets jump action in command but do not execute.
define_simple_executor!(PutJump(context) {
    put_action(context, Action::Jump);
});

// -------------------------------------------------------------------------------------------------

/// Sets dive action in command but do not execute.
define_simple_executor!(PutDive(context) {
    put_action(context, Action::Dive);
});

// -------------------------------------------------------------------------------------------------

/// Sets move action in command but do not execute.
define_simple_executor!(PutMove(context) {
    put_action(context, Action::Move);
});

// -------------------------------------------------------------------------------------------------

/// Sets north direction in command but do not execute.
define_simple_executor!(PutNorth(context) {
    put_direction(context, Direction::North);
});

// -------------------------------------------------------------------------------------------------

/// Sets east direction in command but do not execute.
define_simple_executor!(PutEast(context) {
    put_direction(context, Direction::East);
});

// -------------------------------------------------------------------------------------------------

/// Sets south direction in command but do not execute.
define_simple_executor!(PutSouth(context) {
    put_direction(context, Direction::South);
});

// -------------------------------------------------------------------------------------------------

/// Sets west direction in command but do not execute.
define_simple_executor!(PutWest(context) {
    put_direction(context, Direction::West);
});

// -------------------------------------------------------------------------------------------------

/// Sets forward direction in command but do not execute.
define_simple_executor!(PutForward(context) {
    put_direction(context, Direction::Forward);
});

// -------------------------------------------------------------------------------------------------

/// Sets backward direction in command but do not execute.
define_simple_executor!(PutBackward(context) {
    put_direction(context, Direction::Backward);
});

// -------------------------------------------------------------------------------------------------

/// Sets begin direction in command but do not execute.
define_simple_executor!(PutBegin(context) {
    put_direction(context, Direction::Begin);
});

// -------------------------------------------------------------------------------------------------

/// Sets end direction in command but do not execute.
define_simple_executor!(PutEnd(context) {
    put_direction(context, Direction::End);
});

// -------------------------------------------------------------------------------------------------

/// Sets magnitude in command but do not execute.
define_simple_executor!(PutMagnitude(context) {
    if let Some(number) = context.get_code_as_number() {
        if context.previous_modification() == PreviousModification::Magnitude {
            let magnitude = context.get_magnitude();
            context.set_magnitude(10 * magnitude + number);
        } else {
            context.set_magnitude(number);
        }
    }
});

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to horizontal.
define_simple_executor!(Horizontalize(context) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::East);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to vertical.
define_simple_executor!(Verticalize(context) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::North);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command changing selected frame geometry to stacked.
define_simple_executor!(Stackize(context) {
    context.set_action(Action::Configure);
    context.set_direction(Direction::End);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command toggling anchorization.
define_simple_executor!(ToggleAnchorization(context) {
    context.set_action(Action::Anchor);
    context.set_direction(Direction::None);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command for circling surfaces forward.
define_simple_executor!(CicleHistoryForward(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Forward);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command for circling surfaces backward.
define_simple_executor!(CicleHistoryBackward(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::Backward);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface on the right.
define_simple_executor!(FocusRight(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface below.
define_simple_executor!(FocusDown(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface on the left.
define_simple_executor!(FocusLeft(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command focusing surface above.
define_simple_executor!(FocusUp(context) {
    context.set_action(Action::Focus);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface on the right.
define_simple_executor!(JumpRight(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface below.
define_simple_executor!(JumpDown(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface on the left.
define_simple_executor!(JumpLeft(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command jumping over surface above.
define_simple_executor!(JumpUp(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Jumps frame one level higher.
define_simple_executor!(Exalt(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::Begin);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Adds new container just above selection.
define_simple_executor!(Ramify(context) {
    context.set_action(Action::Jump);
    context.set_direction(Direction::End);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame on the right.
define_simple_executor!(DiveRight(context) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::East);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame below.
define_simple_executor!(DiveDown(context) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::South);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame on the left.
define_simple_executor!(DiveLeft(context) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::West);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Executes command diving into frame above.
define_simple_executor!(DiveUp(context) {
    context.set_action(Action::Dive);
    context.set_direction(Direction::North);
    context.set_magnitude(1);
    context.execute_command();
});

// -------------------------------------------------------------------------------------------------

/// Jumps selected frame to workspace (does not focus workspace).
///
/// E.g. if key [5] was pressed, will jump into workspace titled "5".
define_simple_executor!(JumpToWorkspace(context) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Jump);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
});

// -------------------------------------------------------------------------------------------------

/// Jumps selected frame to workspace (focuses workspace).
///
/// E.g. if key [5] was pressed, will dive into workspace titled "5".
define_simple_executor!(DiveToWorkspace(context) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Dive);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
});

// -------------------------------------------------------------------------------------------------

/// Focuses the workspace basing on key code.
///
/// E.g. if key [5] was pressed, workspace titled "5" will be focused.
define_simple_executor!(FocusWorkspace(context) {
    if let Some(number) = context.get_code_as_number() {
        context.set_action(Action::Focus);
        context.set_direction(Direction::Workspace);
        context.set_magnitude(1);
        context.set_string(number.to_string());
        context.execute_command();
    }
});

// -------------------------------------------------------------------------------------------------

/// Switches normal mode off and insert mode on.
define_simple_executor!(SwapModeNormalToInsert(context) {
    log_info2!("Swap mode from normal to insert");
    context.activate_mode(mode_name::NORMAL, false);
    context.activate_mode(mode_name::INSERT, true);
});

// -------------------------------------------------------------------------------------------------

/// Switches insert mode off and normal mode on.
define_simple_executor!(SwapModeInsertToNormal(context) {
    log_info2!("Swap mode from insert to normal");
    context.activate_mode(mode_name::INSERT, false);
    context.activate_mode(mode_name::NORMAL, true);
});

// -------------------------------------------------------------------------------------------------

/// Spawns new process.
#[derive(Clone)]
pub struct SpawnProcess {
    command: Vec<String>,
}

impl SpawnProcess {
    pub fn new(command: &[&'static str]) -> Box<Self> {
        Box::new(SpawnProcess {
                     command: command.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                 })
    }

    pub fn new_from_vec(command: Vec<String>) -> Box<Self> {
        Box::new(SpawnProcess { command: command })
    }
}

impl Executor for SpawnProcess {
    fn execute(&self, _context: &mut InputContext) {
        functions::spawn_process(&self.command);
    }

    fn duplicate(&self) -> Box<Executor> {
        Self::new_from_vec(self.command.clone())
    }
}

// -------------------------------------------------------------------------------------------------
