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

//! This module parses Wayland protocol definition in XML file and generates Skylane implementation.

// TODO: Divide this file into more modules.
// TODO: Generate comments along the code to provide documentation.

use std;
use std::fs::File;
use std::path::Path;

use xml;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
#[allow(unreachable_patterns)] // NOTE: Compiler bug: issue #38885
pub enum Error {
    IO(std::io::Error),
    Xml(xml::reader::Error),
    ParseInt(std::num::ParseIntError),
    ParseStr(std::string::ParseError),
    Element(String),
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<xml::reader::Error> for Error {
    fn from(error: xml::reader::Error) -> Self {
        Error::Xml(error)
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

// -------------------------------------------------------------------------------------------------

impl std::convert::From<std::string::ParseError> for Error {
    fn from(error: std::string::ParseError) -> Self {
        Error::ParseStr(error)
    }
}

// -------------------------------------------------------------------------------------------------

/// This enum represents name and type of variable-sized argument of event.
#[derive(Debug)]
enum VariableType {
    String(String),
    Array(String),
}

// -------------------------------------------------------------------------------------------------

/// This enum represents memory size of Wayland types. Some may by constant like `uint` (4 bytes)
/// or variable like string (4 bytes size + contents + padding).
#[derive(Debug)]
enum Size {
    Constant(usize),
    String(usize),
    Array(usize),
}

// -------------------------------------------------------------------------------------------------

/// Helper struct for calculating message size. Constant size is kept in `size` field, while `names`
/// contains names of variable size arguments to be put in generated code.
#[derive(Debug)]
struct MessageSize {
    size: usize,
    names: Vec<VariableType>,
}

// -------------------------------------------------------------------------------------------------

impl MessageSize {
    /// Calculates message size from given arguments including header size.
    pub fn from_arguments(arguments: &Vec<Argument>) -> Self {
        let mut message_size = MessageSize {
            size: 8, // header size
            names: Vec::new(),
        };

        for arg in arguments.iter() {
            match arg.rust_type.get_size() {
                Size::Constant(size) => message_size.size += size,
                Size::String(size) => {
                    message_size.size += size;
                    message_size.names.push(VariableType::String(arg.name.clone()));
                }
                Size::Array(size) => {
                    message_size.size += size;
                    message_size.names.push(VariableType::Array(arg.name.clone()));
                }
            }
        }

        message_size
    }

    /// Checks if message does not contain variable size arguments.
    pub fn is_constant(&self) -> bool {
        self.names.len() == 0
    }

    /// Check if message contains string.
    pub fn contains_string(&self) -> bool {
        for variable_type in &self.names {
            match variable_type {
                &VariableType::String(_) => return true,
                _ => (),
            }
        }
        false
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing main tag of protocol definition XML file.
struct Document {
    description: Text,
    protocol: Text,
    copyright: Text,
    interfaces: Vec<Interface>,
}

// -------------------------------------------------------------------------------------------------

impl Document {
    pub fn new() -> Self {
        Document {
            description: Text::new(None),
            protocol: Text::new(None),
            copyright: Text::new(None),
            interfaces: Vec::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing text tags of protocol definition XML file (like "description" or
/// "copyright").
struct Text {
    text: Option<String>,
}

// -------------------------------------------------------------------------------------------------

impl Text {
    pub fn new(text: Option<String>) -> Self {
        Text { text: text }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "interface" tag of protocol definition XML file.
struct Interface {
    name: String,
    version: u32,
    description: Text,
    requests: Vec<Request>,
    events: Vec<Event>,
    enums: Vec<Enum>,
}

// -------------------------------------------------------------------------------------------------

impl Interface {
    pub fn new(name: String, version: u32) -> Self {
        Interface {
            name: name,
            version: version,
            description: Text::new(None),
            requests: Vec::new(),
            events: Vec::new(),
            enums: Vec::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "request" tag of protocol definition XML file.
struct Request {
    name: String,
    opcode: u16,
    description: Text,
    arguments: Vec<Argument>,
}

// -------------------------------------------------------------------------------------------------

impl Request {
    pub fn new(name: String, opcode: u16) -> Self {
        Request {
            name: name,
            opcode: opcode,
            description: Text::new(None),
            arguments: Vec::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "event" tag of protocol definition XML file.
struct Event {
    name: String,
    opcode: u16,
    description: Text,
    arguments: Vec<Argument>,
}

// -------------------------------------------------------------------------------------------------

impl Event {
    pub fn new(name: String, opcode: u16) -> Self {
        Event {
            name: name,
            opcode: opcode,
            description: Text::new(None),
            arguments: Vec::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "arg" tag of protocol definition XML file.
struct Argument {
    name: String,
    rust_type: RustType,
    description: Text,
}

// -------------------------------------------------------------------------------------------------

impl Argument {
    pub fn new(name: String, rust_type: RustType, description: Option<String>) -> Self {
        Argument {
            name: util::validate_name(&name).to_owned(),
            rust_type: rust_type,
            description: Text::new(description),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "enum" tag of protocol definition XML file.
struct Enum {
    name: String,
    description: Text,
    entries: Vec<Entry>,
}

// -------------------------------------------------------------------------------------------------

impl Enum {
    pub fn new(name: String) -> Self {
        Enum {
            name: name,
            description: Text::new(None),
            entries: Vec::new(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing "entry" tag of protocol definition XML file.
struct Entry {
    name: String,
    value: String,
}

// -------------------------------------------------------------------------------------------------

impl Entry {
    pub fn new(name: String, value: String) -> Self {
        Entry {
            name: name,
            value: value,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Enum describing Wayland protocol wire types. Provides means to convert XML representation to
/// Rust representation for generated code.
#[derive(Clone, Copy, PartialEq)]
enum RustType {
    Object,
    NewId,
    UInt,
    Int,
    Fixed,
    String,
    Array,
    FD,
}

// -------------------------------------------------------------------------------------------------

impl RustType {
    /// Converts "type" identifier from protocol definition to `RustType` enum.
    pub fn from_raw(raw_type: &str) -> Result<RustType, Error> {
        if raw_type == "object" {
            Ok(RustType::Object)
        } else if raw_type == "new_id" {
            Ok(RustType::NewId)
        } else if raw_type == "uint" {
            Ok(RustType::UInt)
        } else if raw_type == "int" {
            Ok(RustType::Int)
        } else if raw_type == "fixed" {
            Ok(RustType::Fixed)
        } else if raw_type == "string" {
            Ok(RustType::String)
        } else if raw_type == "array" {
            Ok(RustType::Array)
        } else if raw_type == "fd" {
            Ok(RustType::FD)
        } else {
            Err(Error::Element(format!("Unknown type '{}'", raw_type)))
        }
    }

    /// Returns Rust representation to be use as input argument type for generated functions.
    pub fn to_input_str(&self) -> &str {
        match *self {
            RustType::Object => "ObjectId",
            RustType::NewId => "ObjectId",
            RustType::UInt => "u32",
            RustType::Int => "i32",
            RustType::Fixed => "f32",
            RustType::String => "&str",
            RustType::Array => "&[u32]",
            RustType::FD => "RawFd",
        }
    }

    /// Returns Rust representation to be use as output argument type for generated functions.
    pub fn to_output_str(&self) -> &str {
        match *self {
            RustType::Object => "ObjectId",
            RustType::NewId => "ObjectId",
            RustType::UInt => "u32",
            RustType::Int => "i32",
            RustType::Fixed => "f32",
            RustType::String => "String",
            RustType::Array => "&[u32]",
            RustType::FD => "RawFd",
        }
    }

    /// Return size in bytes of corresponding Wayland wire type.
    pub fn get_size(&self) -> Size {
        match *self {
            RustType::Object => Size::Constant(4),
            RustType::NewId => Size::Constant(4),
            RustType::UInt => Size::Constant(4),
            RustType::Int => Size::Constant(4),
            RustType::Fixed => Size::Constant(4),
            RustType::String => Size::String(4),
            RustType::Array => Size::Array(4),
            RustType::FD => Size::Constant(4),
        }
    }
}

// -------------------------------------------------------------------------------------------------

mod util {
    use super::{Error, OwnedAttribute};

    /// Searches given mendatory attribute in attribute list and return value.
    pub fn get_attr(name: &str, attributes: &Vec<OwnedAttribute>) -> Result<String, Error> {
        for attr in attributes {
            if attr.name.local_name == name {
                return Ok(attr.value.clone());
            }
        }
        Err(Error::Element(format!("Attribute '{}' not found", name)))
    }

    /// Searches given optional attribute in attribute list and return value.
    pub fn get_optional_attr(name: &str, attributes: &Vec<OwnedAttribute>) -> Option<String> {
        for attr in attributes {
            if attr.name.local_name == name {
                return Some(attr.value.clone());
            }
        }
        None
    }

    /// Appends "_" to tokens which are Rust keywords and trims it from tokens which are keywords in
    /// C/C++ but not in Rust.
    pub fn validate_name(name: &str) -> &str {
        if name == "move" {
            "move_"
        } else if name == "class_" {
            "class"
        } else {
            name
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for building indented code. Keeps string contents and current indent value.
struct Buffer {
    indent: u32,
    content: String,
}

// -------------------------------------------------------------------------------------------------

impl Buffer {
    /// Creates new `Buffer`.
    pub fn new(indent: u32) -> Self {
        Buffer {
            indent: indent,
            content: String::new(),
        }
    }

    /// Returns content string.
    pub fn get_content(&self) -> &String {
        &self.content
    }

    /// Pushes string into contents.
    pub fn push(&mut self, content: &str) -> &mut Buffer {
        self.content.push_str(content);
        self
    }

    /// Pushes string into contents with correct indent and adds new line.
    pub fn push_line(&mut self, line: &str) -> &mut Buffer {
        self.push_indent();
        self.content.push_str(line);
        self.content.push('\n');
        self
    }

    /// Pushes new line into contents;
    pub fn push_end_of_line(&mut self) -> &mut Buffer {
        self.content.push('\n');
        self
    }

    /// Pushes current indent.
    pub fn push_indent(&mut self) -> &mut Buffer {
        for _ in 0..self.indent {
            self.content.push_str("    ");
        }
        self
    }

    /// Increases level in indentation.
    pub fn increase_indent(&mut self) -> &mut Buffer {
        self.indent += 1;
        self
    }

    /// Decreases level in indentation.
    pub fn decrease_indent(&mut self) -> &mut Buffer {
        self.indent -= 1;
        self
    }

    /// Remove `n` characters from the end.
    pub fn pop(&mut self, n: u32) -> &mut Buffer {
        for _ in 0..n {
            self.content.pop();
        }
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// Trait for code generators.
trait Generator {
    fn new(indent: u32) -> Self;
    fn generate(&mut self, document: &Document) -> String;
}

// -------------------------------------------------------------------------------------------------

/// Generator for server-side code.
struct ServerGenerator {
    buffer: Buffer,
}

// -------------------------------------------------------------------------------------------------

impl Generator for ServerGenerator {
    /// Creates new `ServerGenerator`.
    fn new(indent: u32) -> Self {
        ServerGenerator { buffer: Buffer::new(indent) }
    }

    /// Returns generated code as string basing on protocol description.
    fn generate(&mut self, document: &Document) -> String {
        let protocol_name = if let Some(ref protocol_name) = document.protocol.text {
            protocol_name.clone()
        } else {
            "".to_owned()
        };

        for interface in document.interfaces.iter() {
            self.generate_module(&protocol_name, &interface);
        }

        self.buffer.get_content().clone()
    }
}

// -------------------------------------------------------------------------------------------------

impl ServerGenerator {
    fn generate_module(&mut self, protocol_name: &str, interface: &Interface) {
        self.generate_module_start(&interface);
        self.generate_enums(&interface);
        self.generate_interface(&interface);
        self.generate_events(protocol_name, &interface);
        self.generate_dispatcher(protocol_name, &interface);
        self.generate_module_end();
    }

    fn generate_enums(&mut self, interface: &Interface) {
        for e in interface.enums.iter() {
            self.generate_enum_start(&e.name);
            for entry in e.entries.iter() {
                self.generate_entry(&entry);
            }
            self.generate_enum_end();
        }
    }

    fn generate_interface(&mut self, interface: &Interface) {
        self.generate_interface_start();
        self.generate_requests(&interface);
        self.generate_interface_end();
    }

    fn generate_requests(&mut self, interface: &Interface) {
        for request in interface.requests.iter() {
            self.generate_request_start(&request.name);
            for argument in request.arguments.iter() {
                self.generate_output_argument(&argument.name, argument.rust_type);
            }
            self.generate_request_end();
        }
    }

    fn generate_events(&mut self, protocol_name: &str, interface: &Interface) {
        for event in interface.events.iter() {
            self.generate_event_start(&event.name);
            for argument in event.arguments.iter() {
                self.generate_input_argument(&argument);
            }
            self.generate_event_middle();
            self.generate_log(false,
                              &protocol_name,
                              &interface.name,
                              &event.name,
                              &event.arguments);
            self.generate_event_body(&event);
            self.generate_event_end();
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl ServerGenerator {
    fn generate_module_start(&mut self, interface: &Interface) {
        self.buffer
            .push_end_of_line()
            .push_indent()
            .push("pub mod ")
            .push(&interface.name)
            .push(" {")
            .push_end_of_line()
            .increase_indent()
            .push_line("use std;");

        // Imports needed only on special occasions.
        if Self::check_if_needs_raw_fd(interface) {
            self.buffer.push_line("use std::os::unix::io::RawFd;");
        }
        if Self::check_if_needs_write(interface) {
            self.buffer.push_line("use std::io::Write;");
        }
        if Self::check_if_needs_write_bytes_ext(interface) {
            self.buffer.push_line("use byteorder::WriteBytesExt;");
        }
        if Self::check_if_needs_read_bytes_ext(interface) {
            self.buffer.push_line("use byteorder::ReadBytesExt;");
        }

        // Other imports.
        self.buffer
            .push_line("use byteorder::NativeEndian;")
            .push_line("use skylane::common::{SkylaneError, Header, ObjectId};")
            .push_line("use skylane::server::{ClientSocket, Task};")
            .push_line("use super::super::Dispatcher as Disp;")
            .push_end_of_line()
            .push_indent()
            .push("pub const NAME: &'static str = \"")
            .push(&interface.name)
            .push("\";")
            .push_end_of_line()
            .push_indent()
            .push("pub const VERSION: u32 = ")
            .push(&interface.version.to_string())
            .push(";")
            .push_end_of_line();
    }

    fn generate_module_end(&mut self) {
        self.buffer
            .decrease_indent()
            .push_line("}");
    }

    fn generate_enum_start(&mut self, name: &str) {
        self.buffer
            .push_end_of_line()
            .push_indent()
            .push("pub mod ")
            .push(&name)
            .push(" {\n")
            .increase_indent();
    }

    fn generate_entry(&mut self, entry: &Entry) {
        // TODO: Add support for bit fields.

        let mut name = entry.name.to_uppercase();
        if let Some(c) = name.chars().nth(0) {
            if !c.is_alphabetic() {
                name = format!("_{}", name);
            }

            self.buffer
                .push_indent()
                .push("pub const ")
                .push(&name)
                .push(": u32 = ")
                .push(&entry.value)
                .push(";\n");
        }
    }

    fn generate_enum_end(&mut self) {
        self.buffer.decrease_indent().push_line("}");
    }

    fn generate_interface_start(&mut self) {
        self.buffer
            .push_end_of_line()
            .push_line("pub trait Interface {")
            .increase_indent();
    }

    fn generate_interface_end(&mut self) {
        self.buffer
            .decrease_indent()
            .push_line("}");
    }

    fn generate_request_start(&mut self, name: &str) {
        self.buffer
            .push_end_of_line()
            .push_indent()
            .push("fn ")
            .push(util::validate_name(name))
            .push(" (\n")
            .increase_indent()
            .push_line("&mut self,")
            .push_line("this_object_id: ObjectId,")
            .push_line("socket: &mut ClientSocket,");
    }

    fn generate_request_end(&mut self) {
        self.buffer
            .decrease_indent()
            .push_line(") -> Task;");
    }

    fn generate_event_start(&mut self, name: &str) {
        self.buffer
            .push_end_of_line()
            .push_indent()
            .push("pub fn ")
            .push(name)
            .push(" (\n")
            .increase_indent()
            .push_line("_socket: &ClientSocket,")
            .push_line("_id: ObjectId,");

    }

    fn generate_event_middle(&mut self) {
        self.buffer
            .decrease_indent()
            .push_line(") -> Result<(), SkylaneError> {")
            .increase_indent();
    }

    fn generate_event_end(&mut self) {
        self.buffer.decrease_indent().push_line("}");
    }

    fn generate_input_argument(&mut self, argument: &Argument) {
        self.buffer
            .push_indent()
            .push(&argument.name)
            .push(": ")
            .push(argument.rust_type.to_input_str())
            .push(",");
        if let Some(ref description) = argument.description.text {
            self.buffer
                .push(" // ")
                .push(&description);
        }
        self.buffer.push_end_of_line();
    }

    fn generate_output_argument(&mut self, name: &str, rust_type: RustType) {
        self.buffer.push_indent().push(name).push(": ").push(rust_type.to_output_str()).push(",\n");
    }

    fn generate_event_body(&mut self, event: &Event) {
        // Prepare sizes
        let message_size = MessageSize::from_arguments(&event.arguments);

        if message_size.is_constant() {
            self.buffer
                .push_indent()
                .push("let mut _bytes: [u8; ")
                .push(&message_size.size.to_string())
                .push("] = unsafe { std::mem::uninitialized() };")
                .push_end_of_line();
        } else {
            let mut sizes = String::new();
            for variable_type in message_size.names.iter() {
                match *variable_type {
                    VariableType::String(ref name) => {
                        self.buffer
                            .push_indent()
                            .push("let _")
                            .push(name)
                            .push("_len = ")
                            .push(name)
                            .push(".len();")
                            .push_end_of_line()
                            .push_indent()
                            .push("let _")
                            .push(name)
                            .push("_size = (_")
                            .push(name)
                            .push("_len / 4) * 4 + 4;")
                            .push_end_of_line()
                            .push_indent()
                            .push("let _")
                            .push(name)
                            .push("_padding = _")
                            .push(name)
                            .push("_size - _")
                            .push(name)
                            .push("_len;")
                            .push_end_of_line();
                        sizes.push_str(" + _");
                        sizes.push_str(name);
                        sizes.push_str("_size");
                    }
                    VariableType::Array(ref name) => {
                        self.buffer
                            .push_indent()
                            .push("let _")
                            .push(name)
                            .push("_len = ")
                            .push(name)
                            .push(".len();")
                            .push_end_of_line()
                            .push_indent()
                            .push("let _")
                            .push(name)
                            .push("_size = 4 * _")
                            .push(name)
                            .push("_len;")
                            .push_end_of_line()
                            .push_indent();
                        sizes.push_str(" + _");
                        sizes.push_str(name);
                        sizes.push_str("_size");
                    }
                }
            }
            self.buffer
                .push_indent()
                .push("let _msg_len = ")
                .push(&message_size.size.to_string())
                .push(&sizes)
                .push(";")
                .push_end_of_line()
                .push_line("let mut _bytes = Vec::<u8>::with_capacity(_msg_len);")
                .push_line("unsafe { _bytes.set_len(_msg_len); }");
        }

        // Prepare fds
        let fd_count = Self::count_raw_fds(&event);
        if fd_count > 0 {
            self.buffer
                .push_line("let mut _fds_pos = 0;")
                .push_indent()
                .push("let mut _fds: [RawFd; ")
                .push(&fd_count.to_string())
                .push("] = unsafe { std::mem::uninitialized() };")
                .push_end_of_line();
        }

        // Create cursor
        self.buffer.push_line("{").increase_indent();
        if message_size.is_constant() {
            self.buffer.push_line("let mut _bytes_buf = std::io::Cursor::new(&mut _bytes[..]);");
        } else {
            self.buffer.push_line("let mut _bytes_buf = std::io::Cursor::new(_bytes.as_mut_slice());");
        }

        // Header
        self.buffer
            .push_line("_bytes_buf.write_u32::<NativeEndian>(_id.get_value())?;")
            .push_indent()
            .push("_bytes_buf.write_u16::<NativeEndian>(")
            .push(&event.opcode.to_string())
            .push(")?;")
            .push_end_of_line();
        if message_size.is_constant() {
            self.buffer
                .push_indent()
                .push("_bytes_buf.write_u16::<NativeEndian>(")
                .push(&message_size.size.to_string())
                .push(")?;")
                .push_end_of_line();
        } else {
            self.buffer.push_line("_bytes_buf.write_u16::<NativeEndian>(_msg_len as u16)?;");
        }

        // Arguments
        for arg in event.arguments.iter() {
            match arg.rust_type {
                RustType::Object => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_u32::<NativeEndian>(")
                        .push(&arg.name)
                        .push(".get_value())?;")
                        .push_end_of_line();
                }
                RustType::NewId => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_u32::<NativeEndian>(")
                        .push(&arg.name)
                        .push(".get_value())?;")
                        .push_end_of_line();
                }
                RustType::UInt => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_u32::<NativeEndian>(")
                        .push(&arg.name)
                        .push(")?;")
                        .push_end_of_line();
                }
                RustType::Int => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_i32::<NativeEndian>(")
                        .push(&arg.name)
                        .push(")?;")
                        .push_end_of_line();
                }
                RustType::Fixed => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_i32::<NativeEndian>(((")
                        .push(&arg.name)
                        .push(" as f64) * 256.0) as i32")
                        .push(")?;")
                        .push_end_of_line();
                }
                RustType::String => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_u32::<NativeEndian>(_")
                        .push(&arg.name)
                        .push("_len as u32 + 1)?;")
                        .push_end_of_line()
                        .push_indent()
                        .push("_bytes_buf.write(")
                        .push(&arg.name)
                        .push(".as_bytes())?;")
                        .push_end_of_line()
                        .push_indent()
                        .push("for _ in 0..(_")
                        .push(&arg.name)
                        .push("_padding) {")
                        .push_end_of_line()
                        .increase_indent()
                        .push_line("_bytes_buf.write_u8(0)?;")
                        .decrease_indent()
                        .push_line("}");
                }
                RustType::Array => {
                    self.buffer
                        .push_indent()
                        .push("_bytes_buf.write_u32::<NativeEndian>(_")
                        .push(&arg.name)
                        .push("_size as u32)?;")
                        .push_end_of_line()
                        .push_indent()
                        .push("for &_e in ")
                        .push(&arg.name)
                        .push(" {")
                        .push_end_of_line()
                        .increase_indent()
                        .push_line("_bytes_buf.write_u32::<NativeEndian>(_e)?;")
                        .decrease_indent()
                        .push_line("}");
                }
                RustType::FD => {
                    self.buffer
                        .push_indent()
                        .push("_fds[_fds_pos] = ")
                        .push(&arg.name)
                        .push(";")
                        .push_end_of_line()
                        .push_line("_fds_pos += 1;");
                }
            }
        }

        // Write
        self.buffer.decrease_indent().push_line("}");
        if fd_count > 0 {
            self.buffer.push_line("_socket.write_with_control_data(&_bytes[..], &_fds[..])?;");
        } else {
            self.buffer.push_line("_socket.write(&_bytes[..])?;");
        }
        self.buffer.push_line("Ok(())");
    }

    fn generate_dispatcher(&mut self, protocol_name: &str, interface: &Interface) {
        self.buffer
            .push_end_of_line()
            .push_line("pub struct Dispatcher {}")
            .push_end_of_line()
            .push_line("impl<I: Interface> Disp<I> for Dispatcher {")
            .increase_indent()
            .push_line("fn new() -> Self {")
            .increase_indent()
            .push_line("Dispatcher {}")
            .decrease_indent()
            .push_line("}")
            .push_end_of_line()
            .push_line("fn dispatch (")
            .increase_indent()
            .push_line("&mut self,")
            .push_line("_object: &mut I,")
            .push_line("_socket: &mut ClientSocket,")
            .push_line("_header: &Header,")
            .push_line("_bytes_buf: &mut std::io::Cursor<&[u8]>,")
            .push_line("_fds_buf: &mut std::io::Cursor<&[u8]>,")
            .decrease_indent()
            .push_line(") -> Result<Task, SkylaneError> {")
            .increase_indent()
            .push_line("match _header.opcode {")
            .increase_indent();

        for request in interface.requests.iter() {
            self.buffer
                .push_indent()
                .push(&request.opcode.to_string())
                .push(" => {")
                .push_end_of_line()
                .increase_indent();

            // Read in arguments
            for arg in request.arguments.iter() {
                match arg.rust_type {
                    RustType::Object => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = ObjectId::new(_bytes_buf.read_u32::<NativeEndian>()?);\n");
                    }
                    RustType::NewId => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = ObjectId::new(_bytes_buf.read_u32::<NativeEndian>()?);\n");
                    }
                    RustType::UInt => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = _bytes_buf.read_u32::<NativeEndian>()?;\n");
                    }
                    RustType::Int => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = _bytes_buf.read_i32::<NativeEndian>()?;\n");
                    }
                    RustType::Fixed => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = (_bytes_buf.read_i32::<NativeEndian>()? as f64) * \
                                   0.00390625;\n");
                    }
                    RustType::String => {
                        self.buffer
                            .push_indent()
                            .push("let _")
                            .push(&arg.name)
                            .push("_len = _bytes_buf.read_u32::<NativeEndian>()? - 1;\n")
                            .push_indent()
                            .push("let _")
                            .push(&arg.name)
                            .push("_size = (_")
                            .push(&arg.name)
                            .push("_len / 4) * 4 + 4;\n")
                            .push_indent()
                            .push("let _")
                            .push(&arg.name)
                            .push("_padding = _")
                            .push(&arg.name)
                            .push("_size - _")
                            .push(&arg.name)
                            .push("_len;\n")
                            .push_indent()
                            .push("let mut ")
                            .push(&arg.name)
                            .push(" = String::with_capacity(_")
                            .push(&arg.name)
                            .push("_len as usize);\n")
                            .push_indent()
                            .push("for _ in 0 .. _")
                            .push(&arg.name)
                            .push("_len {\n")
                            .increase_indent()
                            .push_line("let c = _bytes_buf.read_u8()?;")
                            .push_indent()
                            .push("if c != 0 { ")
                            .push(&arg.name)
                            .push(".push(c as char) }\n")
                            .decrease_indent()
                            .push_indent()
                            .push("}\n")
                            .push_indent()
                            .push("for _ in 0 .. _")
                            .push(&arg.name)
                            .push("_padding {\n")
                            .increase_indent()
                            .push_line("let _ = _bytes_buf.read_u8()?;")
                            .decrease_indent()
                            .push_indent()
                            .push("}\n");
                    }
                    RustType::Array => {
                        // FIXME: Implement reading array.
                        panic!("Reading array is not yet implemented");
                    }
                    RustType::FD => {
                        self.buffer
                            .push_indent()
                            .push("let ")
                            .push(&arg.name)
                            .push(" = _fds_buf.read_i32::<NativeEndian>()?;\n");
                    }
                }
            }

            // Print log
            self.generate_log(true,
                              &protocol_name,
                              &interface.name,
                              &request.name,
                              &request.arguments);

            // Call handler
            self.buffer
                .push_indent()
                .push("Ok( _object.")
                .push(util::validate_name(&request.name))
                .push(" (")
                .push_end_of_line()
                .increase_indent()
                .push_line("ObjectId::new(_header.object_id),")
                .push_line("_socket,");

            for arg in request.arguments.iter() {
                self.buffer
                    .push_indent()
                    .push(&arg.name)
                    .push(",\n");
            }

            self.buffer
                .decrease_indent()
                .push_line("))")
                .decrease_indent()
                .push_line("}");
        }

        self.buffer
            .push_line("_ => {")
            .increase_indent()
            .push_line("Err(SkylaneError::WrongOpcode { name: NAME, object_id: \
                        _header.object_id, opcode: _header.opcode })")
            .decrease_indent()
            .push_line("}")
            .decrease_indent()
            .push_line("}")
            .decrease_indent()
            .push_line("}")
            .decrease_indent()
            .push_line("}");
    }

    fn generate_log(&mut self,
                    incoming: bool,
                    protocol_name: &str,
                    interface_name: &str,
                    method_name: &str,
                    arguments: &Vec<Argument>) {
        self.buffer
            .push_line("if let Some(_logger) = _socket.get_logger() {")
            .increase_indent()
            .push_indent()
            .push("_logger(format!(\"")
            .push(if incoming { "--> " } else { " <-- " })
            .push(protocol_name)
            .push("::")
            .push(interface_name)
            .push("@{}.")
            .push(method_name)
            .push("(");

        for arg in arguments.iter() {
            self.buffer.push(&arg.name);
            match arg.rust_type {
                RustType::String => self.buffer.push(": '{}', "),
                _ => self.buffer.push(": {:?}, "),
            };
        }

        if arguments.len() > 0 {
            self.buffer.pop(2);
        }

        self.buffer
            .push(")\",\n")
            .increase_indent();

        if incoming {
            self.buffer.push_line("_header.object_id,");
        } else {
            self.buffer.push_line("_id,");
        }

        for arg in arguments.iter() {
            self.buffer
                .push_indent()
                .push(&arg.name)
                .push(",\n");
        }

        self.buffer
            .decrease_indent()
            .push_line("));")
            .decrease_indent()
            .push_line("}");
    }
}

// -------------------------------------------------------------------------------------------------

impl ServerGenerator {
    fn check_if_needs_write(interface: &Interface) -> bool {
        for event in interface.events.iter() {
            let message_size = MessageSize::from_arguments(&event.arguments);
            if message_size.contains_string() {
                return true;
            }
        }
        false
    }

    fn check_if_needs_write_bytes_ext(interface: &Interface) -> bool {
        interface.events.len() > 0
    }

    fn check_if_needs_read_bytes_ext(interface: &Interface) -> bool {
        let mut arg_count = 0;
        for request in interface.requests.iter() {
            arg_count += request.arguments.len();
        }
        arg_count > 0
    }

    fn check_if_needs_raw_fd(interface: &Interface) -> bool {
        for request in interface.requests.iter() {
            for arg in request.arguments.iter() {
                if arg.rust_type == RustType::FD {
                    return true;
                }
            }
        }
        for event in interface.events.iter() {
            for arg in event.arguments.iter() {
                if arg.rust_type == RustType::FD {
                    return true;
                }
            }
        }
        false
    }

    fn count_raw_fds(event: &Event) -> usize {
        let mut count = 0;
        for arg in event.arguments.iter() {
            if arg.rust_type == RustType::FD {
                count += 1;
            }
        }
        count
    }
}

// -------------------------------------------------------------------------------------------------

/// Generator for client-side code.
struct ClientGenerator {
    buffer: Buffer,
}

// -------------------------------------------------------------------------------------------------

// TODO: Finish `ClientGenerator` implementation.
impl Generator for ClientGenerator {
    /// Creates new `ClientGenerator`.
    fn new(indent: u32) -> Self {
        ClientGenerator { buffer: Buffer::new(indent) }
    }

    /// Returns generated code as string basing on protocol description.
    #[allow(unused_variables)] // TODO: Implement generating client code.
    fn generate(&mut self, document: &Document) -> String {
        self.buffer.get_content().clone()
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure parses protocol definition XML into `Document` structure.
struct Aggregator {
    parser: EventReader<File>,
}

// -------------------------------------------------------------------------------------------------

impl Aggregator {
    /// Creates new `Aggregator`.
    pub fn new(parser: EventReader<File>) -> Self {
        Aggregator { parser: parser }
    }

    /// Parses protocol definition into `Document` structure.
    pub fn aggregate(mut self) -> Result<Document, Error> {
        self.aggregate_document()
    }
}

// -------------------------------------------------------------------------------------------------

// TODO: This implementation should be refactored using macros to avoid code duplication.
impl Aggregator {
    /// Aggregates main tag of protocol definition XML file.
    fn aggregate_document(&mut self) -> Result<Document, Error> {
        let mut document = Document::new();
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let n = name.local_name;
                    if n == "interface" {
                        document.interfaces.push(self.aggregate_interface(&attributes)?);
                    } else if n == "description" {
                        document.description = self.aggregate_description()?;
                    } else if n == "protocol" {
                        document.protocol = self.aggregate_protocol(&attributes)?;
                    } else if n == "copyright" {
                        document.copyright = self.aggregate_copyright()?;
                    } else {
                        return Err(Error::Element(format!("Unexpected start of element '{}'", n)));
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return Ok(document);
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "copyright" tag of protocol definition XML file.
    fn aggregate_copyright(&mut self) -> Result<Text, Error> {
        let mut copyright = Text::new(None);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    return Err(Error::Element(format!("Unexpected start of element {}", name)));
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "copyright" {
                        return Ok(copyright);
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::Characters(characters)) => {
                    copyright.text = Some(characters);
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "description" tag of protocol definition XML file.
    fn aggregate_description(&mut self) -> Result<Text, Error> {
        let mut description = Text::new(None);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    return Err(Error::Element(format!("Unexpected start of element {}", name)));
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "description" {
                        return Ok(description);
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::Characters(characters)) => {
                    description.text = Some(characters);
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "interface" tag of protocol definition XML file.
    fn aggregate_interface(&mut self,
                           attributes: &Vec<OwnedAttribute>)
                           -> Result<Interface, Error> {
        let mut interface = Interface::new(util::get_attr("name", &attributes)?,
                                           util::get_attr("version", &attributes)?.parse::<u32>()?);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let n = name.local_name;
                    if n == "request" {
                        let num = interface.requests.len();
                        interface.requests.push(self.aggregate_request(num, &attributes)?);
                    } else if n == "event" {
                        let num = interface.events.len();
                        interface.events.push(self.aggregate_event(num, &attributes)?);
                    } else if n == "enum" {
                        interface.enums.push(self.aggregate_enum(&attributes)?);
                    } else if n == "description" {
                        interface.description = self.aggregate_description()?;
                    } else {
                        return Err(Error::Element(format!("Unexpected start of element '{}'", n)));
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "interface" {
                        return Ok(interface);
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    fn aggregate_request(&mut self,
                         num: usize,
                         attributes: &Vec<OwnedAttribute>)
                         -> Result<Request, Error> {
        let mut request = Request::new(util::get_attr("name", &attributes)?, num as u16);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let n = name.local_name;
                    if n == "arg" {
                        request.arguments.push(self.aggregate_argument(&attributes)?);
                    } else if n == "description" {
                        request.description = self.aggregate_description()?;
                    } else {
                        return Err(Error::Element(format!("Unexpected start of element '{}'", n)));
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "request" {
                        return Ok(request);
                    } else if n == "arg" {
                        // Ignore this
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "event" tag of protocol definition XML file.
    fn aggregate_event(&mut self,
                       num: usize,
                       attributes: &Vec<OwnedAttribute>)
                       -> Result<Event, Error> {
        let mut event = Event::new(util::get_attr("name", &attributes)?, num as u16);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let n = name.local_name;
                    if n == "arg" {
                        event.arguments.push(self.aggregate_argument(&attributes)?);
                    } else if n == "description" {
                        event.description = self.aggregate_description()?;
                    } else {
                        return Err(Error::Element(format!("Unexpected start of element '{}'", n)));
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "event" {
                        return Ok(event);
                    } else if n == "arg" {
                        // Ignore this
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "enum" tag of protocol definition XML file.
    fn aggregate_enum(&mut self, attributes: &Vec<OwnedAttribute>) -> Result<Enum, Error> {
        let mut enumeration = Enum::new(util::get_attr("name", &attributes)?);
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let n = name.local_name;
                    if n == "entry" {
                        enumeration.entries.push(self.aggregate_entry(&attributes)?);
                    } else if n == "description" {
                        enumeration.description = self.aggregate_description()?;
                    } else {
                        return Err(Error::Element(format!("Unexpected start of element '{}'", n)));
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let n = name.local_name;
                    if n == "enum" {
                        return Ok(enumeration);
                    } else if n == "entry" {
                        // Ignore this
                    } else {
                        return Err(Error::Element(format!("Unexpected end of element {}", n)));
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return Err(Error::Element("Unexpected end of document".to_owned()));
                }
                Ok(_) => {
                    // Ignore others
                }
                Err(err) => {
                    return Err(Error::Xml(err));
                }
            }
        }
    }

    /// Aggregates "protocol" attributes of protocol definition XML file.
    fn aggregate_protocol(&self, attributes: &Vec<OwnedAttribute>) -> Result<Text, Error> {
        Ok(Text::new(Some(util::get_attr("name", &attributes)?)))
    }

    /// Aggregates "arg" attributes of protocol definition XML file.
    fn aggregate_argument(&self, attributes: &Vec<OwnedAttribute>) -> Result<Argument, Error> {
        Ok(Argument::new(util::get_attr("name", &attributes)?,
                         RustType::from_raw(&util::get_attr("type", &attributes)?)?,
                         util::get_optional_attr("summary", &attributes)))
    }

    /// Aggregates "entry" attributes of protocol definition XML file.
    fn aggregate_entry(&self, attributes: &Vec<OwnedAttribute>) -> Result<Entry, Error> {
        Ok(Entry::new(util::get_attr("name", &attributes)?,
                      util::get_attr("value", &attributes)?))
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure allows to read in XML protocol definition file and parse it into server- or
/// client-size code.
pub struct Scanner {
    document: Document,
}

// -------------------------------------------------------------------------------------------------

impl Scanner {
    /// Creates new `Scanner`.
    pub fn new(path: &Path) -> Result<Scanner, Error> {
        Ok(Scanner { document: Aggregator::new(EventReader::new(File::open(path)?)).aggregate()? })
    }

    /// Returns name of the protocol.
    pub fn get_protocol_name(&mut self) -> Result<String, Error> {
        if let Some(ref protocol) = self.document.protocol.text {
            Ok(protocol.clone())
        } else {
            Err(Error::Element("Protocol name not found".to_owned()))
        }
    }

    /// Generates server-size code.
    pub fn generate_server_interface(&mut self, indent: u32) -> String {
        ServerGenerator::new(indent).generate(&self.document)
    }

    /// Generates client-size code.
    pub fn generate_client_interface(&mut self, indent: u32) -> String {
        ClientGenerator::new(indent).generate(&self.document)
    }
}

// -------------------------------------------------------------------------------------------------
