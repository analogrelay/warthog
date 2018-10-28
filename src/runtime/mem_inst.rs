use std::{
    ops::Index, ops::IndexMut, ops::Range, ops::RangeFrom, ops::RangeFull, slice::SliceIndex,
};

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
            // Or manually allocate?
            data: vec![0; min_size],
            max_size: max_size,
        }
    }

    pub fn max_size(&self) -> Option<usize> {
        self.max_size
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Index<usize> for MemInst {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &u8 {
        &self.data()[index]
    }
}

impl Index<Range<usize>> for MemInst {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.data()[index]
    }
}

impl Index<RangeFrom<usize>> for MemInst {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        &self.data()[index]
    }
}

impl Index<RangeFull> for MemInst {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFull) -> &[u8] {
        &self.data()[index]
    }
}

impl IndexMut<usize> for MemInst {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<Range<usize>> for MemInst {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<RangeFrom<usize>> for MemInst {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<RangeFull> for MemInst {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}
