// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(unused_variables))]

#![feature(fnbox)]

#[macro_use]
extern crate timber;
#[macro_use]
extern crate qualia;
extern crate dharma;
extern crate device_manager;
extern crate wayland_frontend;

mod wayland_module;

use std::boxed::FnBox;

use dharma::{EventLoopInfo, Dispatcher, SignalEventHandler, Signaler, Module};
use qualia::{Context, Coordinator, Perceptron};

use device_manager::DeviceManagerModule;
use wayland_module::WaylandModule;

type Mod = Box<Module<T = Perceptron, C = Context>>;
type Constructor = Box<FnBox() -> Box<Module<T = Perceptron, C = Context>> + Send + Sync>;

fn main() {
    let env = qualia::Env::create();

    // Prepare state
    let signaler = Signaler::new();
    let mut dispatcher = Dispatcher::new();
    let coordinator = Coordinator::new();
    let context = Context::new(signaler.clone(), dispatcher.clone(), coordinator.clone());

    let signal_source = Box::new(SignalEventHandler::new(dispatcher.clone(), signaler.clone()));
    dispatcher.add_source(signal_source);

    // Create loops
    let mut utils_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:utils".to_owned(), signaler, context);

    // Create modules
    let device_manager_module: Constructor =
        Box::new(|| Box::new(DeviceManagerModule::new()) as Mod);
    let wayland_module: Constructor = Box::new(|| Box::new(WaylandModule::new()) as Mod);

    // Assign modules to threads
    utils_info.add_module(device_manager_module);
    utils_info.add_module(wayland_module);

    // Start threads
    let mut join_handles = std::collections::VecDeque::new();
    join_handles.push_back(utils_info.start_event_loop().unwrap());

    // Start main loop
    dispatcher.start();
    log_info1!("Stoped dispatcher!");

    // Join threads
    for jh in join_handles.pop_front() {
        jh.join().unwrap();
    }
    log_info1!("Joined all threads!");
}
