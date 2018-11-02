use std::{fmt, io};

use crate::{utils, Error};

#[derive(PartialEq, Clone)]
pub struct MemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

impl MemoryType {
    pub fn new(min: u32, max: Option<u32>) -> MemoryType {
        MemoryType { min, max }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<MemoryType, Error> {
        let (min, max) = utils::read_limits(reader)?;
        Ok(MemoryType { min, max })
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
