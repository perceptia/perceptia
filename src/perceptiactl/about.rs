// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Printing useful information about `perceptia`.

const ABOUT: &'static str = r#"
Perceptia - dynamic window manager with support for Wayland

Source code:
   https://github.com/perceptia/perceptia/

Bug tracker:
    https://github.com/perceptia/perceptia/issues

Configuration:
    https://github.com/perceptia/perceptia/blob/master/info/configuration.md

Contributors:
    https://github.com/perceptia/perceptia/blob/master/info/authors.md

Mailing list:
    perceptia@freelists.org
"#;

pub fn process() {
    println!("{}", ABOUT);
}
