// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit test for `dharma::bridge` module.
//!
//! NOTE: these tests use only `Receiver::recv_timeout` instead of `Receiver::recv` to avoid
//! blocking when something goes wrong with assumption that implementation of `Receiver::recv`
//! should work just the same.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

use std::{thread, time};

use self::dharma::ReceiveResult;

// -------------------------------------------------------------------------------------------------

const T: &'static str = "test";
const H1: &'static str = "hello 1";
const H2: &'static str = "hello 2";

// -------------------------------------------------------------------------------------------------

/// Enum used in these test as data type for exchange between `Sender` and 'Receiver'.
#[derive(Clone, PartialEq, Debug)]
enum E {
    A,
    B(i32),
    C(String),
}

// -------------------------------------------------------------------------------------------------

impl dharma::Transportable for E {}

// -------------------------------------------------------------------------------------------------

/// Connect single `Sender` and single `Receiver` and perform `send` once. `Receiver` should receive
/// data once. Second try should indicate empty queue.
#[test]
fn test_one_sender_with_one_receiver_once() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s, &r);

    s.send(E::A);
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Connect single `Sender` and single `Receiver` and perform `send` many times. `Receiver` should
/// receive exact same data as sent. Excess receive tries should indicate empty queue.
#[test]
fn test_one_sender_with_one_receiver_repeatedly() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s, &r);

    s.send(E::A);
    s.send(E::B(3));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    s.send(E::C(String::from(T)));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::C(String::from(T))));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Connect single `Sender` to many `Receiver`s and perform `send` many times. `Receiver`s should
/// receive exact same data as sent independently, that is every receiver receives his own copy of
/// data. Excess receive tries should indicate empty queue.
#[test]
fn test_one_sender_with_two_receivers() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r1 = dharma::Receiver::new();
    let mut r2 = dharma::Receiver::new();
    dharma::connect(&mut s, &r1);
    dharma::connect(&mut s, &r2);

    s.send(E::A);
    s.send(E::B(3));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::A));
    s.send(E::C(String::from(T)));
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::C(String::from(T))));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::C(String::from(T))));
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Connect many `Sender`s to one `Receiver`s and perform `send` many times. `Receiver` should
/// receive data same data as sent.
#[test]
fn test_two_senders_with_one_receiver() {
    let d = time::Duration::new(1, 0);
    let mut s1 = dharma::Sender::new();
    let mut s2 = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s1, &r);
    dharma::connect(&mut s2, &r);

    s1.send(E::A);
    s1.send(E::B(3));
    s2.send(E::A);
    s2.send(E::B(4));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::A));
    s2.send(E::C(String::from(H2)));
    s1.send(E::C(String::from(H1)));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::B(4)));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::C(String::from(H2))));
    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::C(String::from(H1))));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Connect many `Sender`s to many `Receiver`s and perform `send` many times. `Receiver`s should
/// receive exact same data as sent independently, that is every receiver receives his own copy of
/// data. Excess receive tries should indicate empty queue.
#[test]
fn test_many_senders_with_many_receivers() {
    let d = time::Duration::new(1, 0);
    let mut s1 = dharma::Sender::new();
    let mut s2 = dharma::Sender::new();
    let mut r1 = dharma::Receiver::new();
    let mut r2 = dharma::Receiver::new();
    dharma::connect(&mut s1, &r1);
    dharma::connect(&mut s1, &r2);
    dharma::connect(&mut s2, &r1);
    dharma::connect(&mut s2, &r2);

    s1.send(E::A);
    s1.send(E::B(3));
    s2.send(E::A);
    s2.send(E::B(4));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::B(3)));
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::A));
    s2.send(E::C(String::from(H2)));
    s1.send(E::C(String::from(H1)));
    assert_eq!(r1.recv_timeout(d), ReceiveResult::Ok(E::B(4)));
    assert_eq!(r1.recv_timeout(d),
               ReceiveResult::Ok(E::C(String::from(H2))));
    assert_eq!(r2.recv_timeout(d), ReceiveResult::Ok(E::B(4)));
    assert_eq!(r1.recv_timeout(d),
               ReceiveResult::Ok(E::C(String::from(H1))));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.recv_timeout(d),
               ReceiveResult::Ok(E::C(String::from(H2))));
    assert_eq!(r2.recv_timeout(d),
               ReceiveResult::Ok(E::C(String::from(H1))));
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Test sending one piece of data from one `Sender` to one `Receiver` from different thread.
#[test]
fn test_one_sender_with_one_receiver_threaded() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s, &r);

    let join_handle = thread::spawn(move || {
        s.send(E::A);
    });

    assert_eq!(r.recv_timeout(d), ReceiveResult::Ok(E::A));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------
