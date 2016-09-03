// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit tests for `dharma::event_loop` module.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

mod common;

use std::clone::Clone;
use std::thread;
use std::time;

use common::mocks::{ContextStub, ModuleMock};
use self::dharma::{EventLoopInfo, Module, Signaler};

// -------------------------------------------------------------------------------------------------

type StringModule = Box<Module<T = String, C = ContextStub>>;

// -------------------------------------------------------------------------------------------------

/// Timeout between starting thread and starting test.
/// (Is there better way to syncronize with signal subscription?)
const D: u64 = 100;

// -------------------------------------------------------------------------------------------------

/// Add one `Module` to one `EventLoop`, start and terminate. `Module` should be intialized and
/// finalized once.
#[test]
fn test_life_of_one_module() {
    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Prepare mock and set expectations
    let module = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(None));
        module.expect_initialized_times(1);
        module.expect_executed_times(0);
        module.expect_finalized_times(1);
        module as StringModule
    });

    // Do test
    info.add_module(module);
    let join_handle = info.start_event_loop().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s to one `EventLoop`, start and terminate. `Module`s should be intialized and
/// finalized once each.
#[test]
fn test_life_of_two_modules() {
    // Prepare mockand set expectations
    let module1 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(None));
        module.expect_initialized_times(1);
        module.expect_executed_times(0);
        module.expect_finalized_times(1);
        module as StringModule
    });

    let module2 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(None));
        module.expect_initialized_times(1);
        module.expect_executed_times(0);
        module.expect_finalized_times(1);
        module as StringModule
    });

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module1);
    info.add_module(module2);
    let join_handle = info.start_event_loop().unwrap();
    thread::sleep(time::Duration::from_millis(100));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add one `Module` which subscribes for three signals to one `EventLoop`, start, emit many signals
/// and terminate. `Module` should be intialized and finalized once and be notified in correct order
/// for all signals it subscribed but not for the ones it did not subscribe.
#[test]
fn test_execution_of_one_module() {
    // Prepare mock and set expectations
    let module = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(Some(vec![1, 2, 3])));
        module.expect_initialized_times(1);
        module.expect_executed_times(3);
        module.expect_execute("2".to_owned());
        module.expect_execute("1".to_owned());
        module.expect_execute("3".to_owned());
        module.expect_finalized_times(1);
        module as StringModule
    });

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module);
    let join_handle = info.start_event_loop().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s which subscribe for three signals (which partialy overlap) to one `EventLoop`,
/// start, emit many signals and terminate. `Module`s should be intialized and finalized once each
/// and be notified in correct order for all signals they subscribed but not for the ones they did
/// not subscribe.
#[test]
fn test_execution_of_two_modules() {
    // Prepare mock and set expectations
    let module1 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(Some(vec![1, 2])));
        module.expect_initialized_times(1);
        module.expect_executed_times(3);
        module.expect_execute("2".to_owned());
        module.expect_execute("1".to_owned());
        module.expect_execute("2".to_owned());
        module.expect_finalized_times(1);
        module as StringModule
    });

    let module2 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(Some(vec![2, 3])));
        module.expect_initialized_times(1);
        module.expect_executed_times(3);
        module.expect_execute("2".to_owned());
        module.expect_execute("3".to_owned());
        module.expect_execute("2".to_owned());
        module.expect_finalized_times(1);
        module as StringModule
    });
    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module1);
    info.add_module(module2);
    let join_handle = info.start_event_loop().unwrap();
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

/// Add two `Module`s which subscribe for three signals (which partialy overlap) to two
/// `EventLoop`s, start, emit many signals and terminate. `Module`s should be intialized and
/// finalized once each and be notified in correct order for all signals they subscribed but not
/// for the ones they did not subscribe.
#[test]
fn test_execution_of_two_modules_in_different_threads() {
    // Prepare mock and set expectations
    let module1 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(Some(vec![1, 2])));
        module.expect_initialized_times(1);
        module.expect_executed_times(3);
        module.expect_execute("2".to_owned());
        module.expect_execute("1".to_owned());
        module.expect_execute("2".to_owned());
        module.expect_finalized_times(1);
        module as StringModule
    });

    let module2 = Box::new(move || {
        let mut module = Box::new(ModuleMock::new(Some(vec![2, 3])));
        module.expect_initialized_times(1);
        module.expect_executed_times(3);
        module.expect_execute("2".to_owned());
        module.expect_execute("3".to_owned());
        module.expect_execute("2".to_owned());
        module.expect_finalized_times(1);
        module as StringModule
    });

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info1 = EventLoopInfo::new("test1".to_owned(), signaler.clone(), context.clone());
    let mut info2 = EventLoopInfo::new("test2".to_owned(), signaler.clone(), context.clone());

    // Do test
    info1.add_module(module1);
    info2.add_module(module2);
    let join_handle1 = info1.start_event_loop().unwrap();
    let join_handle2 = info2.start_event_loop().unwrap();
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
