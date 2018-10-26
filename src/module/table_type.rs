use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{utils, Error};

pub struct TableType {
    pub elem_type: u8,
    pub min: u32,
    pub max: Option<u32>,
}

impl TableType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<TableType, Error> {
        let elem_type = reader.read_u8()?;
        if elem_type != 0x70 {
            Err(Error::InvalidModule)
        } else {
            let (min, max) = utils::read_limits(reader)?;
            Ok(TableType {
                elem_type,
                min,
                max,
            })
        }
    }
}

impl fmt::Display for TableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(table {}", self.min)?;
        if let Some(max) = self.max {
            write!(f, " {}", max)?;
        }
        write!(f, " anyfunc)")
    }
}
