use std::{fmt, io};

use crate::{utils, Error, Instruction, ValType};

#[derive(PartialEq, Clone)]
pub struct FuncBody {
    locals: Vec<ValType>,
    body: Vec<Instruction>,
}

impl FuncBody {
    pub fn new(locals: Vec<ValType>, body: Vec<Instruction>) -> FuncBody {
        FuncBody { locals, body }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<FuncBody, Error> {
        utils::read_leb128_u32(reader)?;

        // Locals is a vec, but each item also indicates repeated locals, so we
        // don't use read_vec because we want the expanded form
        let size = utils::read_leb128_u32(reader)?;
        let mut locals = Vec::new();
        for _ in 0..size {
            let count = utils::read_leb128_u32(reader)?;
            let typ = ValType::read(reader)?;
            for _ in 0..count {
                locals.push(typ);
            }
        }

        let body = utils::read_instructions(reader)?;

        Ok(FuncBody { locals, body })
    }

    pub fn locals(&self) -> &[ValType] {
        &self.locals
    }

    pub fn body(&self) -> &[Instruction] {
        &self.body
    }
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = true;
        for param in self.locals.iter() {
            if start {
                start = false;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{}", param)?;
        }
        for inst in self.body.iter() {
            if start {
                start = false;
            } else {
                write!(f, " ")?;
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
