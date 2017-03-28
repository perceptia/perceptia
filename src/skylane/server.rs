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

//! Server part of Skylane crate.

use std;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Cursor, SeekFrom, Seek};
use std::os::unix::io::RawFd;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use nix;
use nix::sys::socket;
use nix::sys::uio;

use common::{SkylaneError, Header, ObjectId};

// -------------------------------------------------------------------------------------------------

pub type Logger = Option<fn(String) -> ()>;

// -------------------------------------------------------------------------------------------------

/// Return enumeration for callbacks.
///
/// Inside callbacks one often wants to create new objects and add the to `Client`. Since `Client`
/// is owner of all `Objects` it would be cumbersome to add it there. Instead callback return their
/// requests to `Client`.
pub enum Task {
    Create { id: ObjectId, object: Box<Object> },
    Destroy { id: ObjectId },
    None,
}

// -------------------------------------------------------------------------------------------------

/// This trait has to be implemented by all objects to be registered as message handlers in
/// `Client`.
pub trait Object {
    fn dispatch(&mut self,
                socket: &mut ClientSocket,
                header: &Header,
                bytes_buf: &mut Cursor<&[u8]>,
                fds_buf: &mut Cursor<&[u8]>)
                -> Result<Task, SkylaneError>;
}

// -------------------------------------------------------------------------------------------------

/// Structure gathering all information about client. Precesses requests and dispatches them to
/// registered handlers.
pub struct Client {
    socket: ClientSocket,
    objects: HashMap<ObjectId, Box<Object>>,
}

// -------------------------------------------------------------------------------------------------

impl Client {
    pub fn new(socket: ClientSocket) -> Client {
        Client {
            socket: socket,
            objects: HashMap::new(),
        }
    }

    /// Adds new objects. From now client requests to object with given `id` will be passed to this
    /// `objects`. If another object is already assigned to this `id` the assignment will be
    /// overridden.
    ///
    /// Here the only requirement for the object is to implement `Object` trait. In practical use
    /// one will pass implementations of `Interface` traits from protocol definitions wrapped in
    /// `Handler` structure with `Dispatcher` attached as defined in `skylane_protocols` crate.
    pub fn add_object(&mut self, id: ObjectId, object: Box<Object>) {
        self.objects.insert(id, object);
    }

    /// Removes object with given `id`.
    pub fn remove_object(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }

    /// Reads data from socket and dispatches messages to registered objects.
    pub fn process_events(&mut self) -> Result<(), SkylaneError> {
        // TODO: What is more optimal - allocation these buffers here, or in struct? They don't
        // have to be zeroed every time, right? What buffer sizes are enough?
        let mut bytes: [u8; 1024] = [0; 1024];
        let mut fds: [u8; 24] = [0; 24];

        let (bytes_size, _fds_size) = self.socket.receive_message(&mut bytes, &mut fds)?;

        let mut bytes_buf = Cursor::new(&bytes[..]);
        let mut fds_buf = Cursor::new(&fds[..]);

        let mut position = 0;
        while position < bytes_size {
            bytes_buf.seek(SeekFrom::Start(position as u64))?;
            let header = Header {
                object_id: bytes_buf.read_u32::<NativeEndian>()?,
                opcode: bytes_buf.read_u16::<NativeEndian>()?,
                size: bytes_buf.read_u16::<NativeEndian>()?,
            };

            self.process_event(&header, &mut bytes_buf, &mut fds_buf)?;
            position += header.size as usize;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Private methods.
impl Client {
    /// Processes events from client:
    ///
    /// 1. searches for handler
    /// 2. calls `dispatch` method on handler
    /// 3. handles return code from `dispatch`.
    fn process_event(&mut self,
                     header: &Header,
                     mut bytes_buf: &mut Cursor<&[u8]>,
                     mut fds_buf: &mut Cursor<&[u8]>)
                     -> Result<(), SkylaneError> {
        let task = if let Some(handler) = self.objects.get_mut(&ObjectId::new(header.object_id)) {
            handler.dispatch(&mut self.socket, &header, bytes_buf, fds_buf)
        } else {
            Err(SkylaneError::WrongObject { object_id: header.object_id })
        }?;

        match task {
            Task::Create { id, object } => self.add_object(id, object),
            Task::Destroy { id } => self.remove_object(id),
            Task::None => {}
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing connection to client.
#[derive(Clone)]
pub struct ClientSocket {
    fd: RawFd,
    next_serial: std::cell::Cell<u32>,
    logger: Logger,
}

// -------------------------------------------------------------------------------------------------

impl ClientSocket {
    pub fn new(fd: RawFd) -> ClientSocket {
        ClientSocket {
            fd: fd,
            next_serial: std::cell::Cell::new(0),
            logger: None,
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.fd
    }

    pub fn get_next_serial(&self) -> u32 {
        let serial = self.next_serial.get();
        self.next_serial.set(serial + 1);
        serial
    }

    pub fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    pub fn get_logger(&self) -> Logger {
        self.logger
    }

    pub fn receive_message(&self,
                           bytes: &mut [u8],
                           fds: &mut [u8])
                           -> Result<(usize, usize), SkylaneError> {
        let mut cmsg: socket::CmsgSpace<[RawFd; 1]> = socket::CmsgSpace::new();
        let mut iov: [uio::IoVec<&mut [u8]>; 1] = [uio::IoVec::from_mut_slice(&mut bytes[..]); 1];

        let msg = socket::recvmsg(self.fd, &mut iov[..], Some(&mut cmsg), socket::MSG_DONTWAIT)?;

        let mut num_fds = 0;
        let mut buf = Cursor::new(fds);
        for cmsg in msg.cmsgs() {
            match cmsg {
                socket::ControlMessage::ScmRights(newfds) => {
                    buf.write_i32::<NativeEndian>(newfds[0])?;
                    num_fds += 1;
                }
                _ => {}
            }
        }
        Ok((msg.bytes, num_fds))
    }

    pub fn write(&self, bytes: &[u8]) -> Result<(), SkylaneError> {
        let iov: [uio::IoVec<&[u8]>; 1] = [uio::IoVec::from_slice(&bytes[..]); 1];
        let cmsgs: [socket::ControlMessage; 0] = unsafe { std::mem::uninitialized() };

        socket::sendmsg(self.fd, &iov[..], &cmsgs[..], socket::MSG_DONTWAIT, None)?;
        Ok(())
    }

    pub fn write_with_control_data(&self, bytes: &[u8], fds: &[RawFd]) -> Result<(), SkylaneError> {
        let iov: [uio::IoVec<&[u8]>; 1] = [uio::IoVec::from_slice(&bytes[..]); 1];
        let cmsgs = [socket::ControlMessage::ScmRights(fds)];

        socket::sendmsg(self.fd, &iov[..], &cmsgs[..], socket::MSG_DONTWAIT, None)?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure representing global socket. After client connects to this socket `ClientSocket` is
/// created.
#[derive(Clone)]
pub struct DisplaySocket {
    fd: RawFd,
    path: std::path::PathBuf,
}

// -------------------------------------------------------------------------------------------------

/// Helper macro for creating meaningful error reports.
/// NOTE: Would be nice if `nix` put more information in errors.
macro_rules! try_sock {
    ($action:expr, $path:expr, $expr:expr) => {
        match $expr {
            Ok(result) => result,
            Err(err) => {
                return Err(SkylaneError::Other(
                    format!("{} {:?}: {:?}", $action, $path, err.description())
                ));
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl DisplaySocket {
    /// Creates new `DisplaySocket`.
    pub fn new(path: &std::path::Path) -> Result<Self, SkylaneError> {
        let sockfd = try_sock!("Creating",
                               path,
                               socket::socket(socket::AddressFamily::Unix,
                                              socket::SockType::Stream,
                                              socket::SOCK_CLOEXEC,
                                              0));

        let unix_addr = try_sock!("Linking", path, socket::UnixAddr::new(path));
        let sock_addr = socket::SockAddr::Unix(unix_addr);
        try_sock!("Binding", path, socket::bind(sockfd, &sock_addr));
        try_sock!("Listening", path, socket::listen(sockfd, 128));

        Ok(DisplaySocket {
               fd: sockfd,
               path: path.to_owned(),
           })
    }

    /// Creates new `DisplaySocket` on default path.
    ///
    /// Path is created from system variables: `$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY` or
    /// `$XDG_RUNTIME_DIR/wayland-0` if `$WAYLAND_DISPLAY` is not set.
    pub fn new_default() -> Result<Self, SkylaneError> {
        let mut path = std::path::PathBuf::from(std::env::var("XDG_RUNTIME_DIR")?);
        if let Ok(sock) = std::env::var("WAYLAND_DISPLAY") {
            path.push(sock);
        } else {
            path.push("wayland-0");
        }

        Self::new(&path)
    }

    /// Accepts client connection and return new `ClientSocket`.
    pub fn accept(&self) -> Result<ClientSocket, SkylaneError> {
        let fd = socket::accept(self.fd)?;
        Ok(ClientSocket::new(fd))
    }

    /// Returns socket file descriptor.
    pub fn get_fd(&self) -> RawFd {
        self.fd
    }
}

// -------------------------------------------------------------------------------------------------

impl Drop for DisplaySocket {
    fn drop(&mut self) {
        // Remove socket path. Nothing to do with result.
        let _ = nix::unistd::unlink(self.path.as_path());
    }
}

// -------------------------------------------------------------------------------------------------
