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
extern crate chrono;

extern crate timber;
extern crate cognitive_graphics as graphics;
extern crate cognitive_qualia as qualia;
extern crate cognitive_device_manager as device_manager;

extern crate gears;
extern crate skylane_simple_framework;

mod info;
mod about;
mod screenshot;
mod verify_config;

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
            .about("Takes screenshot")
            .arg(clap::Arg::with_name("path")
                .long("path")
                .help("Sets screenshot path")
                .value_name("PATH")
                .takes_value(true)))
        .subcommand(clap::SubCommand::with_name("verify-config")
            .about("Verifies validity of configurations files(s)"))
        .get_matches();

    match matches.subcommand() {
        ("info", Some(_)) => {
            info::process();
        }
        ("about", Some(_)) => {
            about::process();
        }
        ("screenshot", Some(subcommand)) => {
            if let Some(path) = subcommand.value_of("path") {
                screenshot::process(String::from(path));
            } else {
                let now = chrono::Local::now();
                screenshot::process(now.format("screenshot-%Y-%m-%d_%H%M%S.png").to_string());
            }
        }
        ("verify-config", Some(_)) => {
            verify_config::process();
        }
        _ => println!("Error during parsing arguments!"),
    }
}
