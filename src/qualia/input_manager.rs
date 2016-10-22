// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to catching key bindings and executing assigned
//! functions.
//!
//! This functionality is inspired by `vim`. As there we have here modes to be able to change
//! applications behavior depending on which modes are on or off.

// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use dharma::Signaler;

use defs::{modifier, mode_name, Command, KeyCode, KeyValue};
use enums::{Action, Direction, KeyState};
use config::Config;
use binding_functions::{self, Executor};
use perceptron::{self, Perceptron};

// -------------------------------------------------------------------------------------------------

/// Enumeration for possible results of catching key.
#[derive(PartialEq)]
pub enum KeyCatchResult {
    Caught,
    Passed,
}

// -------------------------------------------------------------------------------------------------

/// Structure for identifying key binding.
///
/// Used as key in hash maps.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Binding {
    code: KeyCode,
    modifiers: modifier::ModifierType,
}

// -------------------------------------------------------------------------------------------------

impl Binding {
    /// `Binding` constructor.
    ///
    /// `uinput_sys` defines codes as `i32` so second constructor added to avoid casting in other
    /// places.
    pub fn new(code: i32, modifiers: modifier::ModifierType) -> Self {
        Binding {
            code: code as KeyCode,
            modifiers: modifiers,
        }
    }

    /// `Binding` constructor.
    pub fn create(code: KeyCode, modifiers: modifier::ModifierType) -> Self {
        Binding {
            code: code,
            modifiers: modifiers,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing mode.
pub struct Mode {
    active: bool,
    name: String,
    bindings: HashMap<Binding, Executor>,
}

// -------------------------------------------------------------------------------------------------

impl Mode {
    /// `Mode` constructor.
    pub fn new(active: bool, name: String) -> Self {
        Mode {
            active: active,
            name: name,
            bindings: HashMap::new(),
        }
    }

    /// Checks if mode is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns name of the mode.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Sets mode active or inactive.
    pub fn make_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Add new binding.
    pub fn add_binding(&mut self, binding: Binding, executor: Executor) {
        self.bindings.insert(binding, executor);
    }

    /// Returns executor for given binding.
    pub fn get_executor(&self, binding: &Binding) -> Option<&Executor> {
        self.bindings.get(binding)
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure manages key bindings and modes.
///
/// Main task is to identify key sequences as bindings and execute assigned function.
/// For thread-safe public version see `InputManager`.
struct InnerInputManager {
    modes: Vec<Mode>,
    command: Command,
    signaler: Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl InnerInputManager {
    /// `InnerInputManager` constructor.
    pub fn new(config: &Config, signaler: Signaler<Perceptron>) -> Self {
        // Create manager
        let mut inner = InnerInputManager {
            modes: Vec::new(),
            command: Command::default(),
            signaler: signaler,
        };

        // Create binding from configuration
        let bindings = config.get_key_binding_config();
        for b in bindings.iter() {
            inner.add_binding(b.mode_name.to_owned(), b.binding.clone(), b.executor);
        }

        // Activate default modes
        inner.make_mode_active(mode_name::COMMON.to_string(), true);
        inner.make_mode_active(mode_name::INSERT.to_string(), true);
        inner
    }


    /// Helper method for finding executor for given binding in active modes.
    fn find_executor(&self, binding: &Binding) -> Option<Executor> {
        for ref mode in self.modes.iter() {
            if mode.is_active() {
                if let Some(executor) = mode.get_executor(binding) {
                    return Some(*executor)
                }
            }
        }
        None
    }

    /// Tries for find executor matching to given key and state of modifiers and execute it if
    /// found.
    pub fn catch_key(&mut self,
                     code: KeyCode,
                     value: KeyValue,
                     modifiers: modifier::ModifierType)
                     -> KeyCatchResult {
        if value == KeyState::Pressed as KeyValue {
            if let Some(executor) = self.find_executor(&Binding::create(code, modifiers)) {
                executor(self);
                KeyCatchResult::Caught
            } else {
                KeyCatchResult::Passed
            }
        } else {
            KeyCatchResult::Passed
        }
    }

    /// Activates or deactivates mode identified by name.
    pub fn make_mode_active(&mut self, mode_name: String, active: bool) {
        for ref mut mode in self.modes.iter_mut() {
            if mode.get_name() == mode_name {
                mode.make_active(active);
                break;
            }
        }
    }

    /// Adds given binding to mode identified by name.
    pub fn add_binding(&mut self, mode_name: String, binding: Binding, executor: Executor) {
        // Try to find mode and add binding to it
        let mut added = false;
        for ref mut mode in self.modes.iter_mut() {
            if mode.get_name() == mode_name {
                mode.add_binding(binding.clone(), executor);
                added = true;
                break;
            }
        }

        // If mode not found - create new
        if !added {
            let mut mode = Mode::new(false, mode_name);
            mode.add_binding(binding, executor);
            self.modes.push(mode);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for locking `InnerInputManager` shared between threads.
///
/// Thread-safe public version of `InnerInputManager`.
#[derive(Clone)]
pub struct InputManager {
    inner: Arc<Mutex<InnerInputManager>>,
}

// -------------------------------------------------------------------------------------------------

impl InputManager {
    /// `InputManager` constructor.
    pub fn new(config: &Config, signaler: Signaler<Perceptron>) -> Self {
        InputManager { inner: Arc::new(Mutex::new(InnerInputManager::new(config, signaler))) }
    }

    /// Lock and call corresponding method from `InnerInputManager`.
    pub fn catch_key(&mut self,
                     code: KeyCode,
                     value: KeyValue,
                     modifiers: modifier::ModifierType)
                     -> KeyCatchResult {
        let mut mine = self.inner.lock().unwrap();
        mine.catch_key(code, value, modifiers)
    }

    /// Lock and call corresponding method from `InnerInputManager`.
    pub fn make_mode_active(&mut self, mode_name: String, active: bool) {
        let mut mine = self.inner.lock().unwrap();
        mine.make_mode_active(mode_name, active)
    }

    /// Lock and call corresponding method from `InnerInputManager`.
    pub fn add_binding(&mut self, mode_name: String, binding: Binding, executor: Executor) {
        let mut mine = self.inner.lock().unwrap();
        mine.add_binding(mode_name, binding, executor)
    }
}

// -------------------------------------------------------------------------------------------------

// These methods will be called from executors when `InputManager` is locked so it is save to
// implement this trait for `InnerInputManager` instead of `InputManager`.
impl binding_functions::InputContext for InnerInputManager {
    fn set_action(&mut self, action: Action) {
        self.command.action = action;
    }

    fn set_direction(&mut self, direction: Direction) {
        self.command.direction = direction;
    }

    fn set_magnitude(&mut self, magnitude: i32) {
        self.command.magnitude = magnitude;
    }

    fn execute_command(&mut self) {
        self.signaler.emit(perceptron::COMMAND, Perceptron::Command(self.command.clone()));
    }

    fn clean_command(&mut self) {
        self.command = Command::default();
    }

    fn activate_mode(&mut self, mode_name: &'static str, active: bool) {
        self.make_mode_active(mode_name.to_string(), active);
    }
}

// -------------------------------------------------------------------------------------------------
