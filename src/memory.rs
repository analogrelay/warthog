use std::ops::{Index, IndexMut, Range, RangeFrom, RangeFull};

/// Represents a growable linear memory, with an optional maximum size
pub struct Memory(Vec<u8>, Option<usize>);

impl Memory {
    pub fn new(min_size: usize, max_size: Option<usize>) -> Memory {
        // TODO: A Vec is probably a bad idea. Probably something with a chain of segments would be better?
        // Or manually allocate?
        Memory(vec![0; min_size], max_size)
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn max_size(&self) -> Option<usize> {
        self.1
    }

    pub fn data(&self) -> &[u8] {
        &self.0
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &u8 {
        &self.data()[index]
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.data()[index]
    }
}

impl Index<RangeFrom<usize>> for Memory {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        &self.data()[index]
    }
}

impl Index<RangeFull> for Memory {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFull) -> &[u8] {
        &self.data()[index]
    }
}

impl IndexMut<usize> for Memory {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<Range<usize>> for Memory {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<RangeFrom<usize>> for Memory {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}

impl IndexMut<RangeFull> for Memory {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut [u8] {
        &mut self.data_mut()[index]
    }
}