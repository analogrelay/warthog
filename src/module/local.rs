use std::{fmt, io};

use crate::{module::ValType, utils, Error};

#[derive(Clone)]
pub struct Local {
    pub count: u32,
    pub typ: ValType,
}

impl Local {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Local, Error> {
        let count = utils::read_leb128_u32(reader)?;
        let typ = ValType::read(reader)?;

        Ok(Local { count, typ })
    }
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(local")?;
        for _ in 0..self.count {
            write!(f, " {}", self.typ)?;
        }
        write!(f, ")")
    }
}
