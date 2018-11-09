use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{utils, Error, Value};

#[derive(Clone, PartialEq)]
pub enum Instruction {
    Const(Value),
    Call(usize),
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Option<Instruction>, Error> {
        let opcode = reader.read_u8()?;
        match opcode {
            0x0B => Ok(None),
            0x41 => Ok(Some(Instruction::Const(Value::Integer32(
                utils::read_leb128_i32(reader)?,
            )))),
            0x10 => Ok(Some(Instruction::Call(utils::read_leb128_u32(reader)? as usize))),
            x => panic!("Instruction not implemented: 0x{:X}", x),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Const(x) => write!(f, "({}.const {})", x.typ(), x),
            Instruction::Call(x) => write!(f, "(call {})", x),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
