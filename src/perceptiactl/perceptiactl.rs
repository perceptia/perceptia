// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Controls `perceptia` and provides useful information about system.

extern crate clap;
extern crate nix;
extern crate egl;
extern crate gl;
extern crate libudev;
extern crate drm as libdrm;
extern crate image;

extern crate timber;
extern crate qualia;
extern crate device_manager;
extern crate output;
extern crate renderer_gl;
extern crate skylane_simple_framework;

mod info;
mod about;
mod screenshot;

fn main() {
    timber::init(std::path::Path::new("/dev/null")).unwrap();

    let matches = clap::App::new("perceptiactl")
        .setting(clap::AppSettings::SubcommandRequired)
        .version("0.0.1")
        .author("Wojciech Kluczka <wojciech.kluczka@gmail.com>")
        .about("Controller for Perceptia")
        .subcommand(clap::SubCommand::with_name("info")
            .about("Prints basic information about system."))
        .subcommand(clap::SubCommand::with_name("about")
            .about("Prints information about this program."))
        .subcommand(clap::SubCommand::with_name("screenshot")
            .about("Takes screenshot"))
        .get_matches();

    match matches.subcommand() {
        ("info", Some(_)) => {
            info::process();
        }
        ("about", Some(_)) => {
            about::process();
        }
        ("screenshot", Some(_)) => {
            screenshot::process();
        }
        _ => println!("Error during parsing arguments!"),
    }
}
