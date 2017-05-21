// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains extentions to Exhibitor useful in tests.

use qualia::{Action, Command, Direction, ExhibitorCoordinationTrait};
use exhibitor::Exhibitor;

// -------------------------------------------------------------------------------------------------

/// Mixin for Exhibitor with shorhand methods for handling commands.
pub trait ExhibitorCommandShorthands {
    /// Executes exalt command.
    fn exalt(&mut self);

    /// Executes ramify command.
    fn ramify(&mut self);

    /// Executes verticalize command.
    fn verticalize(&mut self);

    /// Executes horizontalize command.
    fn horizontalize(&mut self);

    /// Executes focus left command.
    fn focus_left(&mut self);

    /// Executes focus up command.
    fn focus_up(&mut self);

    /// Executes focus right command.
    fn focus_right(&mut self);

    /// Executes focus down command.
    fn focus_down(&mut self);

    /// Executes dive left command.
    fn dive_left(&mut self);

    /// Executes dive up command.
    fn dive_up(&mut self);

    /// Executes dive right command.
    fn dive_right(&mut self);

    /// Executes dive down command.
    fn dive_down(&mut self);

    /// Executes jump to workspace command.
    fn jump_to_workspace(&mut self, workspace_name: &str);
}

// -------------------------------------------------------------------------------------------------

impl<C> ExhibitorCommandShorthands for Exhibitor<C>
    where C: ExhibitorCoordinationTrait
{
    /// Executes exalt command.
    fn exalt(&mut self) {
        let command = Command {
            action: Action::Jump,
            direction: Direction::Begin,
            magnitude: 0,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes ramify command.
    fn ramify(&mut self) {
        let command = Command {
            action: Action::Jump,
            direction: Direction::End,
            magnitude: 0,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes verticalize command.
    fn verticalize(&mut self) {
        let command = Command {
            action: Action::Configure,
            direction: Direction::North,
            magnitude: 0,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes horizontalize command.
    fn horizontalize(&mut self) {
        let command = Command {
            action: Action::Configure,
            direction: Direction::West,
            magnitude: 0,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes focus left command.
    fn focus_left(&mut self) {
        let command = Command {
            action: Action::Focus,
            direction: Direction::West,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes focus up command.
    fn focus_up(&mut self) {
        let command = Command {
            action: Action::Focus,
            direction: Direction::North,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes focus right command.
    fn focus_right(&mut self) {
        let command = Command {
            action: Action::Focus,
            direction: Direction::East,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes focus down command.
    fn focus_down(&mut self) {
        let command = Command {
            action: Action::Focus,
            direction: Direction::South,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes dive left command.
    fn dive_left(&mut self) {
        let command = Command {
            action: Action::Dive,
            direction: Direction::West,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes dive up command.
    fn dive_up(&mut self) {
        let command = Command {
            action: Action::Dive,
            direction: Direction::North,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes dive right command.
    fn dive_right(&mut self) {
        let command = Command {
            action: Action::Dive,
            direction: Direction::East,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes dive down command.
    fn dive_down(&mut self) {
        let command = Command {
            action: Action::Dive,
            direction: Direction::South,
            magnitude: 1,
            string: String::default(),
        };

        self.on_command(command);
    }

    /// Executes jump to workspace command.
    fn jump_to_workspace(&mut self, workspace_name: &str) {
        let command = Command {
            action: Action::Jump,
            direction: Direction::Workspace,
            magnitude: 0,
            string: workspace_name.to_owned(),
        };

        self.on_command(command);
    }
}

// -------------------------------------------------------------------------------------------------
