// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module provides buffer memory management tools.
//!
//! Clients usually share images with server using shared memory. Client creates big shared memory
//! and then tells server which parts server should use for drawing on surfaces. `MappedMemory`
//! represents these shared memory (and is owner of data) and `MemoryView` can be used to view parts
//! of it.
//!
//! Images to redraw can be also created locally or read from file. These images can be stored in
//! `Buffer`. `Buffer` and `MemoryView` implement `Pixmap` trait, but `Buffer` is owned of its data
//! unlike `MemoryView`.
//!
//! `MemoryPool` is used to provide mechanism for storing mapped and buffered memory. The only way
//! to construct `MemoryView` is through `MemoryPool`. Both have counted reference to
//! `MappedMemory` and `MappedMemory` is destructed when its reference count goes to zero, so
//! `MemoryView`s can be safely used even after `MappedMemory` was removed from `MemoryPool`.

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
    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    unsafe fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents memory shared with client.
pub struct MappedMemory {
    data: *mut u8,
    size: usize,
}

// -------------------------------------------------------------------------------------------------

unsafe impl Send for MappedMemory {}

/// `MappedMemory` is `Sync` as long as its contents do not change. All modifying methods should be
/// marked as `unsafe` and it is programmers responsibility to ensure they are used properly.
unsafe impl Sync for MappedMemory {}

// -------------------------------------------------------------------------------------------------

impl MappedMemory {
    /// Constructs new `MappedMemory`.
    pub fn new(fd: RawFd, size: usize) -> Result<MappedMemory, errors::Illusion> {
        match mman::mmap(std::ptr::null_mut(),
                         size,
                         mman::PROT_READ | mman::PROT_WRITE,
                         mman::MAP_SHARED,
                         fd,
                         0) {
            Ok(memory) => {
                Ok(MappedMemory {
                       data: memory as *mut u8,
                       size: size,
                   })
            }
            Err(err) => Err(errors::Illusion::General(format!("Failed to map memory! {:?}", err))),
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

impl Drop for MappedMemory {
    fn drop(&mut self) {
        let _ = mman::munmap(self.data as *mut _, self.size);
    }
}

// -------------------------------------------------------------------------------------------------

/// Represents view into memory shared with client.
pub struct MemoryView {
    memory: Arc<MemoryKind>,
    data: *const u8,
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
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.offset(0), self.height * self.stride) }
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

/// Enumeration used by `MemoryPool` to keep track of data types it holds.
enum MemoryKind {
    Mapped(MappedMemory),
    Buffered(Buffer),
}

// -------------------------------------------------------------------------------------------------

/// This structure is used to provide storage for images of different type: shared memory and
/// buffers and return views to them.
pub struct MemoryPool {
    memory: Arc<MemoryKind>,
}

// -------------------------------------------------------------------------------------------------

impl MemoryPool {
    /// Creates new `MemoryPool` containing `MappedMemory`.
    pub fn new_from_mapped_memory(memory: MappedMemory) -> Self {
        MemoryPool { memory: Arc::new(MemoryKind::Mapped(memory)) }
    }

    /// Creates new `MemoryPool` containing `Buffer`.
    pub fn new_from_buffer(buffer: Buffer) -> Self {
        MemoryPool { memory: Arc::new(MemoryKind::Buffered(buffer)) }
    }

    /// Returns `MemoryView`s into `Buffer`s and `MappedMemory`s stored in `MemoryPool`.
    pub fn get_memory_view(&self,
                           format: PixelFormat,
                           offset: usize,
                           width: usize,
                           height: usize,
                           stride: usize)
                           -> MemoryView {
        // FIXME: Check if boundaries given as arguments are correct.
        match *self.memory {
            MemoryKind::Mapped(ref map) => {
                MemoryView {
                    memory: self.memory.clone(),
                    data: unsafe { map.data.offset(offset as isize) },
                    width: width,
                    height: height,
                    stride: stride,
                    format: format,
                }
            }
            MemoryKind::Buffered(ref buffer) => {
                MemoryView {
                    memory: self.memory.clone(),
                    data: unsafe { buffer.as_ptr() as *const u8 },
                    width: width,
                    height: height,
                    stride: stride,
                    format: format,
                }
            }
        }
    }

    /// Consumes the pool and if
    /// - it was created from mapped memory
    /// - there are no other references to this memory left
    /// returns the mapped memory.
    pub fn take_mapped_memory(self) -> Option<MappedMemory> {
        match Arc::try_unwrap(self.memory) {
            Ok(kind) => {
                match kind {
                    MemoryKind::Mapped(mapped_memory) => Some(mapped_memory),
                    MemoryKind::Buffered(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------
