// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Verification to validity of configuration files.

use qualia;
use gears::Config;

// -------------------------------------------------------------------------------------------------

pub fn process() {
    verify_config();
}

// -------------------------------------------------------------------------------------------------

/// Verifies validity of configuration files. In case of success prints effective configuration.
/// In case of failure prints error returned by parser.
fn verify_config() {
    let env = qualia::env::Env::create(qualia::LogDestination::Disabled, "perceptia");
    match Config::read(env.get_directories()) {
        Ok(config) => {
            println!("Config valid!");
            println!("{}", config.serialize());
        }
        Err(err) => println!("{}", err),
    }
}

// -------------------------------------------------------------------------------------------------
