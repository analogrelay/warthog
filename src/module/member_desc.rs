use std::fmt;
use std::io;

use byteorder::ReadBytesExt;

use crate::{
    module::{GlobalType, MemoryType, TableType},
    utils, Error,
};

pub enum MemberDesc {
    Function(u32),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

impl MemberDesc {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<MemberDesc, Error> {
        let code = reader.read_u8()?;
        match code {
            0x00 => {
                let type_id = utils::read_leb128_u32(reader)?;
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
