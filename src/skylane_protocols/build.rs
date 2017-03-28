// Copyright 2016 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate skylane_scanner;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Read OUT_DIR variable");
    let src_dir = env::current_dir().expect("Get current directory");

    for protocol in vec!["wayland", "xdg-shell-unstable-v6"] {
        let mut src_path = src_dir.clone();
        src_path.push("skylane_protocols");
        src_path.set_file_name(protocol);
        src_path.set_extension("xml");

        let mut scanner =
            skylane_scanner::Scanner::new(&src_path)
                .expect(format!("Initialize scanner for file {:?}", &src_path).as_str());
        let protocol_name = scanner.get_protocol_name().expect("Extract protocol name");

        let mut dst_server_path = PathBuf::new();
        dst_server_path.push(&out_dir);
        dst_server_path.push("out");
        dst_server_path.set_file_name(format!("{}_server", protocol_name));
        dst_server_path.set_extension("rs");

        let mut dst_client_path = PathBuf::new();
        dst_client_path.push(&out_dir);
        dst_client_path.push("out");
        dst_client_path.set_file_name(format!("{}_client", protocol_name));
        dst_client_path.set_extension("rs");

        let mut server_file = File::create(&dst_server_path).expect("Create file");
        server_file.write_all(scanner.generate_server_interface(0).as_bytes())
            .expect(format!("Write to file: {:?}", &dst_server_path).as_str());
        let mut client_file = File::create(&dst_client_path).expect("Create file");
        client_file.write_all(scanner.generate_client_interface(0).as_bytes())
            .expect(format!("Write to file: {:?}", &dst_client_path).as_str());
    }
}
