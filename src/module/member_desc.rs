use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{
    module::{GlobalType, MemoryType, TableType},
    utils, Error,
};

#[derive(PartialEq, Clone)]
pub enum MemberDesc {
    Function(usize),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

impl MemberDesc {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<MemberDesc, Error> {
        let code = reader.read_u8()?;
        match code {
            0x00 => {
                let type_id = utils::read_leb128_u32(reader)? as usize;
                Ok(MemberDesc::Function(type_id))
            }
            0x01 => Ok(MemberDesc::Table(TableType::read(reader)?)),
            0x02 => Ok(MemberDesc::Memory(MemoryType::read(reader)?)),
            0x03 => Ok(MemberDesc::Global(GlobalType::read(reader)?)),
            _ => Err(Error::InvalidModule),
        }
    }
}

impl fmt::Display for MemberDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemberDesc::Function(x) => write!(f, "(func {})", x),
            MemberDesc::Table(x) => write!(f, "{}", x),
            MemberDesc::Memory(x) => write!(f, "{}", x),
            MemberDesc::Global(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Debug for MemberDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
