// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(dead_code))]
#![cfg_attr(not(test), allow(unused_variables))]

#![feature(fnbox)]

extern crate dharma;
extern crate qualia;
extern crate device_manager;

use std::boxed::FnBox;

use dharma::{EventLoopInfo, Dispatcher, Signaler, Module};
use qualia::{Context, Perceptron};
use device_manager::DeviceManagerModule;

type Mod = Box<Module<T = Perceptron, C = Context>>;
type Constructor = Box<FnBox() -> Box<Module<T = Perceptron, C = Context>> + Send + Sync>;

fn main() {
    // Prepare state
    let signaler = Signaler::new();
    let dispatcher = Dispatcher::new();
    let context = Context::new(signaler.clone(), dispatcher.clone());

    // Create loops
    let mut utils_info: EventLoopInfo<_, _> =
        EventLoopInfo::new("p:utils".to_owned(), signaler, context);

    // Create modules
    let device_manager_module: Constructor =
        Box::new(|| Box::new(DeviceManagerModule::new()) as Mod);

    // Assign modules to threads
    utils_info.add_module(device_manager_module);

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
