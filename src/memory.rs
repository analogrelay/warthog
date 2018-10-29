use std::{
    alloc::{self, Layout},
    mem, slice,
};

use crate::error::Error;

/// Represents a growable linear memory, with an optional maximum size
///
/// WebAssembly memory is inherently "unsafe" in Rust terms because the
/// WebAssembly runtime doesn't expect the same safety guarantees. It's up to the
/// WebAssembly program to ensure safety
pub struct Memory(*mut u8, usize, Option<usize>);

impl Memory {
    pub fn new(min_size: usize, max_size: Option<usize>) -> Result<Memory, Error> {
        let layout = Layout::from_size_align(min_size, mem::align_of::<u8>())?;
        unsafe {
            let mem = alloc::alloc_zeroed(layout);
            Ok(Memory(mem, min_size, max_size))
        }
    }

    pub fn ptr(&self) -> *mut u8 {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn max_size(&self) -> Option<usize> {
        self.2
    }

    pub unsafe fn data(&self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.0, self.1)
    }
}
