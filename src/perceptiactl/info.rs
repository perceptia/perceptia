// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

use device_manager;

pub fn process() {
    print_input_devices();
}

fn print_input_devices() {
    let udev = device_manager::udev::Udev::new();
    udev.iterate_event_devices(|devnode, device| {
            println!("{:?}: {:?}", devnode, device_manager::udev::determine_device_kind(&device));
            println!("\tProperties:");
            for p in device.properties() {
                println!("\t\t{:?}: {:?}", p.name(), p.value())
            }
            println!("\tAttributes:");
            for a in device.attributes() {
                if let Some(value) = a.value() {
                    println!("\t\t{:?}: {:?}", a.name(), value);
                }
            }
            println!("");
        })
}

