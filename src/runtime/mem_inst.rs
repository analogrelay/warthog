use crate::{module::MemoryType, PAGE_SIZE};

addr_type!(MemAddr);

pub struct MemInst {
    data: Vec<u8>,
    max_size: Option<usize>,
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
            // TODO: A Vec is probably a bad idea. Probably something with a chain of segments would be better?
            data: Vec::with_capacity(min_size),
            max_size: max_size,
        }
    }

    pub fn max_size(&self) -> Option<usize> {
        self.max_size
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
