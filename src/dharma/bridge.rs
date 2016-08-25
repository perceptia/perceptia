// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module implements generic communication between two points (possibly two threads) in
//! single-producer multi-consumer style. `Sender` constitutes input point while `Receiver`
//! constitutes output point. `Sender`s and `Receiver`s can be freely binded with `connect`
//! function, so single `Sender` may send to many `Receiver`s and single `Receiver` may receive from
//! many `Sender`s.
//!
//! # Example
//!
//! ```
//! use dharma::{connect, Sender, Receiver, ReceiveResult, Transportable};
//!
//! #[derive(Clone, PartialEq, Debug)]
//! enum E { A, B, C, D }
//! impl Transportable for E {}
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
//!         s1.send(E::A);
//!         s1.send(E::B);
//!         s2.send(E::C);
//!         s2.send(E::D);
//!     });
//!
//! assert_eq!(r1.recv(), ReceiveResult::Ok(E::A));
//! assert_eq!(r2.recv(), ReceiveResult::Ok(E::A));
//! assert_eq!(r1.recv(), ReceiveResult::Ok(E::B));
//! assert_eq!(r2.recv(), ReceiveResult::Ok(E::B));
//! assert_eq!(r1.try_recv(), ReceiveResult::Empty);
//! assert_eq!(r2.recv(), ReceiveResult::Ok(E::C));
//! assert_eq!(r2.recv(), ReceiveResult::Ok(E::D));
//! assert_eq!(r2.try_recv(), ReceiveResult::Empty);
//! ```

// -------------------------------------------------------------------------------------------------

use std::clone::Clone;
use std::collections::VecDeque as Fifo;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

// -------------------------------------------------------------------------------------------------

/// All transported data must derive from this trait. This ensures we can move it between threads
/// and copy.
pub trait Transportable: Send + Clone {}

impl Transportable for String {}

// -------------------------------------------------------------------------------------------------

/// Result returned from `Receiver::recv`, `Receiver::try_recv` or `Receiver::recv_timeout`.
#[derive(Clone, PartialEq, Debug)]
pub enum ReceiveResult<T: Transportable> {
    /// Everything went OK. Data is ready.
    Ok(T),
    /// Queue was empty.
    Empty,
    /// Wait timed out.
    Timeout,
    /// Unknown error.
    Err,
}

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between `Bridge`s from different threads.
struct InnerBridge<T: Transportable> {
    fifo: Fifo<T>,
}

// -------------------------------------------------------------------------------------------------

/// Structure binding `Sender` and `Receiver`. Connected `Sender` and `Receiver` share common
/// `Bridge`.
///
/// Bridge implements simple FIFO queue on which `Sender` pushes data and `Receiver` pops it.
struct Bridge<T: Transportable> {
    inner: Arc<Mutex<InnerBridge<T>>>,
    on_ready: Arc<Condvar>,
}

// -------------------------------------------------------------------------------------------------

/// All function in this `impl` intentionally panic if when error occurred using mutex or
/// conditional value.
impl<T: Transportable> Bridge<T> {
    /// Bridge constructor.
    pub fn new() -> Self {
        Bridge {
            inner: Arc::new(Mutex::new(InnerBridge { fifo: Fifo::with_capacity(10) })),
            on_ready: Arc::new(Condvar::new()),
        }
    }

    /// Push new data to FIFO queue.
    fn push(&mut self, package: T) {
        let mut mine = self.inner.lock().unwrap();
        mine.fifo.push_back(package.clone());
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
    fn process(fifo: &mut Fifo<T>) -> ReceiveResult<T> {
        match fifo.pop_front() {
            Some(package) => ReceiveResult::Ok(package),
            None => ReceiveResult::Err,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Implement clone: construct new object pointing to reference counted shared memory on heap.
impl<T: Transportable> Clone for Bridge<T> {
    fn clone(&self) -> Bridge<T> {
        Bridge {
            inner: self.inner.clone(),
            on_ready: self.on_ready.clone(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Allows sending data to `Receiver`s.
pub struct Sender<T: Transportable> {
    bridges: Vec<Bridge<T>>,
}

// -------------------------------------------------------------------------------------------------

impl<T: Transportable> Sender<T> {
    /// Sender constructor.
    pub fn new() -> Self {
        Sender { bridges: Vec::new() }
    }

    /// Send passed data package to all connected `Receiver`s.
    pub fn send(&mut self, package: T) {
        for b in self.bridges.iter_mut() {
            b.push(package.clone());
        }
    }

    /// Helper function to connect to `Receiver` with `Bridge`.
    fn add_bridge(&mut self, bridge: Bridge<T>) {
        self.bridges.push(bridge);
    }
}

// -------------------------------------------------------------------------------------------------

/// Allows receiving data from `Sender`s.
pub struct Receiver<T: Transportable> {
    bridge: Bridge<T>,
}

// -------------------------------------------------------------------------------------------------

impl<T: Transportable> Receiver<T> {
    /// Receiver constructor.
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
    use bridge::ReceiveResult;
    use bridge::Transportable;

    #[derive(Clone, PartialEq, Debug)]
    enum F {
        A,
        B(i32),
    }
    impl Transportable for F {}

    /// Get bridge from `Receiver` and check if `Receiver` will be able to receive data pushed
    /// once.
    #[test]
    fn test_receiver_one_push_receive() {
        let d = time::Duration::new(1, 0);
        let mut r = super::Receiver::new();
        let mut b = r.get_bridge();
        b.push(F::A);
        assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(F::A));
        assert_eq!(r.try_recv(), ReceiveResult::Empty);
    }

    /// Get bridge from `Receiver` and check if `Receiver` will be able to receive data pushed many
    /// times.
    #[test]
    fn test_receiver_push_receive_in_order() {
        let d = time::Duration::new(1, 0);
        let mut r = super::Receiver::new();
        let mut b = r.get_bridge();
        b.push(F::B(1));
        b.push(F::B(2));
        b.push(F::B(3));
        assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(F::B(1)));
        assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(F::B(2)));
        assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(F::B(3)));
        assert_eq!(r.try_recv(), ReceiveResult::Empty);
    }
}

// -------------------------------------------------------------------------------------------------

/// Connect given `Sender` to given `Receiver`. One can freely connect many `Sender`'s to many
/// `Receiver`'s.
pub fn connect<T: Transportable>(sender: &mut Sender<T>, receiver: &Receiver<T>) {
    sender.add_bridge(receiver.get_bridge());
}

// -------------------------------------------------------------------------------------------------
