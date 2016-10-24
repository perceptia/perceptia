// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module define surface history functionality.

// TODO: Add unit tests.

// -------------------------------------------------------------------------------------------------

// TODO: Check what is better: VecDeque or LinkedList.
use std::collections::VecDeque as Ordering;

use qualia::SurfaceId;

// -------------------------------------------------------------------------------------------------

mod magic {
    pub const AVARAGE_NUM_SURFACES: usize = 10;
    pub const PEEK_TO_AVARAGE_RATIO: usize = 3;
    pub const OPTIMAL_TO_AVARAGE_RATIO: usize = 2;
}

// -------------------------------------------------------------------------------------------------

/// Provides functionality to manage surface history as resizable list.
pub struct SurfaceHistory {
    history: Ordering<SurfaceId>,
}

// -------------------------------------------------------------------------------------------------

impl SurfaceHistory {
    /// `SurfaceHistory` constructor.
    pub fn new() -> Self {
        SurfaceHistory { history: Ordering::with_capacity(magic::AVARAGE_NUM_SURFACES) }
    }

    /// Add surface as the latest in history.
    pub fn add(&mut self, sid: SurfaceId) {
        self.history.push_front(sid);
    }

    /// Return `n`th element from begin. If `n` is negative, return `n`th element from end.
    pub fn get_nth(&self, n: isize) -> Option<SurfaceId> {
        if let Some(sid) = if n < 0 {
            let m = -n as usize;
            self.history.get(self.history.len() - m)
        } else {
            self.history.get(n as usize)
        } {
            Some(*sid)
        } else {
            None
        }
    }

    /// Make given surface the latest in history.
    pub fn pop(&mut self, sid: SurfaceId) {
        self.simple_remove(sid);
        self.add(sid);
    }

    /// Remove surface. Shrink underlying memory pool if needed.
    pub fn remove(&mut self, sid: SurfaceId) {
        self.simple_remove(sid);

        let len = self.history.len();
        let capacity = self.history.capacity();
        if (len > magic::AVARAGE_NUM_SURFACES) &&
           ((magic::PEEK_TO_AVARAGE_RATIO * len) > capacity) {
            self.history.truncate(magic::OPTIMAL_TO_AVARAGE_RATIO * len);
        }
    }

    /// Remove surface without shrinking memory pool.
    fn simple_remove(&mut self, sid: SurfaceId) {
        let len = self.history.len();
        for i in 0..len {
            if *self.history.get(i).unwrap() == sid {
                self.history.remove(i);
                break;
            }
        }
    }

    /// Return iterator for `SurfaceHistory`.
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Iter<'a> {
    history: &'a SurfaceHistory,
    pos: isize,
}

// -------------------------------------------------------------------------------------------------

impl<'a> Iter<'a> {
    /// `Iter` constructor.
    fn new(history: &'a SurfaceHistory) -> Self {
        Iter {
            history: history,
            pos: -1,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Iterator for Iter<'a> {
    type Item = SurfaceId;

    fn next(&mut self) -> Option<SurfaceId> {
        self.pos += 1;
        self.history.get_nth(self.pos)
    }
}

// -------------------------------------------------------------------------------------------------
