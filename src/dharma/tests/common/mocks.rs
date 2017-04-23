// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Module containing mocks for unit test in `dharma`.

// -------------------------------------------------------------------------------------------------

extern crate dharma;

use std::sync::{Arc, Mutex};
use std::thread;

use dharma::{InitResult, Module, ModuleConstructor};

// -------------------------------------------------------------------------------------------------

/// Helper structure constituting shared memory between mocks from different threads.
struct InnerModuleMock {
    times_initialized: u32,
    times_executed: u32,
    times_finalized: u32,
    packages: Vec<String>,

    expected_times_initialized: Option<u32>,
    expected_times_executed: Option<u32>,
    expected_times_finalized: Option<u32>,
    expected_packages: Option<Vec<String>>,

    signals: Vec<dharma::SignalId>,
}

// -------------------------------------------------------------------------------------------------

/// Stub for `Context`.
#[derive(Clone)]
pub struct ContextStub {}

// -------------------------------------------------------------------------------------------------

impl ContextStub {
    /// Constructs new `ContextStub`.
    pub fn new() -> Self {
        ContextStub {}
    }
}

// -------------------------------------------------------------------------------------------------

/// Mock of `Module`.
pub struct ModuleMock {
    inner: Arc<Mutex<InnerModuleMock>>,
}

// -------------------------------------------------------------------------------------------------

impl ModuleMock {
    /// Constructs new `ModuleMock`.
    pub fn new(signals: Vec<dharma::SignalId>) -> Self {
        ModuleMock {
            inner: Arc::new(Mutex::new(InnerModuleMock {
                                           times_initialized: 0,
                                           times_executed: 0,
                                           times_finalized: 0,
                                           packages: Vec::new(),
                                           expected_times_initialized: None,
                                           expected_times_executed: None,
                                           expected_times_finalized: None,
                                           expected_packages: None,
                                           signals: signals,
                                       })),
        }
    }

    /// Set expectation on number of invocations of `initialize`.
    pub fn expect_initialized_times(&mut self, times: u32) {
        let mut mine = self.inner.lock().unwrap();
        mine.expected_times_initialized = Some(times);
    }

    /// Set expectation on number of invocations of `execute`.
    pub fn expect_executed_times(&mut self, times: u32) {
        let mut mine = self.inner.lock().unwrap();
        mine.expected_times_executed = Some(times);
    }

    /// Set expectation on arguments of `execute`.
    pub fn expect_execute(&mut self, package: String) {
        let mut mine = self.inner.lock().unwrap();
        let mut need_initialize = false;
        match mine.expected_packages {
            Some(ref mut pkgs) => pkgs.push(package.clone()),
            None => need_initialize = true,
        }
        if need_initialize {
            mine.expected_packages = Some(vec![package.clone()]);
        }
    }

    /// Set expectation on number of invocations of `finalize`.
    pub fn expect_finalized_times(&mut self, times: u32) {
        let mut mine = self.inner.lock().unwrap();
        mine.expected_times_finalized = Some(times);
    }

    /// Perform check of all set expectations.
    fn check_expectations(&self) {
        let mine = self.inner.lock().unwrap();

        // Check number of invocations of `initialize`
        match mine.expected_times_initialized {
            Some(expected) => {
                assert_eq!(expected, mine.times_initialized);
            }
            None => {}
        }

        // Check number of invocations of `execute`
        match mine.expected_times_executed {
            Some(expected) => {
                assert_eq!(expected, mine.times_executed);
            }
            None => {}
        }

        // Check number of invocations of `finalize`
        match mine.expected_times_finalized {
            Some(expected) => {
                assert_eq!(expected, mine.times_finalized);
            }
            None => {}
        }

        // Check if `execute` was invoked with correct arguments in correct order.
        match mine.expected_packages {
            Some(ref expected) => {
                assert_eq!(expected.as_slice(), mine.packages.as_slice());
            }
            None => {}
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// On drop check expectations.
impl Drop for ModuleMock {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.check_expectations();
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Clone for ModuleMock {
    fn clone(&self) -> Self {
        ModuleMock { inner: self.inner.clone() }
    }
}

// -------------------------------------------------------------------------------------------------

impl Module for ModuleMock {
    type T = String;
    type C = ContextStub;

    /// Handle `initialize` invocation.
    #[allow(unused_variables)]
    fn initialize(&mut self) -> InitResult {
        let mut mine = self.inner.lock().unwrap();
        mine.times_initialized += 1;
        mine.signals.clone()
    }

    /// Handle `execute` invocation.
    fn execute(&mut self, package: &Self::T) {
        let mut mine = self.inner.lock().unwrap();
        mine.times_executed += 1;
        mine.packages.push(package.clone());
    }

    /// Handle `finalize` invocation.
    fn finalize(&mut self) {
        let mut mine = self.inner.lock().unwrap();
        mine.times_finalized += 1;
    }
}

// -------------------------------------------------------------------------------------------------

/// Mock of `ModuleConstructor`.
pub struct ModuleConstructorMock {
    signals: Vec<dharma::SignalId>,
    packages: Vec<String>,
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructorMock {
    /// Constructs new `ModuleConstructorMock`.
    pub fn new(signals: Vec<dharma::SignalId>, packages: Vec<String>) -> Self {
        ModuleConstructorMock {
            signals: signals,
            packages: packages,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl ModuleConstructor for ModuleConstructorMock {
    type T = String;
    type C = ContextStub;

    fn construct(&self, _context: &mut Self::C) -> Box<Module<T = Self::T, C = Self::C>> {
        let mut module = Box::new(ModuleMock::new(self.signals.clone()));
        module.expect_initialized_times(1);
        module.expect_executed_times(self.packages.len() as u32);
        for package in self.packages.iter() {
            module.expect_execute(package.clone());
        }
        module.expect_finalized_times(1);
        module
    }
}

// -------------------------------------------------------------------------------------------------
