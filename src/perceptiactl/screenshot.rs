// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Taking screenshot.

use std::collections::HashSet;
use std::path::Path;
use image;

use skylane_simple_framework::{Application, Controller, OutputInfo};
use skylane_simple_framework::{Listener, ListenerConstructor};

// -------------------------------------------------------------------------------------------------

const GLOBAL_OUTPUT: &'static str = "wl_output";
const GLOBAL_SHM: &'static str = "wl_shm";
const GLOBAL_SCREENSHOOTER: &'static str = "weston_screenshooter";

// -------------------------------------------------------------------------------------------------

pub fn process() {
    println!("Taking screenshot");
    Application::new().run(ScreenshooterConstructor::new())
}

// -------------------------------------------------------------------------------------------------

struct ScreenshooterConstructor {}

impl ScreenshooterConstructor {
    fn new() -> Self {
        ScreenshooterConstructor {}
    }
}

impl ListenerConstructor for ScreenshooterConstructor {
    type Listener = Screenshooter;

    fn construct(&self, controller: Controller) -> Box<Self::Listener> {
        Box::new(Screenshooter::new(controller))
    }
}

// -------------------------------------------------------------------------------------------------

struct Screenshooter {
    controller: Controller,
}

// -------------------------------------------------------------------------------------------------

impl Screenshooter {
    pub fn new(controller: Controller) -> Self {
        Screenshooter { controller: controller }
    }
}

// -------------------------------------------------------------------------------------------------

impl Listener for Screenshooter {
    fn init_done(&mut self, globals: HashSet<String>) {
        println!("Init done");
        for global in vec![GLOBAL_OUTPUT, GLOBAL_SHM, GLOBAL_SCREENSHOOTER] {
            if !globals.contains(global) {
                println!("Server does not provide '{}' global interface", global);
                self.controller.stop();
            }
        }
    }

    fn outputs_done(&mut self, mut outputs: Vec<OutputInfo>) {
        println!("Outputs:");
        for output in outputs.iter() {
            println!(" => {:?}", output);
        }
        self.controller.take_screenshot(&outputs.pop().unwrap());
    }

    fn screenshot_done(&mut self, buffer: &[u8], width: usize, height: usize) {
        println!("Screenshot done");

        // TODO: Generate better file name and allow passing it from command line.
        let path = Path::new("screenshot.png");

        match image::save_buffer(&path, buffer, width as u32, height as u32, image::RGBA(8)) {
            Ok(_) => println!("Screenshot written to {:?}", path),
            Err(_) => println!("Failed to write screenshot to {:?}", path),
        }
        self.controller.stop();
    }

    fn screenshot_failed(&mut self) {
        println!("Screenshot failed");
        self.controller.stop();
    }
}

// -------------------------------------------------------------------------------------------------
