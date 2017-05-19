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

/// Connect single `Sender` and single `Receiver` and perform `send` once. `Receiver` should receive
/// data once. Second try should indicate empty queue.
#[test]
fn test_one_sender_with_one_receiver_once() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s, &r);

    s.send_plain(E::A);
    assert!(r.recv_timeout(d).is_plain(E::A));
    assert!(r.try_recv().is_empty());
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

    s.send_plain(E::A);
    s.send_plain(E::B(3));
    assert!(r.recv_timeout(d).is_plain(E::A));
    assert!(r.recv_timeout(d).is_plain(E::B(3)));
    assert!(r.try_recv().is_empty());
    s.send_plain(E::C(String::from(T)));
    assert!(r.recv_timeout(d).is_plain(E::C(String::from(T))));
    assert!(r.try_recv().is_empty());
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

    s.send_plain(E::A);
    s.send_plain(E::B(3));
    assert!(r1.recv_timeout(d).is_plain(E::A));
    assert!(r1.recv_timeout(d).is_plain(E::B(3)));
    assert!(r1.try_recv().is_empty());
    assert!(r2.recv_timeout(d).is_plain(E::A));
    s.send_plain(E::C(String::from(T)));
    assert!(r2.recv_timeout(d).is_plain(E::B(3)));
    assert!(r1.recv_timeout(d).is_plain(E::C(String::from(T))));
    assert!(r1.try_recv().is_empty());
    assert!(r2.recv_timeout(d).is_plain(E::C(String::from(T))));
    assert!(r2.try_recv().is_empty());
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

    s1.send_plain(E::A);
    s1.send_plain(E::B(3));
    s2.send_plain(E::A);
    s2.send_plain(E::B(4));
    assert!(r.recv_timeout(d).is_plain(E::A));
    assert!(r.recv_timeout(d).is_plain(E::B(3)));
    assert!(r.recv_timeout(d).is_plain(E::A));
    s2.send_plain(E::C(String::from(H2)));
    s1.send_plain(E::C(String::from(H1)));
    assert!(r.recv_timeout(d).is_plain(E::B(4)));
    assert!(r.recv_timeout(d).is_plain(E::C(String::from(H2))));
    assert!(r.recv_timeout(d).is_plain(E::C(String::from(H1))));
    assert!(r.try_recv().is_empty());
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

    s1.send_plain(E::A);
    s1.send_plain(E::B(3));
    s2.send_plain(E::A);
    s2.send_plain(E::B(4));
    assert!(r1.recv_timeout(d).is_plain(E::A));
    assert!(r1.recv_timeout(d).is_plain(E::B(3)));
    assert!(r1.recv_timeout(d).is_plain(E::A));
    assert!(r2.recv_timeout(d).is_plain(E::A));
    assert!(r2.recv_timeout(d).is_plain(E::B(3)));
    assert!(r2.recv_timeout(d).is_plain(E::A));
    s2.send_plain(E::C(String::from(H2)));
    s1.send_plain(E::C(String::from(H1)));
    assert!(r1.recv_timeout(d).is_plain(E::B(4)));
    assert!(r1.recv_timeout(d).is_plain(E::C(String::from(H2))));
    assert!(r2.recv_timeout(d).is_plain(E::B(4)));
    assert!(r1.recv_timeout(d).is_plain(E::C(String::from(H1))));
    assert!(r1.try_recv().is_empty());
    assert!(r2.recv_timeout(d).is_plain(E::C(String::from(H2))));
    assert!(r2.recv_timeout(d).is_plain(E::C(String::from(H1))));
    assert!(r2.try_recv().is_empty());
}

// -------------------------------------------------------------------------------------------------

/// Connect two `DirectSender`s to `Receiver` and perform send, then reset `Receiver`. Old
/// `Receiver` should not be able to receive anything from reseted `DirectSender` but still be
/// connected to not reseted. All messages should be received even after disconnection.
#[test]
fn test_two_direct_senders_with_one_receiver() {
    let d = time::Duration::new(1, 0);
    let mut s1 = dharma::DirectSender::new();
    let mut s2 = dharma::DirectSender::new();
    let mut r1 = dharma::Receiver::new();
    let mut r2 = dharma::Receiver::new();
    dharma::direct_connect(&mut s1, &r1);
    dharma::direct_connect(&mut s2, &r1);

    s1.send_plain(E::B(1));
    s2.send_plain(E::B(2));
    assert!(r1.recv_timeout(d).is_plain(E::B(1)));
    assert!(r1.recv_timeout(d).is_plain(E::B(2)));
    assert!(r1.try_recv().is_empty());

    dharma::direct_connect(&mut s1, &r2);

    s1.send_plain(E::B(3));
    s2.send_plain(E::B(4));
    s2.send_plain(E::B(5));
    assert!(r2.recv_timeout(d).is_plain(E::B(3)));
    assert!(r1.recv_timeout(d).is_plain(E::B(4)));
    assert!(r2.try_recv().is_empty());

    dharma::direct_connect(&mut s2, &r2);

    s1.send_plain(E::B(6));
    s2.send_plain(E::B(7));
    assert!(r1.recv_timeout(d).is_plain(E::B(5)));
    assert!(r2.recv_timeout(d).is_plain(E::B(6)));
    assert!(r2.recv_timeout(d).is_plain(E::B(7)));
    assert!(r1.try_recv().is_empty());
    assert!(r2.try_recv().is_empty());
}

// -------------------------------------------------------------------------------------------------

/// Test sending one piece of data from one `Sender` to one `Receiver` from different thread.
#[test]
fn test_one_sender_with_one_receiver_threaded() {
    let d = time::Duration::new(1, 0);
    let mut s = dharma::Sender::new();
    let mut r = dharma::Receiver::new();
    dharma::connect(&mut s, &r);

    let join_handle = thread::spawn(move || { s.send_plain(E::A); });

    assert!(r.recv_timeout(d).is_plain(E::A));
    assert!(r.try_recv().is_empty());
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------
