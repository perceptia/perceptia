// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of handlers for Wayland protocol.

pub mod display;
pub mod registry;
pub mod shm;

pub mod compositor;
pub mod shell;
pub mod xdg_shell_v6;
pub mod seat;

pub mod output;
