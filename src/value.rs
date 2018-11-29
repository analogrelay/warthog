use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{Error, Trap};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValType {
    Nil = 0x00,
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl ValType {
    pub fn from_u8(v: u8) -> Result<ValType, Error> {
        match v {
            0x7F => Ok(ValType::I32),
            0x7E => Ok(ValType::I64),
            0x7D => Ok(ValType::F32),
            0x7C => Ok(ValType::F64),
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
            ValType::I32 => "i32",
            ValType::I64 => "i64",
            ValType::F32 => "f32",
            ValType::F64 => "f64",
        };
        write!(f, "{}", v)
    }
}

impl fmt::Debug for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Value {
    Nil,
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

impl Value {
    pub fn typ(&self) -> ValType {
        match self {
            Value::Nil => ValType::Nil,
            Value::I32(_) => ValType::I32,
            Value::I64(_) => ValType::I64,
            Value::F32(_) => ValType::F32,
            Value::F64(_) => ValType::F64,
        }
    }

    pub fn unwrap_u32(&self) -> u32 {
        if let Value::I32(v) = self {
            *v
        } else {
            panic!("Expected an i32, but the value is a {}", self.typ());
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::I32(x) => write!(f, "{}", x),
            Value::I64(x) => write!(f, "{}", x),
            Value::F32(x) => write!(f, "{}", x),
            Value::F64(x) => write!(f, "{}", x),
        }
    }
}

macro_rules! impl_from {
    ($t: ty, $repr: ty, $v: ident) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Value {
                Value::$v(v as $repr)
            }
        }
    };
}

impl_from!(u32, u32, I32);
impl_from!(u64, u64, I64);
impl_from!(i32, u32, I32);
impl_from!(i64, u64, I64);
impl_from!(f32, f32, F32);
impl_from!(f64, f64, F64);

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self, Trap>;
}

macro_rules! impl_from_value {
    ($repr: ty, $v: ident) => {
        impl FromValue for $repr {
            fn from_value(v: Value) -> Result<$repr, Trap> {
                match v {
                    Value::$v(x) => Ok(x as $repr),
                    Value::Nil => Err("Stack underflow.".into()),
                    x => Err(format!(
                        "Type mismatch. Expected '{}' but got '{}'",
                        ValType::$v,
                        x.typ()
                    ).into()),
                }
            }
        }
    };
}

impl_from_value!(u32, I32);
impl_from_value!(u64, I64);
impl_from_value!(i32, I32);
impl_from_value!(i64, I64);
