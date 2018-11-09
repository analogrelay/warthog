use std::{fmt, io};

use crate::{utils, Error};

#[derive(PartialEq, Clone)]
pub struct MemoryType {
    min: usize,
    max: Option<usize>,
}

impl MemoryType {
    pub fn new(min: usize, max: Option<usize>) -> MemoryType {
        MemoryType { min, max }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<MemoryType, Error> {
        let (min, max) = utils::read_limits(reader)?;
        Ok(MemoryType { min, max })
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }
}

impl fmt::Display for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(memory {}", self.min)?;
        if let Some(max) = self.max {
            write!(f, " {}", max)?;
        }
        write!(f, ")")
    }
}

impl fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
