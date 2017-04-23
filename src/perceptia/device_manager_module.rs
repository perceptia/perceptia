// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of `dharma::Module` for Device Manager.

// -------------------------------------------------------------------------------------------------

use dharma::{InitResult, Module, ModuleConstructor};
use qualia::{Perceptron, perceptron};
use coordination::Context;
use device_manager::DeviceManager;

// -------------------------------------------------------------------------------------------------

pub struct DeviceManagerModule<'a> {
    manager: DeviceManager<'a>,
}

// -------------------------------------------------------------------------------------------------

impl<'a> DeviceManagerModule<'a> {
    /// `DeviceManagerModule` constructor.
    pub fn new(context: &mut Context) -> Self {
        DeviceManagerModule { manager: DeviceManager::new(context.clone()) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Module for DeviceManagerModule<'a> {
    type T = Perceptron;
    type C = Context;

    fn initialize(&mut self) -> InitResult {
        vec![perceptron::SUSPEND, perceptron::WAKEUP]
    }

    // FIXME: Finnish handling signals in `DeviceManagerModule`.
    fn execute(&mut self, package: &Self::T) {
        match *package {
            Perceptron::Suspend => self.manager.on_suspend(),
            Perceptron::WakeUp => self.manager.on_wakeup(),
            _ => {}
        }
    }

    fn finalize(&mut self) {
        log_info1!("Finalized Device Manager module");
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
