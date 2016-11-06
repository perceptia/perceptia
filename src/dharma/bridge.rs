// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module implements generic communication between two points (possibly two threads) in
//! single-producer multi-consumer style. `Sender` constitutes input point while `Receiver`
//! constitutes output point. `Sender`s and `Receiver`s can be freely binded with `connect`
//! function, so single `Sender` may send to many `Receiver`s and single `Receiver` may receive from
//! many `Sender`s.
//!
//! For cases when it is needed to ensure that only one `Receiver` is connected one should use
//! `DirectSender` instead of `Sender`.
//!
//! # Example
//!
//! ```
//! use dharma::{connect, Sender, Receiver, ReceiveResult};
//!
//! #[derive(Clone, PartialEq, Debug)]
//! enum E { A, B, C, D }
//!
//! let mut s1 = Sender::new();
//! let mut s2 = Sender::new();
//! let mut r1 = Receiver::new();
//! let mut r2 = Receiver::new();
//! connect(&mut s1, &r1);
//! connect(&mut s1, &r2);
//! connect(&mut s2, &r2);
//!
//! std::thread::spawn(move || {
//!         s1.send_plain(E::A);
//!         s1.send_plain(E::B);
//!         s2.send_plain(E::C);
//!         s2.send_plain(E::D);
//!     });
//!
//! assert!(r1.recv().is_plain(E::A));
//! assert!(r2.recv().is_plain(E::A));
//! assert!(r1.recv().is_plain(E::B));
//! assert!(r2.recv().is_plain(E::B));
//! assert!(r1.try_recv().is_empty());
//! assert!(r2.recv().is_plain(E::C));
//! assert!(r2.recv().is_plain(E::D));
//! assert!(r2.try_recv().is_empty());
//! ```

// -------------------------------------------------------------------------------------------------

use std::any::Any;
use std::clone::Clone;
use std::collections::VecDeque as Fifo;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

// -------------------------------------------------------------------------------------------------

/// Type alias for signal IDs.
pub type SignalId = usize;

// -------------------------------------------------------------------------------------------------

/// Enumeration for special type of command.
#[derive(Debug, PartialEq)]
pub enum SpecialCommand {
    /// Request termination on receiver side.
    Terminate,
}

// -------------------------------------------------------------------------------------------------

/// Container for stored events.
enum Container<T>
    where T: Clone + Send
{
    /// Simple, plain package with no ID.
    Plain(T),
    /// Package with integer ID.
    Defined(SignalId, T),
    /// Package with custom string ID.
    Custom(&'static str, T),
    /// For use with `DirectSender` when copying is not allowed.
    Any(String, Box<Any + Send>),
    /// Special predefined command.
    Special(SpecialCommand),
}

// -------------------------------------------------------------------------------------------------

/// Result returned from `Receiver::recv`, `Receiver::try_recv` or `Receiver::recv_timeout`.
#[derive(Debug)]
pub enum ReceiveResult<T>
    where T: Clone + Send
{
    /// Successful receive with plain variant
    Plain(T),
    /// Successful receive with defined variant
    Defined(SignalId, T),
    /// Successful receive with custom variant
    Custom(&'static str, T),
    /// Successful receive with any variant
    Any(String, Box<Any + Send>),
    /// Successful receive with special variant
    Special(SpecialCommand),
    /// Queue was empty.
    Empty,
    /// Wait timed out.
    Timeout,
    /// Unknown error.
    Err,
}

// -------------------------------------------------------------------------------------------------

/// Comparing `ReceiveResult`. This is needed because `Any` can not be compared directly.
impl<T> ReceiveResult<T>
    where T: Clone + Send + PartialEq
{
    /// Returns true if `ReceiveResult` is `Plain` with given parameters.
    pub fn is_plain(&self, pkg1: T) -> bool {
        match self {
            &ReceiveResult::Plain(ref pkg2) => &pkg1 == pkg2,
            _ => false,
        }
    }

    /// Returns true if `ReceiveResult` is `Defined` with given parameters.
    pub fn is_defined(&self, id1: SignalId, pkg1: T) -> bool {
        match self {
            &ReceiveResult::Defined(id2, ref pkg2) => (id1 == id2) && (&pkg1 == pkg2),
            _ => false,
        }
    }

    /// Returns true if `ReceiveResult` is `Custom` with given parameters.
    pub fn is_custom(&self, id1: &'static str, pkg1: T) -> bool {
        match self {
            &ReceiveResult::Custom(id2, ref pkg2) => (id1 == id2) && (&pkg1 == pkg2),
            _ => false,
        }
    }

    /// Returns true if `ReceiveResult` is `Special` with given parameters.
    pub fn is_special(&self, cmd1: SpecialCommand) -> bool {
        match self {
            &ReceiveResult::Special(ref cmd2) => &cmd1 == cmd2,
            _ => false,
        }
    }

    /// Returns true if `ReceiveResult` is `Empty`.
    pub fn is_empty(&self) -> bool {
        match self {
            &ReceiveResult::Empty => true,
            _ => false,
        }
    }

    /// Returns true if `ReceiveResult` is `Timeout`.
    pub fn is_timeout(&self) -> bool {
        match self {
            &ReceiveResult::Timeout => true,
            _ => false,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between `Bridge`s from different threads.
struct InnerBridge<T>
    where T: Clone + Send
{
    fifo: Fifo<Container<T>>,
}

// -------------------------------------------------------------------------------------------------

/// Structure binding `Sender` and `Receiver`. Connected `Sender` and `Receiver` share common
/// `Bridge`.
///
/// Bridge implements simple FIFO queue on which `Sender` pushes data and `Receiver` pops it.
struct Bridge<T>
    where T: Clone + Send
{
    inner: Arc<Mutex<InnerBridge<T>>>,
    on_ready: Arc<Condvar>,
}

// -------------------------------------------------------------------------------------------------

/// All function in this `impl` intentionally panic when error occurred using mutex or conditional
/// value.
impl<T> Bridge<T>
    where T: Clone + Send
{
    /// `Bridge` constructor.
    pub fn new() -> Self {
        Bridge {
            inner: Arc::new(Mutex::new(InnerBridge { fifo: Fifo::with_capacity(10) })),
            on_ready: Arc::new(Condvar::new()),
        }
    }

    /// Push new data to FIFO queue.
    fn push(&mut self, container: Container<T>) {
        let mut mine = self.inner.lock().unwrap();
        mine.fifo.push_back(container);
        self.on_ready.notify_one();
    }

    /// Try take data from queue.
    /// If queue is empty return `ReceiveResult::Empty`.
    pub fn try_take(&mut self) -> ReceiveResult<T> {
        let mut mine = self.inner.lock().unwrap();

        if mine.fifo.len() == 0 {
            ReceiveResult::Empty
        } else {
            Self::process(&mut mine.fifo)
        }
    }

    /// Take data from queue. If queue is empty, wait for data to be available.
    pub fn take(&mut self) -> ReceiveResult<T> {
        let mut mine = self.inner.lock().unwrap();
        if mine.fifo.len() == 0 {
            mine = self.on_ready.wait(mine).unwrap();
        }

        Self::process(&mut mine.fifo)
    }

    /// Take data from queue. If queue is empty, wait for data to be available for given amount of
    /// time. If time went out return `ReceiveResult::Timeout`.
    pub fn take_timeout(&mut self, duration: Duration) -> ReceiveResult<T> {
        let mut mine = self.inner.lock().unwrap();
        if mine.fifo.len() == 0 {
            let pair = self.on_ready.wait_timeout(mine, duration).unwrap();
            if pair.1.timed_out() {
                return ReceiveResult::Timeout;
            } else {
                mine = pair.0;
            }
        }

        Self::process(&mut mine.fifo)
    }

    /// Helper function for processing output data.
    fn process(fifo: &mut Fifo<Container<T>>) -> ReceiveResult<T> {
        match fifo.pop_front() {
            Some(container) => {
                match container {
                    Container::Plain(package) => ReceiveResult::Plain(package),
                    Container::Defined(id, package) => ReceiveResult::Defined(id, package),
                    Container::Custom(id, package) => ReceiveResult::Custom(id, package),
                    Container::Any(id, package) => ReceiveResult::Any(id, package),
                    Container::Special(id) => ReceiveResult::Special(id),
                }
            }
            None => ReceiveResult::Err,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Implement clone: construct new object pointing to reference counted shared memory on heap.
impl<T> Clone for Bridge<T>
    where T: Clone + Send
{
    fn clone(&self) -> Bridge<T> {
        Bridge {
            inner: self.inner.clone(),
            on_ready: self.on_ready.clone(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Allows sending data to one `Receiver`.
pub struct DirectSender<T>
    where T: Clone + Send
{
    bridge: Option<Bridge<T>>,
}

// -------------------------------------------------------------------------------------------------

impl<T> DirectSender<T>
    where T: Clone + Send
{
    /// `DirectSender` constructor.
    pub fn new() -> Self {
        DirectSender { bridge: None }
    }

    /// Sends plain package variant.
    pub fn send_plain(&mut self, package: T) {
        if let Some(ref mut bridge) = self.bridge {
            bridge.push(Container::Plain(package));
        }
    }

    /// Sends defined package variant.
    pub fn send_defined(&mut self, id: SignalId, package: T) {
        if let Some(ref mut bridge) = self.bridge {
            bridge.push(Container::Defined(id, package));
        }
    }

    /// Sends custom package variant.
    pub fn send_custom(&mut self, id: &'static str, package: T) {
        if let Some(ref mut bridge) = self.bridge {
            bridge.push(Container::Custom(id, package));
        }
    }

    /// Sends any variant.
    pub fn send_any(&mut self, id: String, package: Box<Any + Send>) {
        if let Some(ref mut bridge) = self.bridge {
            bridge.push(Container::Any(id, package));
        }
    }

    /// Sends special variant.
    pub fn send_special(&mut self, id: SpecialCommand) {
        if let Some(ref mut bridge) = self.bridge {
            bridge.push(Container::Special(id));
        }
    }

    /// Helper function to connect to `Receiver` with `Bridge`.
    /// This function will fail override current bridge.
    fn set_bridge(&mut self, bridge: Bridge<T>) {
        self.bridge = Some(bridge)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T> Clone for DirectSender<T>
    where T: Clone + Send
{
    fn clone(&self) -> Self {
        let mut new = DirectSender::new();
        if let Some(ref bridge) = self.bridge {
            new.set_bridge(bridge.clone())
        }
        new
    }
}

// -------------------------------------------------------------------------------------------------

/// Allows sending data to many `Receiver`s.
pub struct Sender<T>
    where T: Clone + Send
{
    bridges: Vec<Bridge<T>>,
}

// -------------------------------------------------------------------------------------------------

impl<T> Sender<T>
    where T: Clone + Send
{
    /// `Sender` constructor.
    pub fn new() -> Self {
        Sender { bridges: Vec::new() }
    }

    /// Sends passed data package to all connected `Receiver`s.
    pub fn send_plain(&mut self, package: T) {
        for b in self.bridges.iter_mut() {
            b.push(Container::Plain(package.clone()));
        }
    }

    /// Sends passed data package to all connected `Receiver`s.
    pub fn send_defined(&mut self, id: SignalId, package: T) {
        for b in self.bridges.iter_mut() {
            b.push(Container::Defined(id, package.clone()));
        }
    }

    /// Sends passed data package to all connected `Receiver`s.
    pub fn send_custom(&mut self, id: &'static str, package: T) {
        for b in self.bridges.iter_mut() {
            b.push(Container::Custom(id, package.clone()));
        }
    }

    /// Helper function to connect to `Receiver` with `Bridge`.
    fn add_bridge(&mut self, bridge: Bridge<T>) {
        self.bridges.push(bridge);
    }
}

// -------------------------------------------------------------------------------------------------

/// Allows receiving data from `Sender`s.
pub struct Receiver<T>
    where T: Clone + Send
{
    bridge: Bridge<T>,
}

// -------------------------------------------------------------------------------------------------

impl<T> Receiver<T>
    where T: Clone + Send
{
    /// `Receiver` constructor.
    pub fn new() -> Self {
        Receiver { bridge: Bridge::new() }
    }

    /// Try receive data from queue.
    /// If queue is empty return `ReceiveResult::Empty`.
    pub fn try_recv(&mut self) -> ReceiveResult<T> {
        self.bridge.try_take()
    }

    /// Receive data from `Sender`. If queue is empty, wait for data to be available.
    #[inline]
    pub fn recv(&mut self) -> ReceiveResult<T> {
        self.bridge.take()
    }

    /// Receive data from `Sender`. If queue is empty, wait for data to be available for given
    /// amount of time. If time went out return `ReceiveResult::Timeout`.
    #[inline]
    pub fn recv_timeout(&mut self, duration: Duration) -> ReceiveResult<T> {
        self.bridge.take_timeout(duration)
    }

    /// Get clone of the `Receiver`'s `Bridge`.
    /// Helper function to connect to `Sender`.
    #[inline]
    fn get_bridge(&self) -> Bridge<T> {
        self.bridge.clone()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test_receiver {
    use std::time;
    use bridge::Container;

    #[derive(Clone, PartialEq, Debug)]
    enum F {
        A,
        B(i32),
    }

    /// Get bridge from `Receiver` and check if `Receiver` will be able to receive data pushed
    /// once.
    #[test]
    fn test_receiver_one_push_receive() {
        let d = time::Duration::new(1, 0);
        let mut r = super::Receiver::new();
        let mut b = r.get_bridge();
        b.push(Container::Plain(F::A));
        assert!(r.recv_timeout(d).is_plain(F::A));
        assert!(r.try_recv().is_empty());
    }

    /// Get bridge from `Receiver` and check if `Receiver` will be able to receive data pushed many
    /// times.
    #[test]
    fn test_receiver_push_receive_in_order() {
        let d = time::Duration::new(1, 0);
        let mut r = super::Receiver::new();
        let mut b = r.get_bridge();
        b.push(Container::Plain(F::B(4)));
        b.push(Container::Plain(F::B(5)));
        b.push(Container::Plain(F::B(6)));
        assert!(r.recv_timeout(d).is_plain(F::B(4)));
        assert!(r.recv_timeout(d).is_plain(F::B(5)));
        assert!(r.recv_timeout(d).is_plain(F::B(6)));
        assert!(r.try_recv().is_empty());
    }
}

// -------------------------------------------------------------------------------------------------

/// Connect given `DirectSender` to given `Receiver`. One can connect `DirectSender` to only one
/// `Receiver`. If `Sender` already has a `Receiver` the connection will be overridden.
pub fn direct_connect<T>(sender: &mut DirectSender<T>, receiver: &Receiver<T>)
    where T: Clone + Send
{
    sender.set_bridge(receiver.get_bridge());
}

// -------------------------------------------------------------------------------------------------

/// Connect given `Sender` to given `Receiver`. One can freely connect many `Sender`'s to many
/// `Receiver`'s.
pub fn connect<T>(sender: &mut Sender<T>, receiver: &Receiver<T>)
    where T: Clone + Send
{
    sender.add_bridge(receiver.get_bridge());
}

// -------------------------------------------------------------------------------------------------
