// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(unused_variables))]

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate dharma;
extern crate exhibitor;
extern crate device_manager;
extern crate wayland_frontend;

mod device_manager_module;
mod exhibitor_module;
mod wayland_service;

use dharma::{EventLoopInfo, Dispatcher, SignalEventHandler, Signaler};
use qualia::{Context, Coordinator, InputManager};

use device_manager_module::DeviceManagerModuleConstructor;
use exhibitor_module::ExhibitorModuleConstructor;
use wayland_service::WaylandServiceConstructor;

fn main() {
    // Set panic hook: log the panic and quit application - we want to exit when one of threads
    // panics.
    std::panic::set_hook(Box::new(|info| qualia::functions::panic_hook(info)));

    // Prepare tools
    let env = qualia::Env::create();
    let config = env.read_config();
    let keymap = qualia::Keymap::new(&env).unwrap();
    let settings = qualia::Settings::new(keymap.get_settings());

    // Prepare state
    let signaler = Signaler::new();
    let mut dispatcher = Dispatcher::new();
    let coordinator = Coordinator::new(signaler.clone());
    let input_manager = InputManager::new(&config, signaler.clone());
    let context = Context::new(config.clone(),
                               settings.clone(),
                               signaler.clone(),
                               dispatcher.clone(),
                               coordinator.clone(),
                               input_manager.clone());

    let signal_source = Box::new(SignalEventHandler::new(dispatcher.clone(), signaler.clone()));
    dispatcher.add_source(signal_source, dharma::event_kind::READ);

    // Create loops
    let mut utils_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:utils".to_owned(), signaler.clone(), context.clone());

    let mut exhibitor_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:exhibitor".to_owned(), signaler.clone(), context.clone());

    let wayland_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:wayland".to_owned(), signaler.clone(), context.clone());

    // Create modules and services
    let device_manager_module = DeviceManagerModuleConstructor::new();
    let exhibitor_module = ExhibitorModuleConstructor::new();
    let wayland_service = WaylandServiceConstructor::new(context.clone());

    // Assign modules to threads
    utils_info.add_module(device_manager_module);
    exhibitor_info.add_module(exhibitor_module);

    // Start threads
    let mut join_handles = std::collections::VecDeque::new();
    join_handles.push_back(utils_info.start_event_loop().unwrap());
    join_handles.push_back(exhibitor_info.start_event_loop().unwrap());
    join_handles.push_back(wayland_info.start_service(wayland_service).unwrap());

    // Start main loop
    dispatcher.start();
    log_info1!("Stopped dispatcher!");

    // Join threads
    for jh in join_handles.pop_front() {
        jh.join().unwrap();
    }
    log_info1!("Joined all threads!");
}
