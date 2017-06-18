// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Taking screenshot.

use std::collections::HashSet;
use std::path::Path;
use image;
use image::GenericImage;

use skylane_simple_framework::{Application, Controller, OutputInfo};
use skylane_simple_framework::{Listener, ListenerConstructor};

// -------------------------------------------------------------------------------------------------

const GLOBAL_OUTPUT: &'static str = "wl_output";
const GLOBAL_SHM: &'static str = "wl_shm";
const GLOBAL_SCREENSHOOTER: &'static str = "weston_screenshooter";

// -------------------------------------------------------------------------------------------------

pub fn process(path: String) {
    println!("Taking screenshot");
    Application::new().run(ScreenshooterConstructor::new(path))
}

// -------------------------------------------------------------------------------------------------

struct ScreenshooterConstructor {
    path: String,
}

impl ScreenshooterConstructor {
    fn new(path: String) -> Self {
        ScreenshooterConstructor {
            path: path,
        }
    }
}

impl ListenerConstructor for ScreenshooterConstructor {
    type Listener = Screenshooter;

    fn construct(&self, controller: Controller) -> Box<Self::Listener> {
        Box::new(Screenshooter::new(controller, self.path.clone()))
    }
}

// -------------------------------------------------------------------------------------------------

struct Screenshooter {
    controller: Controller,
    outputs: Vec<OutputInfo>,
    image: Option<image::RgbaImage>,
    x_offset: isize,
    y_offset: isize,
    path: String
}

// -------------------------------------------------------------------------------------------------

impl Screenshooter {
    pub fn new(controller: Controller, path: String) -> Self {
        Screenshooter {
            controller: controller,
            outputs: Vec::new(),
            image: None,
            x_offset: 0,
            y_offset: 0,
            path: path,
        }
    }

    pub fn handle_screenshot(&mut self, buffer: Vec<u8>) {
        let output = self.outputs.pop().expect("pop outputs");
        println!(" => Output {} done", output.id);

        let image = image::RgbaImage::from_raw(output.width as u32, output.height as u32, buffer)
            .expect("create subimage");
        {
            let pos_x = output.x + self.x_offset;
            let pos_y = output.y + self.y_offset;
            self.image.as_mut().expect("borrow image").copy_from(&image,
                                                                 pos_x as u32,
                                                                 pos_y as u32);
        }
    }

    pub fn continue_screenshot(&mut self) {
        if self.outputs.len() > 0 {
            self.controller.take_screenshot(&self.outputs.last().expect("get output"));
        } else {
            let path = Path::new(&self.path);

            println!("Writing to {:?}", path);
            match self.image.as_mut().expect("borrow image").save(&path) {
                Ok(_) => println!("Done"),
                Err(err) => println!("Failed: {}", err),
            }
            self.controller.stop();
        }
    }

    pub fn prepare_image(&mut self) {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;

        println!("Outputs:");
        for output in self.outputs.iter() {
            println!(" => {:?}", output);

            let x = output.x;
            if x < left {
                left = x;
            }

            let y = output.y;
            if y < top {
                top = y;
            }

            let x = output.x + output.width as isize;
            if right < x {
                right = x;
            }

            let y = output.y + output.height as isize;
            if bottom < y {
                bottom = y;
            }
        }

        let width = right - left;
        let height = bottom - top;
        let zeros = vec![0; (4 * width * height) as usize];

        println!("Screenshot size: {} x {}", width, height);
        self.image = image::RgbaImage::from_raw(width as u32, height as u32, zeros);
        self.x_offset = -left;
        self.y_offset = -top;
    }
}

// -------------------------------------------------------------------------------------------------

impl Listener for Screenshooter {
    fn globals_done(&mut self, globals: HashSet<String>) {
        println!("Init done");
        for global in vec![GLOBAL_OUTPUT, GLOBAL_SHM, GLOBAL_SCREENSHOOTER] {
            if !globals.contains(global) {
                println!("Server does not provide '{}' global interface", global);
                self.controller.stop();
            }
        }
    }

    fn outputs_done(&mut self, outputs: Vec<OutputInfo>) {
        self.outputs = outputs;
        self.prepare_image();
        self.controller.initialize_screenshoter();
        self.controller.take_screenshot(&self.outputs.last().expect("get output"));
    }

    fn screenshot_done(&mut self, buffer: Vec<u8>) {
        self.handle_screenshot(buffer);
        self.continue_screenshot();
    }

    fn screenshot_failed(&mut self) {
        println!("Screenshot failed");
        self.controller.stop();
    }
}

// -------------------------------------------------------------------------------------------------
