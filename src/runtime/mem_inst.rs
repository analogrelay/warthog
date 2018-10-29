use std::{ops::{Deref, DerefMut}, cell::RefCell};

use crate::{module::MemoryType, Memory, PAGE_SIZE};

addr_type!(MemAddr);

pub struct MemInst {
    mem: RefCell<Memory>,
}

impl MemInst {
    pub fn from_type(mem_type: &MemoryType) -> MemInst {
        match mem_type.max {
            Some(max) => MemInst::new(
                mem_type.min as usize * PAGE_SIZE,
                Some(max as usize * PAGE_SIZE),
            ),
            None => MemInst::new(mem_type.min as usize * PAGE_SIZE, None),
        }
    }

    pub fn new(min_size: usize, max_size: Option<usize>) -> MemInst {
        MemInst {
            mem: RefCell::new(Memory::new(min_size, max_size)),
        }
    }

    pub fn memory<'a>(&'a self) -> impl 'a + Deref<Target=Memory> {
        self.mem.borrow()
    }

    pub fn memory_mut<'a>(&'a self) -> impl 'a + DerefMut<Target=Memory> {
        self.mem.borrow_mut()
    }
}
