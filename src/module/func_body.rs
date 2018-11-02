use std::{fmt, io};

use crate::{
    module::{Instruction, Local},
    utils, Error,
};

#[derive(PartialEq, Clone)]
pub struct FuncBody {
    pub locals: Vec<Local>,
    pub body: Vec<Instruction>,
}

impl FuncBody {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<FuncBody, Error> {
        let size = utils::read_leb128_u32(reader)?;
        let locals = utils::read_vec(reader, |r| Local::read(r))?;
        let body = utils::read_instructions(reader)?;

        Ok(FuncBody { locals, body })
    }
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = true;
        for param in self.locals.iter() {
            if start {
                start = false;
            } else {
                write!(f, " ");
            }
            write!(f, "{}", param)?;
        }
        for inst in self.body.iter() {
            if start {
                start = false;
            } else {
                write!(f, " ");
            }
            write!(f, "{}", inst)?;
        }
        Ok(())
    }
}

impl fmt::Debug for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
