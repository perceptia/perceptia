// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Defines `Frame` data structure representing space and time layout of surfaces.

// -------------------------------------------------------------------------------------------------

use std::{fmt, mem, ptr};
use std::default::Default;

use alloc::heap;

use qualia::{SurfaceId, Area, Position, Size};

// -------------------------------------------------------------------------------------------------

/// Alias for optional frame.
type Link = Option<Frame>;

// -------------------------------------------------------------------------------------------------

/// Helper data structure for defining edges and nodes in frame tree graph.
struct Edges {
    /// Links to previous frame in order.
    prev: Link,

    /// Links to next frame in order.
    next: Link,

    /// Links to child first in order.
    first: Link,

    /// Links to child last in order.
    last: Link,
}

// -------------------------------------------------------------------------------------------------

impl Default for Edges {
    fn default() -> Self {
        Edges {
            prev: None,
            next: None,
            first: None,
            last: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper data structure for defining edges and nodes in frame tree graph.
struct Node {
    /// Links to parent frame.
    matter: Link,

    /// Links to space-like siblings and children.
    space: Edges,

    /// Links to time-like siblings and children.
    time: Edges,
}

// -------------------------------------------------------------------------------------------------

impl Default for Node {
    fn default() -> Self {
        Node {
            matter: None,
            space: Edges::default(),
            time: Edges::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Defines mode of the frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Root,
    Special,
    Workspace,
    Container,
    Leaf,
}

// -------------------------------------------------------------------------------------------------

/// Defines geometry of the frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Geometry {
    /// Children of frame with this geometry are placed vertically - in one row.
    Vertical,

    /// Children of frame with this geometry are placed how - in one column.
    Horizontal,

    /// Children of frame with this geometry are placed on stack - only one is visible at a time.
    Stacked,

    /// Children of frame with this geometry can be in arbitrary place and have arbitrary size.
    Floating,
}

// -------------------------------------------------------------------------------------------------

/// Parameters of the frame defining its properties.
pub struct Parameters {
    /// ID of assigned surface.
    pub sid: SurfaceId,

    /// Mode.
    pub mode: Mode,

    /// Geometry.
    pub geometry: Geometry,

    /// Position.
    pub pos: Position,

    /// Size.
    pub size: Size,

    /// Title.
    pub title: String,
}

// -------------------------------------------------------------------------------------------------

impl Parameters {
    /// Creates new parameters for root frame.
    pub fn new_root() -> Self {
        Parameters {
            sid: SurfaceId::invalid(),
            mode: Mode::Root,
            geometry: Geometry::Floating,
            pos: Position::default(),
            size: Size::default(),
            title: "PERCEPTIA".to_owned(),
        }
    }

    /// Creates new parameters for display frame.
    pub fn new_display(area: Area, title: String) -> Self {
        Parameters {
            sid: SurfaceId::invalid(),
            mode: Mode::Special,
            geometry: Geometry::Stacked,
            pos: area.pos,
            size: area.size,
            title: title,
        }
    }

    /// Creates new parameters for workspace frame.
    pub fn new_workspace() -> Self {
        Parameters {
            sid: SurfaceId::invalid(),
            mode: Mode::Special,
            geometry: Geometry::Stacked,
            pos: Position::default(),
            size: Size::default(),
            title: "".to_owned(),
        }
    }

    /// Creates new parameters for container frame.
    pub fn new_container(geometry: Geometry) -> Self {
        Parameters {
            sid: SurfaceId::invalid(),
            mode: Mode::Container,
            geometry: geometry,
            pos: Position::default(),
            size: Size::default(),
            title: "".to_owned(),
        }
    }

    /// Creates new parameters for leaf frame.
    pub fn new_leaf(sid: SurfaceId, geometry: Geometry) -> Self {
        Parameters {
            sid: sid,
            mode: Mode::Leaf,
            geometry: geometry,
            pos: Position::default(),
            size: Size::default(),
            title: "".to_owned(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for defining `Frame` structure.
pub struct InnerFrame {
    /// Parameters.
    params: Parameters,

    /// Links to neighbouring frames.
    node: Node,
}


// -------------------------------------------------------------------------------------------------

/// Frame main data structure.
#[derive(Clone)]
pub struct Frame {
    inner: *mut InnerFrame,
}

// -------------------------------------------------------------------------------------------------

/// Public constructors and destructor.
impl Frame {
    /// Creates new root frame.
    pub fn new_root() -> Self {
        Self::allocate(InnerFrame {
            params: Parameters::new_root(),
            node: Node::default(),
        })
    }

    /// Creates new display frame.
    pub fn new_display(area: Area, title: String) -> Self {
        Self::allocate(InnerFrame {
            params: Parameters::new_display(area, title),
            node: Node::default(),
        })
    }

    /// Creates new workspace frame.
    pub fn new_workspace() -> Self {
        Self::allocate(InnerFrame {
            params: Parameters::new_workspace(),
            node: Node::default(),
        })
    }

    /// Creates new container frame.
    pub fn new_container(geometry: Geometry) -> Self {
        Self::allocate(InnerFrame {
            params: Parameters::new_container(geometry),
            node: Node::default(),
        })
    }

    /// Creates new leaf frame.
    pub fn new_leaf(sid: SurfaceId, geometry: Geometry) -> Self {
        Self::allocate(InnerFrame {
            params: Parameters::new_leaf(sid, geometry),
            node: Node::default(),
        })
    }

    /// Destroys frame recursively and deallocate memory.
    pub fn destroy(&self) {
        for f in self.time_iter() {
            f.destroy();
        }
        self.deallocate();
    }
}

// -------------------------------------------------------------------------------------------------

/// Getting iterators.
impl Frame {
    /// Gets iterator over children in time order.
    pub fn time_iter(&self) -> FrameTimeIterator {
        FrameTimeIterator { frame: self.get_first_time() }
    }

    /// Gets iterator over children in space order.
    pub fn space_iter(&self) -> FrameSpaceIterator {
        FrameSpaceIterator { frame: self.get_first_space() }
    }
}

// -------------------------------------------------------------------------------------------------

/// Public getters for frame internals.
impl Frame {
    /// Gets ID of assigned surface.
    #[inline]
    pub fn get_sid(&self) -> SurfaceId {
        unsafe { (*self.inner).params.sid }
    }

    /// Gets mode.
    #[inline]
    pub fn get_mode(&self) -> Mode {
        unsafe { (*self.inner).params.mode }
    }

    /// Gets geometry.
    #[inline]
    pub fn get_geometry(&self) -> Geometry {
        unsafe { (*self.inner).params.geometry }
    }

    /// Gets position.
    #[inline]
    pub fn get_position(&self) -> Position {
        unsafe { (*self.inner).params.pos.clone() }
    }

    /// Gets size.
    #[inline]
    pub fn get_size(&self) -> Size {
        unsafe { (*self.inner).params.size.clone() }
    }
}

// -------------------------------------------------------------------------------------------------

impl Frame {
    /// Sets size without informing other parts of application.
    #[inline]
    pub fn set_plumbing_position(&mut self, pos: Position) {
        unsafe { (*self.inner).params.pos = pos; }
    }

    /// Sets size without informing other parts of application.
    #[inline]
    pub fn set_plumbing_size(&mut self, size: Size) {
        unsafe { (*self.inner).params.size = size; }
    }

    /// Sets position and size without informing other parts of application.
    #[inline]
    pub fn set_plumbing_position_and_size(&mut self, pos: Position, size: Size) {
        unsafe { (*self.inner).params.pos = pos; }
        unsafe { (*self.inner).params.size = size; }
    }

}

// -------------------------------------------------------------------------------------------------

/// Public getters for neighbouring frames.
impl Frame {
    /// Checks if frame has parent.
    #[inline]
    pub fn has_parent(&self) -> bool {
        unsafe { (*self.inner).node.matter.is_some() }
    }

    /// Checks if frame has children.
    #[inline]
    pub fn has_children(&self) -> bool {
        unsafe { (*self.inner).node.time.last.is_some() }
    }

    /// Optionally returns frames parent.
    #[inline]
    pub fn get_parent(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.matter.clone() }
    }

    /// Optionally returns child first in time order.
    #[inline]
    pub fn get_first_time(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.time.first.clone() }
    }

    /// Optionally returns sibling next in time order.
    #[inline]
    pub fn get_next_time(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.time.next.clone() }
    }

    /// Optionally returns child first in space order.
    #[inline]
    pub fn get_first_space(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.space.first.clone() }
    }

    /// Optionally returns sibling previous in space order.
    #[inline]
    pub fn get_prev_space(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.space.prev.clone() }
    }

    /// Optionally returns sibling next in space order.
    #[inline]
    pub fn get_next_space(&self) -> Option<Frame> {
        unsafe { (*self.inner).node.space.next.clone() }
    }
}

// -------------------------------------------------------------------------------------------------

/// Public manipulators. Allow to change order relations between frames.
impl Frame {
    /// Prepends in spatial order and appends in time order given frame to self children.
    pub fn prepend(&mut self, frame: &mut Frame) {
        self.append_time(frame);
        self.prepend_space(frame);
        frame.set_matter(self);
    }

    /// Appends in spatial order and appends in time order given frame to self children.
    pub fn append(&mut self, frame: &mut Frame) {
        self.append_time(frame);
        self.append_space(frame);
        frame.set_matter(self);
    }

    /// Inserts given frame as previous in spatial order sibling of self. Frame becomes last
    /// sibling in time order.
    pub fn prejoin(&mut self, frame: &mut Frame) {
        if let Some(ref mut parent) = self.get_parent() {
            parent.append_time(frame);
            if let Some(ref mut prev) = self.get_prev_space() {
                prev.join_space(frame);
                frame.join_space(self);
            } else {
                parent.prepend_space(frame);
            }
            frame.set_matter(parent);
        }
    }

    /// Inserts given frame as next in spatial order sibling of self. Frame becomes last
    /// sibling in time order.
    pub fn adjoin(&mut self, frame: &mut Frame) {
        if let Some(ref mut parent) = self.get_parent() {
            parent.append_time(frame);
            if let Some(ref mut next) = self.get_next_space() {
                self.join_space(frame);
                frame.join_space(next);
            } else {
                parent.append_space(frame);
            }
            frame.set_matter(parent);
        }
    }

    /// Make given frame first in time order of all its siblings. Spatial order is untouched.
    pub fn pop(&mut self) {
        if let Some(ref mut parent) = self.get_parent() {
            self.unjoin_time();
            parent.prepend_time(self);
        }
    }

    /// Remove given frame from its parent children.
    pub fn remove(&mut self) {
        if self.has_parent() {
            self.unjoin_time();
            self.unjoin_space();
            self.reset_matter();
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Private manipulators. Allow to change order relations between frames.
impl Frame {
    /// Set parent.
    #[inline]
    fn set_matter(&mut self, frame: &Frame) {
        unsafe {
            (*self.inner).node.matter = Some(frame.clone());
        }
    }

    /// Unset parent.
    #[inline]
    fn reset_matter(&mut self) {
        unsafe {
            (*self.inner).node.matter = None;
        }
    }

    /// Prepend frame in time order.
    #[inline]
    fn prepend_time(&mut self, frame: &mut Frame) {
        unsafe {
            if let Some(ref mut first_time) = (*self.inner).node.time.first {
                (*first_time.inner).node.time.prev = Some(frame.clone());
                (*frame.inner).node.time.next = Some(first_time.clone());
            } else {
                (*self.inner).node.time.last = Some(frame.clone());
            }
            (*self.inner).node.time.first = Some(frame.clone());
        }
    }

    /// Append frame in time order.
    #[inline]
    fn append_time(&mut self, frame: &mut Frame) {
        unsafe {
            if let Some(ref mut last_time) = (*self.inner).node.time.last {
                (*last_time.inner).node.time.next = Some(frame.clone());
                (*frame.inner).node.time.prev = Some(last_time.clone());
            } else {
                (*self.inner).node.time.first = Some(frame.clone());
            }
            (*self.inner).node.time.last = Some(frame.clone());
        }
    }

    /// Prepend frame in space order.
    #[inline]
    fn prepend_space(&mut self, frame: &mut Frame) {
        unsafe {
            if let Some(ref mut first_space) = (*self.inner).node.space.first {
                frame.join_space(first_space);
            } else {
                (*self.inner).node.space.last = Some(frame.clone());
            }
            (*self.inner).node.space.first = Some(frame.clone());
        }
    }

    /// Append frame in space order.
    #[inline]
    fn append_space(&mut self, frame: &mut Frame) {
        unsafe {
            if let Some(ref mut last_space) = (*self.inner).node.space.last {
                last_space.join_space(frame);
            } else {
                (*self.inner).node.space.first = Some(frame.clone());
            }
            (*self.inner).node.space.last = Some(frame.clone());
        }
    }

    /// Join two frames in space order. `self` becomes previous from `frame` and `frame` next from
    /// `self`.
    #[inline]
    fn join_space(&mut self, frame: &mut Frame) {
        unsafe {
            (*self.inner).node.space.next = Some(frame.clone());
            (*frame.inner).node.space.prev = Some(self.clone());
        }
    }

    /// Remove given frame from time order.
    #[inline]
    fn unjoin_time(&mut self) {
        unsafe {
            if let Some(ref mut next_time) = (*self.inner).node.time.next {
                (*next_time.inner).node.time.prev = (*self.inner).node.time.prev.clone();
            } else if let Some(ref mut matter) = (*self.inner).node.matter {
                (*matter.inner).node.time.last = (*self.inner).node.time.prev.clone();
            }
            if let Some(ref mut prev_time) = (*self.inner).node.time.prev {
                (*prev_time.inner).node.time.next = (*self.inner).node.time.next.clone();
            } else if let Some(ref mut matter) = (*self.inner).node.matter {
                (*matter.inner).node.time.first = (*self.inner).node.time.next.clone();
            }
            (*self.inner).node.time.prev = None;
            (*self.inner).node.time.next = None;
        }
    }

    /// Remove given frame from space order.
    #[inline]
    fn unjoin_space(&mut self) {
        unsafe {
            if let Some(ref mut next_space) = (*self.inner).node.space.next {
                (*next_space.inner).node.space.prev = (*self.inner).node.space.prev.clone();
            } else if let Some(ref mut matter) = (*self.inner).node.matter {
                (*matter.inner).node.space.last = (*self.inner).node.space.prev.clone();
            }
            if let Some(ref mut prev_space) = (*self.inner).node.space.prev {
                (*prev_space.inner).node.space.next = (*self.inner).node.space.next.clone();
            } else if let Some(ref mut matter) = (*self.inner).node.matter {
                (*matter.inner).node.space.first = (*self.inner).node.space.next.clone();
            }
            (*self.inner).node.space.prev = None;
            (*self.inner).node.space.next = None;
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Allocators/deallocators.
impl Frame {
    /// Helper method for allocating new frame on heap.
    fn allocate(inner: InnerFrame) -> Self {
        let ptr = unsafe {
            heap::allocate(mem::size_of::<InnerFrame>(), mem::align_of::<InnerFrame>())
        } as *mut _;
        unsafe {
            ptr::write(ptr, inner);
        }

        Frame { inner: ptr }
    }

    /// Helper method for destroying frame. Deallocate memory on heap.
    fn deallocate(&self) {
        let ptr = self.inner as *mut _;
        unsafe {
            heap::deallocate(ptr,
                             mem::size_of::<InnerFrame>(),
                             mem::align_of::<InnerFrame>());
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Miscellaneous other methods.
impl Frame {
    /// Compares frame internals for check if frames are not only same but **the** same.
    #[inline]
    pub fn equals_exact(&self, other: &Frame) -> bool {
        self.inner == other.inner
    }

    /// Counts children and returns their number.
    pub fn count_children(&self) -> usize {
        let mut result = 0;
        for _ in self.time_iter() {
            result += 1
        }
        result
    }
}

// -------------------------------------------------------------------------------------------------

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Frame(sid: {:?}, mode: {:?}, geometry: {:?}, size: {:?})",
               self.get_sid(),
               self.get_mode(),
               self.get_geometry(),
               self.get_size())
    }
}

// -------------------------------------------------------------------------------------------------

/// Iterator over frames in time order.
pub struct FrameTimeIterator {
    frame: Link,
}

// -------------------------------------------------------------------------------------------------

impl Iterator for FrameTimeIterator {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        let result = self.frame.clone();
        self.frame = if let Some(ref mut frame) = self.frame {
            frame.get_next_time()
        } else {
            None
        };
        result
    }
}

// -------------------------------------------------------------------------------------------------

/// Iterator over frames in space order.
pub struct FrameSpaceIterator {
    frame: Link,
}

// -------------------------------------------------------------------------------------------------

impl Iterator for FrameSpaceIterator {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        let result = self.frame.clone();
        self.frame = if let Some(ref mut frame) = self.frame {
            frame.get_next_space()
        } else {
            None
        };
        result
    }
}

// -------------------------------------------------------------------------------------------------
