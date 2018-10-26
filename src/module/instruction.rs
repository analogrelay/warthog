use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{utils, Error};

#[derive(Eq, PartialEq)]
pub enum Instruction {
    ConstI32(i32),
    Call(u32),
    End,
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Instruction, Error> {
        let opcode = reader.read_u8()?;
        match opcode {
            0x0B => Ok(Instruction::End),
            0x41 => Ok(Instruction::ConstI32(utils::read_leb128_i32(reader)?)),
            0x10 => Ok(Instruction::Call(utils::read_leb128_u32(reader)?)),
            x => panic!("Instruction not implemented: 0x{:X}", x),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::ConstI32(x) => write!(f, "i32.const {}", x),
            Instruction::Call(x) => write!(f, "call {}", x),
            Instruction::End => write!(f, "end"),
        }
    }
}
