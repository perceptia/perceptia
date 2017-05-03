// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains mock for `Output`.

extern crate output;

use std::cell::RefCell;
use std::rc::Rc;

use qualia::{OutputInfo, Illusion, Buffer, Position, SurfaceContext, SurfaceViewer};
use self::output::Output;

// -------------------------------------------------------------------------------------------------

/// Mock of `Output`.
pub struct InnerOutputMock {
    info: OutputInfo,
}

// -------------------------------------------------------------------------------------------------

impl InnerOutputMock {
    pub fn new(info: OutputInfo) -> Self {
        InnerOutputMock {
            info: info,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Mock of `Output`.
#[derive(Clone)]
pub struct OutputMock {
    mock: Rc<RefCell<InnerOutputMock>>,
}

// -------------------------------------------------------------------------------------------------

impl OutputMock {
    pub fn new(info: OutputInfo) -> Self {
        OutputMock {
            mock: Rc::new(RefCell::new(InnerOutputMock::new(info))),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl Output for OutputMock {
    fn draw(&mut self,
            layunder: &Vec<SurfaceContext>,
            surfaces: &Vec<SurfaceContext>,
            layover: &Vec<SurfaceContext>,
            viewer: &SurfaceViewer)
            -> Result<(), Illusion> {
        Ok(())
    }

    fn take_screenshot(&self) -> Result<Buffer, Illusion> {
        panic!("Taking screenshot not supported in unit test");
    }

    fn get_info(&self) -> OutputInfo {
        let mine = self.mock.borrow();
        mine.info.clone()
    }

    fn set_position(&mut self, _position: Position) {}

    fn swap_buffers(&mut self) -> Result<u32, Illusion> {
        Ok(u32::default())
    }

    fn schedule_pageflip(&self) -> Result<(), Illusion> {
        Ok(())
    }

    fn recreate(&self) -> Result<Box<Output>, Illusion> {
        panic!("Recreating not supported in unit test");
    }
}

// -------------------------------------------------------------------------------------------------
