use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{utils, Error};

#[repr(u8)]
#[derive(Copy, PartialEq, Clone)]
pub enum ElemType {
    AnyFunc = 0x70,
}

#[derive(PartialEq, Clone)]
pub struct TableType {
    elem_type: ElemType,
    min: usize,
    max: Option<usize>,
}

impl TableType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<TableType, Error> {
        let elem_type = reader.read_u8()?;
        if elem_type != 0x70 {
            Err(Error::InvalidModule)
        } else {
            let (min, max) = utils::read_limits(reader)?;
            Ok(TableType {
                elem_type: ElemType::AnyFunc,
                min,
                max,
            })
        }
    }

    pub fn elem_type(&self) -> ElemType {
        self.elem_type
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
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

impl fmt::Debug for TableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
