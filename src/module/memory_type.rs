use std::{fmt, io};

use crate::{utils, Error};

pub struct MemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

impl MemoryType {
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
