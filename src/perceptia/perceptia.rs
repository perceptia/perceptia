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

use dharma::{EventLoopInfo, Dispatcher, Signaler, Module};
use qualia::{Context, Coordinator, Perceptron};

use device_manager::DeviceManagerModule;
use wayland_module::WaylandModule;

type Mod = Box<Module<T = Perceptron, C = Context>>;
type Constructor = Box<FnBox() -> Box<Module<T = Perceptron, C = Context>> + Send + Sync>;

fn main() {
    match timber::init(std::path::Path::new("/tmp/log")) {
        Ok(_) => log_info1!("Welcome to perceptia! Log in /tmp/log"),
        Err(err) => log_error!("Could not initialize logger: {}", err),
    };

    // Prepare state
    let signaler = Signaler::new();
    let dispatcher = Dispatcher::new();
    let coordinator = Coordinator::new();
    let context = Context::new(signaler.clone(), dispatcher.clone(), coordinator.clone());

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

    // Join threads
    for jh in join_handles.pop_front() {
        jh.join().unwrap();
    }
}
