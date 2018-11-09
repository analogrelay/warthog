use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::Error;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValType {
    Nil = 0x00,
    Integer32 = 0x7F,
    Integer64 = 0x7E,
    Float32 = 0x7D,
    Float64 = 0x7C,
}

impl ValType {
    pub fn from_u8(v: u8) -> Result<ValType, Error> {
        match v {
            0x7F => Ok(ValType::Integer32),
            0x7E => Ok(ValType::Integer64),
            0x7D => Ok(ValType::Float32),
            0x7C => Ok(ValType::Float64),
            _ => Err(Error::InvalidModule),
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<ValType, Error> {
        let v = reader.read_u8()?;
        ValType::from_u8(v)
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            ValType::Nil => "nil",
            ValType::Integer32 => "i32",
            ValType::Integer64 => "i64",
            ValType::Float32 => "f32",
            ValType::Float64 => "f64",
        };
        write!(f, "{}", v)
    }
}

impl fmt::Debug for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
