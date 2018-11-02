use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{module::ValType, Error};

#[derive(PartialEq, Clone)]
pub struct GlobalType {
    pub valtype: ValType,
    pub mutable: bool,
}

impl GlobalType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<GlobalType, Error> {
        let valtype = ValType::read(reader)?;
        let mutable = match reader.read_u8()? {
            0x00 => false,
            0x01 => true,
            _ => return Err(Error::InvalidModule),
        };
        Ok(GlobalType { valtype, mutable })
    }
}

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(global ")?;
        if self.mutable {
            write!(f, "(mut {})", self.valtype)?;
        } else {
            write!(f, "{}", self.valtype)?;
        }
        write!(f, ")")
    }
}

impl fmt::Debug for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
