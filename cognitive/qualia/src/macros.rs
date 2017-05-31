// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains commonly used macros.

// -------------------------------------------------------------------------------------------------

/// This macro helps define structure being `Rc<RefCell>` for another structure and implements
/// methods for borrowing it. Borrowing it when already borrowed mutably is program logic error
/// and suitable warning is logged in such case.
#[macro_export]
macro_rules! define_ref {
    (struct $name:ty as $ref_name:ident) => {
        #[derive(Clone)]
        pub struct $ref_name (std::rc::Rc<std::cell::RefCell<$name>>);

        #[allow(dead_code)]
        impl $ref_name {
            define_ref!(_impl_ $name as $ref_name);
        }
    };

    (trait $name:ty as $ref_name:ident) => {
        #[derive(Clone)]
        pub struct $ref_name<T> (std::rc::Rc<std::cell::RefCell<T>>) where T: $name;

        #[allow(dead_code)]
        impl<T> $ref_name<T> where T: $name {
            define_ref!(_impl_ T as $ref_name);
        }
    };

    (_impl_ $name:ty as $ref_name:ident) => {
        pub fn new(obj: $name) -> Self {
            $ref_name(std::rc::Rc::new(std::cell::RefCell::new(obj)))
        }

        pub fn transform(obj: std::rc::Rc<std::cell::RefCell<$name>>) -> Self {
            $ref_name(obj)
        }

        pub fn borrow(&self) -> std::cell::Ref<$name> {
            match self.0.try_borrow() {
                Ok(obj) => {
                    obj
                }
                Err(err) => {
                    let msg = format!("Failed to borrow {}! \
                                      This is fail of programs internal logic. {:?}",
                                      stringify!($name), err);
                    log_fatal!("{}", msg);
                    panic!(msg);
                }
            }
        }

        pub fn borrow_mut(&self) -> std::cell::RefMut<$name> {
            match self.0.try_borrow_mut() {
                Ok(obj) => {
                    obj
                }
                Err(err) => {
                    let msg = format!("Failed to borrow {} mutably! \
                                      This is fail of programs internal logic. {:?}",
                                      stringify!($name), err);
                    log_fatal!("{}", msg);
                    panic!(msg);
                }
            }
        }

        pub fn downgrade(&self) -> std::rc::Weak<std::cell::RefCell<$name>> {
            std::rc::Rc::downgrade(&self.0)
        }
    };
}

// -------------------------------------------------------------------------------------------------

/// This macro helps implement ID type.
#[macro_export]
macro_rules! define_id {
    ($name:ident: $ty:ty) => {
        #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
        pub struct $name($ty);
        define_id!{_impl_ $name}
    };

    (pub $name:ident: $ty:ty) => {
        #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
        pub struct $name($ty);
        define_id!{_impl_ $name}
    };

    {_impl_ $name:ident} => {
        impl $name {
            pub fn initial() -> Self {
                $name(1)
            }

            pub fn incremented(id: &Self) -> Self {
                $name(id.0 + 1)
            }

            pub fn increment(&mut self) -> Self {
                self.0 += 1;
                *self
            }
        }
    };
}

// -------------------------------------------------------------------------------------------------
