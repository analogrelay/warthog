use std::{fmt, io};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{utils, Error, Value};

#[derive(PartialEq, Clone)]
pub struct BranchTable(Vec<u32>, u32);

impl BranchTable {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<BranchTable, Error> {
        let branches = utils::read_vec(reader, utils::read_leb128_u32)?;
        let else_case = utils::read_leb128_u32(reader)?;
        Ok(BranchTable(branches, else_case))
    }
}

impl fmt::Display for BranchTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for branch in self.0.iter() {
            write!(f, "{}, ", branch)?;
        }
        write!(f, "{}", self.1)
    }
}

impl fmt::Debug for BranchTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

include!(concat!(env!("OUT_DIR"), "/instructions.g.rs"));

impl Instruction {
    pub fn read_sequence<R: io::Read>(reader: &mut R) -> Result<Vec<Instruction>, Error> {
        let mut insts = Vec::new();
        let mut blocks = 1;
        loop {
            let inst = Instruction::read(reader)?;
            if inst.is_block() {
                blocks += 1;
            } else if inst == Instruction::End {
                blocks -= 1;
                if blocks == 0 {
                    return Ok(insts);
                }
            }
            insts.push(inst);
        }
    }
}

#[inline]
fn read_idx<R: io::Read>(reader: &mut R) -> Result<u32, Error> {
    Ok(utils::read_leb128_u32(reader)?)
}

#[inline]
fn read_i32<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    Ok(Value::I32(utils::read_leb128_s(reader)?))
}

#[inline]
fn read_i64<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    Ok(Value::I64(utils::read_leb128_s(reader)?))
}

#[inline]
fn read_f32<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    let bits = reader.read_u32::<LittleEndian>()?;
    Ok(Value::F32(f32::from_bits(bits)))
}

#[inline]
fn read_f64<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    let bits = reader.read_u64::<LittleEndian>()?;
    Ok(Value::F64(f64::from_bits(bits)))
}
