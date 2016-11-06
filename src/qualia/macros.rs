// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains commonly used macros.

// -------------------------------------------------------------------------------------------------

/// This macro helps define structure being `Rc<RefCell>` for another structure and implements
/// methods for borrowing it. Borrowing it when already borrowed mutably is program logic error
/// and suitable warning is logged in such case.
#[macro_export]
macro_rules! define_ref {
    ($name:ident, $ref_name:ident) => {
        #[derive(Clone)]
        pub struct $ref_name (std::rc::Rc<std::cell::RefCell<$name>>);

        impl $ref_name {
            pub fn new(obj: $name) -> Self {
                $ref_name(std::rc::Rc::new(std::cell::RefCell::new(obj)))
            }

            pub fn borrow(&self) -> std::cell::Ref<$name> {
                match self.0.try_borrow() {
                    Ok(obj) => {
                        obj
                    }
                    Err(err) => {
                        let msg = format!("Failed to borrow $name! \
                                          This is fail of programs internal logic. {:?}",
                                          err);
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
                        let msg = format!("Failed to borrow $name mutably! \
                                          This is fail of programs internal logic. {:?}",
                                          err);
                        log_fatal!("{}", msg);
                        panic!(msg);
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This macro helps implement ID type.
#[macro_rules]
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
