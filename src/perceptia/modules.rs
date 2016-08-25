// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Temporary module containing example `Modules`.

// -------------------------------------------------------------------------------------------------

use dharma::{Context, InitResult, Module};

use perceptron;
use perceptron::Perceptron;

// -------------------------------------------------------------------------------------------------

pub struct WaylandModule {
    i: i32,
}

// -------------------------------------------------------------------------------------------------

impl WaylandModule {
    /// `WaylandModule` constructor.
    pub fn new() -> Self {
        WaylandModule { i: 5 }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for WaylandModule {
    type T = Perceptron;

    #[allow(unused_variables)]
    fn initialize(&mut self, context: &mut Context<Self::T>) -> InitResult {
        println!("initialize {}", self.i);
        vec![perceptron::EVENT_A, perceptron::EVENT_B]
    }

    fn execute(&mut self, package: &Self::T) {
        println!("exec {} {}", self.i, package);
        match *package {
            Perceptron::A(ref a) => {
                println!("{}", a);
            }
            Perceptron::B(ref b) => {
                println!("{}", b);
            }
        }
    }

    fn finalize(&mut self) {
        println!("finalize {}", self.i);
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ExhibitorModule {
    s: String,
}

// -------------------------------------------------------------------------------------------------

impl ExhibitorModule {
    /// `ExhibitorModule` constructor.
    pub fn new() -> Self {
        ExhibitorModule { s: String::from("ex") }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for ExhibitorModule {
    type T = Perceptron;

    #[allow(unused_variables)]
    fn initialize(&mut self, context: &mut Context<Self::T>) -> InitResult {
        println!("initialize {}", self.s);
        vec![0]
    }

    #[allow(unused_variables)]
    fn execute(&mut self, package: &Self::T) {
        println!("exec {} {}", self.s, package);
    }

    fn finalize(&mut self) {
        println!("finalize {}", self.s);
    }
}

// -------------------------------------------------------------------------------------------------
