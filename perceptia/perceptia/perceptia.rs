// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(unused_variables))]

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate dharma;
extern crate output;
extern crate coordination;

extern crate aesthetics;
extern crate device_manager;
extern crate exhibitor;
extern crate wayland_frontend;

mod aesthetics_module;
mod device_manager_module;
mod exhibitor_module;
mod wayland_service;

use dharma::{EventLoopInfo, Dispatcher, ServiceInfo, SignalEventHandler, Signaler};
use qualia::InputManager;
use coordination::{Context, Coordinator};

use aesthetics_module::AestheticsModuleConstructor;
use device_manager_module::DeviceManagerModuleConstructor;
use exhibitor_module::ExhibitorModuleConstructor;
use wayland_service::WaylandServiceConstructor;

fn main() {
    // Set panic hook: log the panic and quit application - we want to exit when one of threads
    // panics.
    std::panic::set_hook(Box::new(|info| qualia::functions::panic_hook(info)));

    // Prepare tools
    let env = qualia::Env::create(qualia::LogDestination::LogFile);
    let config = env.read_config();
    let keymap = qualia::Keymap::new(&env, config.get_keyboard_config()).unwrap();
    let settings = qualia::Settings::new(keymap.get_settings());

    // Prepare state
    let signaler = Signaler::new();
    let mut dispatcher = Dispatcher::new();
    let mut dispatcher_controller = dispatcher.get_controller();
    let coordinator = Coordinator::new(signaler.clone());
    let input_manager = InputManager::new(&config.get_keybindings_config(), signaler.clone());
    let context = Context::new(config.clone(),
                               settings.clone(),
                               signaler.clone(),
                               dispatcher_controller.clone(),
                               coordinator.clone(),
                               input_manager.clone());

    // Set up signal handler
    let signal_source = Box::new(SignalEventHandler::new(dispatcher_controller.clone(),
                                                         signaler.clone()));
    dispatcher_controller.add_source(signal_source, dharma::event_kind::READ);

    // Create modules and services
    let aesthetics_module = AestheticsModuleConstructor::new();
    let device_manager_module = DeviceManagerModuleConstructor::new();
    let exhibitor_module = ExhibitorModuleConstructor::new();
    let wayland_service = WaylandServiceConstructor::new(context.clone());

    // Create loops
    let mut utils_info =
        EventLoopInfo::new("p:utils".to_owned(), signaler.clone(), context.clone());

    let mut exhibitor_info =
        EventLoopInfo::new("p:exhibitor".to_owned(), signaler.clone(), context.clone());

    let wayland_info = ServiceInfo::new("p:wayland".to_owned(), wayland_service);

    // Assign modules to threads
    utils_info.add_module(device_manager_module);
    utils_info.add_module(aesthetics_module);
    exhibitor_info.add_module(exhibitor_module);

    // Start threads
    let mut join_handles = std::collections::VecDeque::new();
    join_handles.push_back(utils_info.start().unwrap());
    join_handles.push_back(exhibitor_info.start().unwrap());
    join_handles.push_back(wayland_info.start().unwrap());

    // Start main loop
    dispatcher.run();
    log_info1!("Stopped dispatcher!");

    // Join threads
    while join_handles.len() > 0 {
        if join_handles.pop_front().unwrap().join().is_err() {
            log_warn2!("Error while joining thread");
        }
    }
    log_info1!("Joined all threads!");
}
