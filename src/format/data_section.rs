use std::fmt;
use std::io;

use error::Error;
use utils;

use super::{Instruction, Section};

use byteorder::ReadBytesExt;

pub struct DataSection {
    data: Vec<DataItem>,
}

impl Section for DataSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<DataSection, Error> {
        let data = utils::read_vec(reader, |r| DataItem::read(r))?;

        Ok(DataSection { data })
    }
}

impl DataSection {
    pub fn data(&self) -> &[DataItem] {
        &self.data
    }
}

pub struct DataItem {
    pub index: u32,
    pub expr: Vec<Instruction>,
    pub init: Vec<u8>,
}

impl DataItem {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<DataItem, Error> {
        let index = utils::read_leb128_u32(reader)?;
        let expr = utils::read_instructions(reader)?;
        let init = utils::read_vec(reader, |r| Ok(r.read_u8()?))?;
        Ok(DataItem { index, expr, init })
    }
}

impl fmt::Display for DataItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data")?;
        if self.index > 0 {
            write!(f, " {}", self.index)?;
        }
        if self.expr.len() > 1 {
            write!(f, " (offset")?;
        }
        write!(f, " (")?;
        let mut first = true;
        for inst in self.expr.iter() {
            if first {
                write!(f, "{}", inst)?;
                first = false;
            } else {
                write!(f, " {}", inst)?;
            }
        }
        write!(f, ")")?;
        if self.expr.len() > 1 {
            write!(f, ")")?;
        }

        // Format the data
        write!(f, " \"")?;
        for byt in self.init.iter().cloned() {
            unsafe {
                let bytes: Vec<_> = std::ascii::escape_default(byt).collect();
                let escaped = std::str::from_utf8_unchecked(&bytes);
                write!(f, "{}", escaped)?;
            }
        }
        write!(f, "\")")
    }
}
