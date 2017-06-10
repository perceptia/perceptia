// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Device Manager.

// -------------------------------------------------------------------------------------------------

use dharma::{Module, ModuleConstructor, SignalId};
use qualia::{Perceptron, perceptron};
use coordination::{Context, Coordinator};
use gears::{InputManager, InputForwarder};
use device_manager::DeviceManager;

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModule {
    manager: DeviceManager<Coordinator>,
}

// -------------------------------------------------------------------------------------------------

impl DeviceManagerModule {
    /// `DeviceManagerModule` constructor.
    pub fn new(context: &mut Context) -> Self {
        let coordinator = context.get_coordinator().clone();
        let signaler = context.get_signaler().clone();
        let config = context.get_config();

        // Construct `InputManager` implementing `InputHandling`.
        let input_manager = InputManager::new(config.get_keybindings_config(), signaler.clone());

        // Construct `InputForwarder` implementing `InputForwarding`.
        let input_forwarder = InputForwarder::new(signaler, context.get_reference_time());

        // Construct the module.
        DeviceManagerModule {
            manager: DeviceManager::new(Box::new(input_manager),
                                        Box::new(input_forwarder),
                                        config.get_input_config().clone(),
                                        coordinator),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for DeviceManagerModule {
    type T = Perceptron;
    type C = Context;

    fn get_signals(&self) -> Vec<SignalId> {
        vec![perceptron::SUSPEND,
             perceptron::WAKEUP,
             perceptron::INPUTS_CHANGED,
             perceptron::OUTPUTS_CHANGED]
    }

    fn initialize(&mut self) {
        log_info1!("Device Manager module initialized");
    }

    // FIXME: Finnish handling signals in `DeviceManagerModule`.
    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::Suspend => self.manager.on_suspend(),
            Perceptron::WakeUp => self.manager.on_wakeup(),
            Perceptron::InputsChanged => self.manager.on_inputs_changed(),
            Perceptron::OutputsChanged => self.manager.on_outputs_changed(),
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Device Manager module finalized");
    }
}

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModuleConstructor {}

// -------------------------------------------------------------------------------------------------

impl DeviceManagerModuleConstructor {
    /// Constructs new `DeviceManagerModuleConstructor`.
    pub fn new() -> Box<ModuleConstructor<T = Perceptron, C = Context>> {
        Box::new(DeviceManagerModuleConstructor {})
    }
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructor for DeviceManagerModuleConstructor {
    type T = Perceptron;
    type C = Context;

    fn construct(&self, context: &mut Self::C) -> Box<Module<T = Self::T, C = Self::C>> {
        Box::new(DeviceManagerModule::new(context))
    }
}

// -------------------------------------------------------------------------------------------------
