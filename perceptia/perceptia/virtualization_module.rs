// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Virtualization.

// -------------------------------------------------------------------------------------------------

use dharma::{Module, ModuleConstructor, SignalId};
use qualia::{perceptron, Perceptron, ClientChange};
use coordination::Context;
use gears::{InputManager, InputForwarder};
use virtualization::Virtualization;

// -------------------------------------------------------------------------------------------------

/// Implementation of `dharma::Module` for Virtualization.
pub struct VirtualizationModule {
    virtualization: Virtualization,
}

// -------------------------------------------------------------------------------------------------

impl VirtualizationModule {
    /// Constructs new `VirtualizationModule`.
    pub fn new(context: &mut Context) -> Self {
        let dispatcher = context.get_dispatcher().clone();
        let signaler = context.get_signaler().clone();
        let config = context.get_config();

        // Construct `InputManager` implementing `InputHandling`.
        let input_manager = InputManager::new(config.get_keybindings_config(), signaler.clone());

        // Construct `InputForwarder` implementing `InputForwarding`.
        let input_forwarder = InputForwarder::new(signaler.clone(), context.get_reference_time());

        // Construct the module.
        VirtualizationModule {
            virtualization: Virtualization::new(Box::new(input_manager),
                                                Box::new(input_forwarder),
                                                dispatcher,
                                                signaler)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for VirtualizationModule {
    type T = Perceptron;
    type C = Context;

    fn get_signals(&self) -> Vec<SignalId> {
        vec![perceptron::REMOTE_CLIENT_CHANGE]
    }

    fn initialize(&mut self) {
        log_info1!("Virtualization module initialized");
    }

    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::RemoteClientChange(change) => {
                match change {
                    ClientChange::Connected { fd } => {
                        self.virtualization.on_client_connected(fd);
                    }
                    ClientChange::Disconnected { id } => {
                        self.virtualization.on_client_disconnected(id);
                    }
                }
            }
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Virtualization module finalized");
    }
}

// -------------------------------------------------------------------------------------------------

pub struct VirtualizationModuleConstructor {}

// -------------------------------------------------------------------------------------------------

impl VirtualizationModuleConstructor {
    /// Constructs new `VirtualizationModuleConstructor`.
    pub fn new() -> Box<ModuleConstructor<T = Perceptron, C = Context>> {
        Box::new(VirtualizationModuleConstructor {})
    }
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructor for VirtualizationModuleConstructor {
    type T = Perceptron;
    type C = Context;

    fn construct(&self, context: &mut Self::C) -> Box<Module<T = Self::T, C = Self::C>> {
        Box::new(VirtualizationModule::new(context))
    }
}

// -------------------------------------------------------------------------------------------------
