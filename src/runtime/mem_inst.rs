use crate::{module::MemoryType, Error, Memory, PAGE_SIZE};

addr_type!(MemAddr);

pub struct MemInst {
    mem: Memory,
}

impl MemInst {
    pub fn from_type(mem_type: &MemoryType) -> Result<MemInst, Error> {
        match mem_type.max {
            Some(max) => MemInst::new(
                mem_type.min as usize * PAGE_SIZE,
                Some(max as usize * PAGE_SIZE),
            ),
            None => MemInst::new(mem_type.min as usize * PAGE_SIZE, None),
        }
    }

    pub fn new(min_size: usize, max_size: Option<usize>) -> Result<MemInst, Error> {
        Ok(MemInst {
            mem: Memory::new(min_size, max_size)?,
        })
    }

    pub fn memory(&self) -> &Memory {
        &self.mem
    }
}
