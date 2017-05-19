// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Unit tests for `dharma::event_loop` module.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

mod common;

use std::thread;
use std::time;

use common::mocks::{ContextStub, ModuleConstructorMock};
use dharma::{EventLoopInfo, Signaler};

// -------------------------------------------------------------------------------------------------

/// Timeout between starting thread and starting test.
/// FIXME: Is there better way to synchronize with signal subscription?
const D: u64 = 100;

// -------------------------------------------------------------------------------------------------

/// Add one `Module` to one `EventLoop`, start and terminate. `Module` should be initialized and
/// finalized once.
#[test]
fn test_life_of_one_module() {
    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    let module = Box::new(ModuleConstructorMock::new(Vec::new(), Vec::new()));

    // Do test
    info.add_module(module);
    let join_handle = info.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s to one `EventLoop`, start and terminate. `Module`s should be initialized and
/// finalized once each.
#[test]
fn test_life_of_two_modules() {
    // Prepare mock and set expectations
    let module1 = Box::new(ModuleConstructorMock::new(Vec::new(), Vec::new()));
    let module2 = Box::new(ModuleConstructorMock::new(Vec::new(), Vec::new()));

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module1);
    info.add_module(module2);
    let join_handle = info.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add one `Module` which subscribes for three signals to one `EventLoop`, start, emit many
/// signals and terminate. `Module` should be initialized and finalized once and be notified in
/// correct order for all signals it subscribed but not for the ones it did not subscribe.
#[test]
fn test_execution_of_one_module() {
    // Prepare mock and set expectations
    let signals = vec![1, 2, 3];
    let packages = vec!["2".to_owned(), "1".to_owned(), "3".to_owned()];
    let module = Box::new(ModuleConstructorMock::new(signals, packages));

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module);
    let join_handle = info.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s which subscribe for three signals (which partially overlap) to one
/// `EventLoop`, start, emit many signals and terminate. `Module`s should be initialized and
/// finalized once each and be notified in correct order for all signals they subscribed but not
/// for the ones they did not subscribe.
#[test]
fn test_execution_of_two_modules() {
    // Prepare mock and set expectations
    let signals1 = vec![1, 2];
    let packages1 = vec!["2".to_owned(), "1".to_owned(), "2".to_owned()];
    let module1 = Box::new(ModuleConstructorMock::new(signals1, packages1));

    let signals2 = vec![2, 3];
    let packages2 = vec!["2".to_owned(), "3".to_owned(), "2".to_owned()];
    let module2 = Box::new(ModuleConstructorMock::new(signals2, packages2));

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info = EventLoopInfo::new("test".to_owned(), signaler.clone(), context);

    // Do test
    info.add_module(module1);
    info.add_module(module2);
    let join_handle = info.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
    signaler.emit(2, "2".to_owned());
    signaler.emit(4, "4".to_owned());
    signaler.emit(1, "1".to_owned());
    signaler.emit(3, "3".to_owned());
    signaler.emit(2, "2".to_owned());
    signaler.terminate();
    assert!(join_handle.join().is_ok());
}

// -------------------------------------------------------------------------------------------------

/// Add two `Module`s which subscribe for three signals (which partially overlap) to two
/// `EventLoop`s, start, emit many signals and terminate. `Module`s should be initialized and
/// finalized once each and be notified in correct order for all signals they subscribed but not
/// for the ones they did not subscribe.
#[test]
fn test_execution_of_two_modules_in_different_threads() {
    // Prepare mock and set expectations
    let signals1 = vec![1, 2];
    let packages1 = vec!["2".to_owned(), "1".to_owned(), "2".to_owned()];
    let module1 = Box::new(ModuleConstructorMock::new(signals1, packages1));

    let signals2 = vec![2, 3];
    let packages2 = vec!["2".to_owned(), "3".to_owned(), "2".to_owned()];
    let module2 = Box::new(ModuleConstructorMock::new(signals2, packages2));

    // Prepare environment
    let mut signaler = Signaler::new();
    let context = ContextStub::new();
    let mut info1 = EventLoopInfo::new("test1".to_owned(), signaler.clone(), context.clone());
    let mut info2 = EventLoopInfo::new("test2".to_owned(), signaler.clone(), context.clone());

    // Do test
    info1.add_module(module1);
    info2.add_module(module2);
    let join_handle1 = info1.start().unwrap();
    let join_handle2 = info2.start().unwrap();
    thread::sleep(time::Duration::from_millis(D));
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
