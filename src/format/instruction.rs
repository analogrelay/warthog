use std::io;
use std::fmt;

use utils;
use error::Error;

use byteorder::ReadBytesExt;

#[derive(Eq, PartialEq)]
pub enum Instruction {
    ConstI32(i32),
    End,
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Instruction, Error> {
        let opcode = reader.read_u8()?;
        match opcode {
            0x0B => Ok(Instruction::End),
            0x41 => {
                let val = utils::read_leb128_i32(reader)?;
                Ok(Instruction::ConstI32(val))
            },
            x => panic!("Instruction not implemented: 0x{:X}", x),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::ConstI32(x) => write!(f, "i32.const {}", x),
            Instruction::End => write!(f, "end"),
        }
    }
}