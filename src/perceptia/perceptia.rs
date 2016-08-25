// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#![cfg_attr(not(test), allow(dead_code))]
#![cfg_attr(not(test), allow(unused_variables))]

extern crate dharma;

mod perceptron;
mod modules;

use dharma::samsara::{Samsara, Module};
use dharma::signaler::Signaler;

use perceptron::Perceptron;
use modules::{ExhibitorModule, WaylandModule};

type Mod = Module<T = Perceptron>;

fn main() {
    let signaler = Signaler::new();
    let mut samsara: Samsara<Perceptron> = Samsara::new(String::from("test"), signaler);

    let mod1: Box<Mod> = Box::new(WaylandModule::new());
    let mod2: Box<Mod> = Box::new(ExhibitorModule::new());
    samsara.add_module(mod1);
    samsara.add_module(mod2);
    let jh = samsara.start().unwrap();
    jh.join().unwrap();
}
