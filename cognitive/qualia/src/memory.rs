// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides buffer memory management tools.
//!
//! Clients usually share images with server using shared memory. Client creates big shared memory
//! and then tells server which parts server should use for drawing on surfaces. `Memory`
//! represents this shared memory (and is owner of data) and `MemoryView` can be used to view parts
//! of it.
//!
//! Images to redraw can be also created locally or read from file. These images can be stored in
//! `Buffer`. `Buffer` and `MemoryView` implement `Pixmap` trait, but `Buffer` is owned of its data
//! unlike `MemoryView`.
//!
//! `MemoryPool` is used to provide mechanism for storing mapped and buffered memory. The only way
//! to construct `MemoryView` is through `MemoryPool`. Both have counted reference to `Memory` and
//! the `Memory` is destructed when its reference count goes to zero, so `MemoryView`s can be
//! safely used even after `Memory` was removed from `MemoryPool`.

use std;
use std::os::unix::io::RawFd;
use std::sync::Arc;

use nix::sys::mman;

use errors;
use defs::Size;
use image::{Image, Pixmap, PixelFormat};

// -------------------------------------------------------------------------------------------------

/// Container for all data required to draw an image.
#[derive(Clone, Debug)]
pub struct Buffer {
    format: PixelFormat,
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<u8>,
}

// -------------------------------------------------------------------------------------------------

impl Buffer {
    /// Constructors `Buffer`.
    ///
    /// Will panic if passed data size does not match declared size.
    pub fn new(format: PixelFormat,
               width: usize,
               height: usize,
               stride: usize,
               data: Vec<u8>)
               -> Self {
        if (stride * height) != data.len() {
            panic!("Data size ({}) does not match declaration ({} * {})",
                   data.len(),
                   stride,
                   height);
        }

        Buffer {
            format: format,
            width: width,
            height: height,
            stride: stride,
            data: data,
        }
    }

    /// Constructs empty `Buffer`.
    pub fn empty() -> Self {
        Buffer {
            format: PixelFormat::XRGB8888,
            width: 0,
            height: 0,
            stride: 0,
            data: Vec::new(),
        }
    }

    /// Copies data from `other` buffer to `self`.
    pub fn assign_from(&mut self, other: &Buffer) {
        self.format = other.format;
        self.width = other.width;
        self.height = other.height;
        self.stride = other.stride;
        self.data = other.data.clone();
    }

    /// Checks if buffer contains drawable data.
    pub fn is_empty(&self) -> bool {
        (self.width == 0) || (self.height == 0) || (self.stride == 0) || (self.data.len() == 0)
    }

    /// Converts `Buffer` to memory.
    ///
    /// Applications share memory with server. It is their responsibility to inform server which
    /// buffer should be used and avoid drawing to it. The same way inner parts of compositor may
    /// want to instruct render to draw surfaces for them and for simplicity they do this in the
    /// same (relatively unsafe) way as clients. This method converts `Buffer` to `Memory` so it
    /// can be used just as mapped memory obtained from client. It is programmes responsibility to
    /// ensure `Buffer` exists until `Memory` exist and not to draw on it while it may be used for
    /// rendering.
    pub unsafe fn as_memory(&mut self) -> Memory {
        Memory::new_borrowed(self.data.as_mut_ptr(), self.stride * self.height)
    }
}

// -------------------------------------------------------------------------------------------------

impl Image for Buffer {
    #[inline]
    fn get_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    fn get_width(&self) -> usize {
        self.width
    }

    #[inline]
    fn get_height(&self) -> usize {
        self.height
    }
}

// -------------------------------------------------------------------------------------------------

impl Pixmap for Buffer {
    #[inline]
    fn get_format(&self) -> PixelFormat {
        self.format
    }

    #[inline]
    fn get_stride(&self) -> usize {
        self.stride
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    #[inline]
    unsafe fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

// -------------------------------------------------------------------------------------------------

enum MemorySource {
    Mapped,
    Borrowed,
}

// -------------------------------------------------------------------------------------------------

/// Represents memory shared with client.
pub struct Memory {
    data: *mut u8,
    size: usize,
    source: MemorySource,
}

// -------------------------------------------------------------------------------------------------

unsafe impl Send for Memory {}

/// `Memory` is `Sync` as long as its contents do not change. All modifying methods should be
/// marked as `unsafe` and it is programmers responsibility to ensure they are used properly.
unsafe impl Sync for Memory {}

// -------------------------------------------------------------------------------------------------

impl Memory {
    /// Constructs new `Memory` as shared with other application.
    pub fn new_mapped(fd: RawFd, size: usize) -> Result<Memory, errors::Illusion> {
        match mman::mmap(std::ptr::null_mut(),
                         size,
                         mman::PROT_READ | mman::PROT_WRITE,
                         mman::MAP_SHARED,
                         fd,
                         0) {
            Ok(memory) => {
                Ok(Memory {
                       data: memory as *mut u8,
                       size: size,
                       source: MemorySource::Mapped,
                   })
            }
            Err(err) => Err(errors::Illusion::General(format!("Failed to map memory! {:?}", err))),
        }
    }

    /// Constructs new `Memory` from borrowed pointer.
    ///
    /// This is unsafe operation because `Memory` does not owns the data. It must be ensured that
    /// the `Memory` is destructed before the memory.
    unsafe fn new_borrowed(data: *mut u8, size: usize) -> Self {
        Memory {
            data: data,
            size: size,
            source: MemorySource::Borrowed,
        }
    }

    /// Copies contents of given buffer to memory.
    ///
    /// Buffers size must be exactly the size of memory.
    pub unsafe fn absorb(&mut self, buffer: &Buffer) -> Result<(), errors::Illusion> {
        let buffer_size = buffer.as_slice().len();
        if buffer_size == self.size {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), self.data, self.size);
            Ok(())
        } else {
            Err(errors::Illusion::General(format!("Sizes differ: memory map is {}, but buffer {}",
                                                  self.size,
                                                  buffer_size)))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Drop for Memory {
    fn drop(&mut self) {
        match self.source {
            MemorySource::Mapped => {
                let _ = mman::munmap(self.data as *mut _, self.size);
            }
            MemorySource::Borrowed => {}
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents view into memory shared with client.
pub struct MemoryView {
    memory: Arc<Memory>,
    data: *mut u8,
    width: usize,
    height: usize,
    stride: usize,
    format: PixelFormat,
}

// -------------------------------------------------------------------------------------------------

unsafe impl Send for MemoryView {}

/// `MemoryView` is `Sync` as long as it does not provide means to change its internals.
unsafe impl Sync for MemoryView {}

// -------------------------------------------------------------------------------------------------

impl Image for MemoryView {
    #[inline]
    fn get_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    fn get_width(&self) -> usize {
        self.width
    }

    #[inline]
    fn get_height(&self) -> usize {
        self.height
    }
}

// -------------------------------------------------------------------------------------------------

impl Pixmap for MemoryView {
    #[inline]
    fn get_format(&self) -> PixelFormat {
        self.format
    }

    #[inline]
    fn get_stride(&self) -> usize {
        self.stride
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.offset(0), self.height * self.stride) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data.offset(0), self.height * self.stride) }
    }

    #[inline]
    unsafe fn as_ptr(&self) -> *const u8 {
        self.data.offset(0)
    }
}

// -------------------------------------------------------------------------------------------------

impl Clone for MemoryView {
    fn clone(&self) -> Self {
        MemoryView {
            memory: self.memory.clone(),
            data: unsafe { self.data.offset(0) },
            width: self.width,
            height: self.height,
            stride: self.stride,
            format: self.format,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// This structure is used to provide storage for images of different type: shared memory and
/// buffers and return views to them.
pub struct MemoryPool {
    memory: Arc<Memory>,
}

// -------------------------------------------------------------------------------------------------

impl MemoryPool {
    /// Creates new `MemoryPool` containing `Memory`.
    pub fn new(memory: Memory) -> Self {
        MemoryPool { memory: Arc::new(memory) }
    }

    /// Returns `MemoryView`s from `Memory`s stored in `MemoryPool`.
    pub fn get_memory_view(&self,
                           format: PixelFormat,
                           offset: usize,
                           width: usize,
                           height: usize,
                           stride: usize)
                           -> MemoryView {
        // FIXME: Check if boundaries given as arguments are correct.
        MemoryView {
            memory: self.memory.clone(),
            data: unsafe { self.memory.data.offset(offset as isize) },
            width: width,
            height: height,
            stride: stride,
            format: format,
        }
    }

    /// Consumes the pool and if there are no other references to this memory left returns the
    /// `Memory`.
    pub fn take_memory(self) -> Option<Memory> {
        Arc::try_unwrap(self.memory).ok()
    }
}

// -------------------------------------------------------------------------------------------------
