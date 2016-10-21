// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides inter process communication via DBUS. This is used to communicate with
//! `logind` to take sessions and devices.

// -------------------------------------------------------------------------------------------------

use dbus::{BusName, BusType, Connection, Interface, Member, Message, MessageItem, Path};
use nix::unistd::getpid;
use std::os::unix::io::RawFd;
use std::sync::{Arc, Mutex};

use errors::Illusion;

// -------------------------------------------------------------------------------------------------

const LOGIN_DESTINATION: &'static str = "org.freedesktop.login1";
const LOGIN_OBJECT_PATH: &'static str = "/org/freedesktop/login1";
const MANAGER_INTERFACE: &'static str = "org.freedesktop.login1.Manager";
const SESSION_INTERFACE: &'static str = "org.freedesktop.login1.Session";

/// Response timeout in milliseconds.
const TIMEOUT: i32 = 1000;

// -------------------------------------------------------------------------------------------------

/// Check if connection is available or return error.
macro_rules! get_connection_or_return {
    ($connection:expr, $result:expr) => {
        match $connection {
            Some(ref connection) => connection,
            None => return $result,
        }
    };
    ($connection:expr) => {
        match $connection {
            Some(ref connection) => connection,
            None => return Err(Illusion::General(format!("No connection to DBUS!"))),
        }
    }
}

/// Check if session object path is available or return error.
macro_rules! get_session_or_return {
    ($session_object_path:expr, $result:expr) => {
        match $session_object_path {
            Some(ref session) => session,
            None => return $result,
        }
    };
    ($session_object_path:expr) => {
        match $session_object_path {
            Some(ref session) => session,
            None => return Err(Illusion::General(format!("Session object path is unknown!"))),
        }
    }
}

/// Check if received reply is correct.
macro_rules! assert_reply {
    ($reply:expr, $result:expr) => {
        match $reply {
            Ok(r) => r,
            Err(_) => return $result,
        }
    };
    ($reply:expr) => {
        match $reply {
            Ok(r) => r,
            Err(err) => {
                return Err(Illusion::General(format!("{}", if err.message().is_some() {
                    err.message().unwrap()
                } else if err.name().is_some() {
                    err.name().unwrap()
                } else {
                    "Unknown error"
                })))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Given device ID return it major and minor number.
/// FIXME: Are these calculations correct?
pub fn major_minor(rdev: u64) -> (u32, u32) {
    let major = (rdev >> 8) as u32;
    let minor = (rdev & ((1_u64 << 8) - 1)) as u32;
    (major, minor)
}

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between `Ipc`s from different threads.
struct InnerIpc {
    connection: Option<Connection>,
    login_destination: BusName<'static>,
    login_object_path: Path<'static>,
    session_object_path: Option<Path<'static>>,
    session_interface: Interface<'static>,
    manager_interface: Interface<'static>,
}

// -------------------------------------------------------------------------------------------------

impl InnerIpc {
    /// `InnerIpc` constructor.
    pub fn new() -> Self {
        let connection = match Connection::get_private(BusType::System) {
            Ok(connection) => Some(connection),
            Err(_) => None,
        };

        InnerIpc {
            connection: connection,
            login_destination: BusName::new(LOGIN_DESTINATION).unwrap(),
            login_object_path: Path::new(LOGIN_OBJECT_PATH).unwrap(),
            session_object_path: None,
            session_interface: Interface::new(SESSION_INTERFACE).unwrap(),
            manager_interface: Interface::new(MANAGER_INTERFACE).unwrap(),
        }
    }

    /// Communicate with `logind` to obtain path to object representing session we are assigned to.
    fn get_session_by_pid(&mut self) -> Option<Path<'static>> {
        let connection = get_connection_or_return!(self.connection, None);
        let message_name = "GetSessionByPID";
        let member = Member::new(message_name).expect(message_name);

        // Prepare message
        let mut message = Message::method_call(&self.login_destination,
                                               &self.login_object_path,
                                               &self.manager_interface,
                                               &member);
        message.append_items(&[MessageItem::UInt32(getpid() as u32)]);

        // Send message and get result
        let r = assert_reply!(connection.send_with_reply_and_block(message, TIMEOUT), None);
        match r.get1().expect(message_name) {
            MessageItem::ObjectPath(path) => Some(path),
            _ => None,
        }
    }

    /// Communicate to `logind` to take control over session.
    fn take_control(&mut self) -> Result<(), Illusion> {
        let connection = get_connection_or_return!(self.connection);
        let session_object_path = get_session_or_return!(self.session_object_path);
        let message_name = "TakeControl";
        let member = Member::new(message_name).unwrap();

        // Prepare message
        let mut message = Message::method_call(&self.login_destination,
                                               &session_object_path,
                                               &self.session_interface,
                                               &member);
        message.append_items(&[false.into()]);

        // Send message and get result
        assert_reply!(connection.send_with_reply_and_block(message, TIMEOUT));
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure proving access to interprocess communication.
pub struct Ipc {
    inner: Arc<Mutex<InnerIpc>>,
}

// -------------------------------------------------------------------------------------------------

impl Ipc {
    /// `Ipc` constructor. Connects to DBUS.
    pub fn new() -> Self {
        Ipc { inner: Arc::new(Mutex::new(InnerIpc::new())) }
    }

    /// Take control over session.
    pub fn initialize(&mut self) -> Result<(), Illusion> {
        let mut mine = self.inner.lock().unwrap();
        mine.session_object_path = mine.get_session_by_pid();
        mine.take_control()
    }

    /// Communicate to `logind` to take control over given device.
    pub fn take_device(&self, rdev: u64) -> Result<RawFd, Illusion> {
        let mine = self.inner.lock().unwrap();
        let connection = get_connection_or_return!(mine.connection);
        let session_object_path = get_session_or_return!(mine.session_object_path);
        let message_name = "TakeDevice";
        let member = Member::new(message_name).unwrap();

        // Prepare message
        let mut message = Message::method_call(&mine.login_destination,
                                               &session_object_path,
                                               &mine.session_interface,
                                               &member);
        let (major, minor) = major_minor(rdev);
        message.append_items(&[major.into(), minor.into()]);

        // Send message and get result
        let r = assert_reply!(connection.send_with_reply_and_block(message, TIMEOUT));
        match r.get1().expect(message_name) {
            MessageItem::UnixFd(fd) => Ok(fd.into_fd()),
            _ => Err(Illusion::Unknown(format!("Received wrong answer!"))),
        }
    }
}

// -------------------------------------------------------------------------------------------------
