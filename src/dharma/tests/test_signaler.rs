// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit tests for `dharma::signaler` module.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

use std::{thread, time};

use self::dharma::bridge::ReceiveResult;

// -------------------------------------------------------------------------------------------------

/// Helper macro producing `ReceiveResult` for assertions.
macro_rules! make_resp {
    ($num:expr, $str:expr) => { ReceiveResult::Ok(dharma::Event::Package{
                id:      $num,
                package: String::from($str),
            })
        };
}

// -------------------------------------------------------------------------------------------------

/// Type of receiver used in tests.
type EventReceiver = dharma::Receiver<dharma::Event<String>>;

// -------------------------------------------------------------------------------------------------

const T: &'static str = "test";
const H1: &'static str = "hello 1";
const H2: &'static str = "hello 2";
const H3: &'static str = "hello 3";

// -------------------------------------------------------------------------------------------------

/// Subscribe single `Receiver` for one signal and perform `emit` once.  `Receiver` should receive
/// exact same data as emitted. Excess receive tries should indicate empty queue or time out.
#[test]
fn test_subscribe_one_recever_on_one_singnal() {
    let d1 = time::Duration::new(1, 0);
    let d2 = time::Duration::new(0, 1);
    let mut r: EventReceiver = dharma::Receiver::new();
    let mut s = dharma::Signaler::new();

    s.subscribe(0, &r);
    s.emit(0, String::from(T));

    assert_eq!(r.recv_timeout(d1), make_resp!(0, T));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
    assert_eq!(r.recv_timeout(d2), ReceiveResult::Timeout);
}

// -------------------------------------------------------------------------------------------------

/// Subscribe single `Receiver` for many signals and perform `emit` once on each. `Receiver` should
/// receive exact same data as emitted and ignore signals it is not subscribed for. Excess receive
/// tries should indicate empty queue or time out.
#[test]
fn test_subscribe_one_recever_on_two_singnals() {
    let d = time::Duration::new(1, 0);
    let mut r: EventReceiver = dharma::Receiver::new();
    let mut s = dharma::Signaler::new();

    s.subscribe(0, &r);
    s.subscribe(1, &r);

    s.emit(0, String::from(H1));
    s.emit(1, String::from(H2));
    s.emit(2, String::from(H3));
    assert_eq!(r.recv_timeout(d), make_resp!(0, H1));
    assert_eq!(r.recv_timeout(d), make_resp!(1, H2));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
    s.emit(1, String::from(T));
    assert_eq!(r.recv_timeout(d), make_resp!(1, T));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Subscribe many `Receiver`s for the same signal and perform `emit`.  `Receiver`s should
/// independently receive exact same data as emitted and ignore signals they are not subscribed for.
/// Excess receive tries should indicate empty queue or time out.
#[test]
fn test_subscribe_two_recevers_on_same_signal() {
    let d = time::Duration::new(1, 0);
    let mut r1: EventReceiver = dharma::Receiver::new();
    let mut r2: EventReceiver = dharma::Receiver::new();
    let mut s = dharma::Signaler::new();

    s.subscribe(0, &r1);
    s.subscribe(0, &r2);

    s.emit(0, String::from(H1));
    s.emit(1, String::from(H2));
    assert_eq!(r1.recv_timeout(d), make_resp!(0, H1));
    assert_eq!(r2.recv_timeout(d), make_resp!(0, H1));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
    s.emit(0, String::from(T));
    assert_eq!(r1.recv_timeout(d), make_resp!(0, T));
    assert_eq!(r2.recv_timeout(d), make_resp!(0, T));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Subscribe many `Receiver`s for different signals and perform `emit`.  `Receiver`s should
/// independently receive exact same data as emitted and ignore signals they are not subscribed for.
/// Excess receive tries should indicate empty queue or time out.
#[test]
fn test_subscribe_two_recevers_on_different_signals() {
    let d = time::Duration::new(1, 0);
    let mut r1: EventReceiver = dharma::Receiver::new();
    let mut r2: EventReceiver = dharma::Receiver::new();
    let mut s = dharma::Signaler::new();

    s.subscribe(1, &r1);
    s.subscribe(2, &r2);

    s.emit(1, String::from(H1));
    s.emit(2, String::from(H2));
    assert_eq!(r1.recv_timeout(d), make_resp!(1, H1));
    assert_eq!(r2.recv_timeout(d), make_resp!(2, H2));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
    s.emit(0, String::from(T));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
    s.emit(2, String::from(H1));
    s.emit(1, String::from(H2));
    assert_eq!(r1.recv_timeout(d), make_resp!(1, H2));
    assert_eq!(r2.recv_timeout(d), make_resp!(2, H1));
    assert_eq!(r1.try_recv(), ReceiveResult::Empty);
    assert_eq!(r2.try_recv(), ReceiveResult::Empty);
}

// -------------------------------------------------------------------------------------------------

/// Subscribe single `Receiver` for one signal and perform `emit` once from other thread.
/// `Receiver` should receive exact same data as emitted. Excess receive tries should indicate empty
/// queue or time out.
#[test]
fn test_subscribe_one_recever_threaded() {
    let d = time::Duration::new(1, 0);
    let mut r: EventReceiver = dharma::Receiver::new();
    let mut s = dharma::Signaler::new();

    s.subscribe(0, &r);
    let join_handle = thread::spawn(move || {
        s.emit(0, String::from(T));
    });

    assert_eq!(r.recv_timeout(d), make_resp!(0, T));
    assert_eq!(r.try_recv(), ReceiveResult::Empty);
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------
