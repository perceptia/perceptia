// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(unused_variables))]

#![feature(fnbox)]

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
mod wayland_module;

use std::boxed::FnBox;

use dharma::{EventLoopInfo, Dispatcher, SignalEventHandler, Signaler, Module};
use qualia::{Context, Coordinator, InputManager, Perceptron};

use device_manager_module::DeviceManagerModule;
use exhibitor_module::ExhibitorModule;
use wayland_module::WaylandModule;

type Mod = Box<Module<T = Perceptron, C = Context>>;
type Constructor = Box<FnBox() -> Box<Module<T = Perceptron, C = Context>> + Send + Sync>;

fn main() {
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
    dispatcher.add_source(signal_source);

    // Create loops
    let mut utils_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:utils".to_owned(), signaler.clone(), context.clone());

    let mut exhibitor_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:exhibitor".to_owned(), signaler.clone(), context.clone());

    // Create modules
    let device_manager_module: Constructor =
        Box::new(|| Box::new(DeviceManagerModule::new()) as Mod);
    let wayland_module: Constructor = Box::new(|| Box::new(WaylandModule::new()) as Mod);
    let exhibitor_module: Constructor = Box::new(|| Box::new(ExhibitorModule::new()) as Mod);

    // Assign modules to threads
    utils_info.add_module(device_manager_module);
    utils_info.add_module(wayland_module);
    exhibitor_info.add_module(exhibitor_module);

    // Start threads
    let mut join_handles = std::collections::VecDeque::new();
    join_handles.push_back(utils_info.start_event_loop().unwrap());
    join_handles.push_back(exhibitor_info.start_event_loop().unwrap());

    // Start main loop
    dispatcher.start();
    log_info1!("Stoped dispatcher!");

    // Join threads
    for jh in join_handles.pop_front() {
        jh.join().unwrap();
    }
    log_info1!("Joined all threads!");
}
