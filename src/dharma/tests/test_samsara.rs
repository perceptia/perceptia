// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit tests for `dharma::samsara` module.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

mod common;

use std::clone::Clone;
use std::thread;
use std::time;

use common::mocks::ModuleMock;
use self::dharma::{Samsara, Signaler};

// -------------------------------------------------------------------------------------------------

/// Timeout between starting thread and starting test.
/// (Is there better way to syncronize with signal subscription?)
const D: u64 = 100;

// -------------------------------------------------------------------------------------------------

/// Add one `Module` to one `Samsara`, start and terminate. `Module` should be intialized and
/// finalized once.
#[test]
fn test_life_of_one_module() {
    // Prepare environment
    let mut module = Box::new(ModuleMock::new(None));
    let mut signaler = Signaler::new();
    let mut samsara = Samsara::new("test".to_owned(), signaler.clone());

    // Set expectations
    module.expect_initialized_times(1);
    module.expect_executed_times(0);
    module.expect_finalized_times(1);

    // Do test
    samsara.add_module(module.clone());
    let join_handle = samsara.start().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s to one `Samsara`, start and terminate. `Module`s should be intialized and
/// finalized once each.
#[test]
fn test_life_of_two_modules() {
    // Prepare environment
    let mut module1 = Box::new(ModuleMock::new(None));
    let mut module2 = Box::new(ModuleMock::new(None));
    let mut signaler = Signaler::new();
    let mut samsara = Samsara::new("test".to_owned(), signaler.clone());

    // Set expectations
    module1.expect_initialized_times(1);
    module1.expect_executed_times(0);
    module1.expect_finalized_times(1);
    module2.expect_initialized_times(1);
    module2.expect_executed_times(0);
    module2.expect_finalized_times(1);

    // Do test
    samsara.add_module(module1.clone());
    samsara.add_module(module2.clone());
    let join_handle = samsara.start().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add one `Module` which subscribes for three signals to one `Samsara`, start, emit many signals
/// and terminate. `Module` should be intialized and finalized once and be notified in correct order
/// for all signals it subscribed but not for the ones it did not subscribe.
#[test]
fn test_execution_of_one_module() {
    // Prepare environment
    let mut module = Box::new(ModuleMock::new(Some(vec![1, 2, 3])));
    let mut signaler = Signaler::new();
    let mut samsara = Samsara::new("test".to_owned(), signaler.clone());

    // Set expectations
    module.expect_initialized_times(1);
    module.expect_executed_times(3);
    module.expect_execute("2".to_owned());
    module.expect_execute("1".to_owned());
    module.expect_execute("3".to_owned());
    module.expect_finalized_times(1);

    // Do test
    samsara.add_module(module.clone());
    let join_handle = samsara.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s which subscribe for three signals (which partialy overlap) to one `Samsara`,
/// start, emit many signals and terminate. `Module`s should be intialized and finalized once each
/// and be notified in correct order for all signals they subscribed but not for the ones they did
/// not subscribe.
#[test]
fn test_execution_of_two_modules() {
    // Prepare environment
    let mut module1 = Box::new(ModuleMock::new(Some(vec![1, 2])));
    let mut module2 = Box::new(ModuleMock::new(Some(vec![2, 3])));
    let mut signaler = Signaler::new();
    let mut samsara = Samsara::new("test".to_owned(), signaler.clone());

    // Set expectations
    module1.expect_initialized_times(1);
    module1.expect_executed_times(3);
    module1.expect_execute("2".to_owned());
    module1.expect_execute("1".to_owned());
    module1.expect_execute("2".to_owned());
    module1.expect_finalized_times(1);
    module2.expect_initialized_times(1);
    module2.expect_executed_times(3);
    module2.expect_execute("2".to_owned());
    module2.expect_execute("3".to_owned());
    module2.expect_execute("2".to_owned());
    module2.expect_finalized_times(1);

    // Do test
    samsara.add_module(module1.clone());
    samsara.add_module(module2.clone());
    let join_handle = samsara.start().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.emit(2, "2".to_owned());
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s which subscribe for three signals (which partialy overlap) to two `Samsara`s,
/// start, emit many signals and terminate. `Module`s should be intialized and finalized once each
/// and be notified in correct order for all signals they subscribed but not for the ones they did
/// not subscribe.
#[test]
fn test_execution_of_two_modules_in_different_threads() {
    // Prepare environment
    let mut module1 = Box::new(ModuleMock::new(Some(vec![1, 2])));
    let mut module2 = Box::new(ModuleMock::new(Some(vec![2, 3])));
    let mut signaler = Signaler::new();
    let mut samsara1 = Samsara::new("test1".to_owned(), signaler.clone());
    let mut samsara2 = Samsara::new("test2".to_owned(), signaler.clone());

    // Set expectations
    module1.expect_initialized_times(1);
    module1.expect_executed_times(3);
    module1.expect_execute("2".to_owned());
    module1.expect_execute("1".to_owned());
    module1.expect_execute("2".to_owned());
    module1.expect_finalized_times(1);
    module2.expect_initialized_times(1);
    module2.expect_executed_times(3);
    module2.expect_execute("2".to_owned());
    module2.expect_execute("3".to_owned());
    module2.expect_execute("2".to_owned());
    module2.expect_finalized_times(1);

    // Do test
    samsara1.add_module(module1.clone());
    samsara2.add_module(module2.clone());
    let join_handle1 = samsara1.start().unwrap();
    let join_handle2 = samsara2.start().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.emit(2, "2".to_owned());
    signaler.terminate();
    assert!(join_handle1.join().is_ok());
    assert!(join_handle2.join().is_ok());
}

// -------------------------------------------------------------------------------------------------
