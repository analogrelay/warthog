use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{Error, TrapCause};

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
    fn from_value(v: Value) -> Result<Self, TrapCause>;
}

macro_rules! impl_from_value {
    ($repr: ty, $v: ident) => {
        impl FromValue for $repr {
            fn from_value(v: Value) -> Result<Self, TrapCause> {
                match v {
                    Value::$v(x) => Ok(x as $repr),
                    Value::Nil => Err(TrapCause::StackUnderflow),
                    x => Err(TrapCause::TypeMismatch { expected: ValType::$v, actual: x.typ() }),
                }
            }
        }
    };
}

impl_from_value!(u32, I32);
impl_from_value!(u64, I64);
impl_from_value!(i32, I32);
impl_from_value!(i64, I64);
impl_from_value!(f32, F32);
impl_from_value!(f64, F64);

/// Basic arithmetic operations defined for BOTH integers and floats.
pub trait ArithmeticOps<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
    fn sub(self, rhs: RHS) -> Self::Output;
    fn mul(self, rhs: RHS) -> Self::Output;
    fn div(self, rhs: RHS) -> Result<Self::Output, TrapCause>;
}

macro_rules! impl_arith_for_integer {
    ($t: ty) => {
        impl ArithmeticOps<$t> for $t {
            type Output = $t;

            fn add(self, rhs: $t) -> $t {
                self.wrapping_add(rhs)
            }

            fn sub(self, rhs: $t) -> $t {
                self.wrapping_sub(rhs)
            }

            fn mul(self, rhs: $t) -> $t {
                self.wrapping_mul(rhs)
            }

            fn div(self, rhs: $t) -> Result<$t, TrapCause> {
                if rhs == 0 {
                    Err(TrapCause::IntegerDivideByZero)
                } else {
                    match self.checked_div(rhs) {
                        Some(y) => Ok(y),
                        None => Err(TrapCause::IntegerOverflow)
                    }
                }
            }
        }
    };
}

impl_arith_for_integer!(u32);
impl_arith_for_integer!(u64);
impl_arith_for_integer!(i32);
impl_arith_for_integer!(i64);

/// Operations defined only for Integers
pub trait IntegerOps<RHS = Self> {
    type Output;

    fn clz(self) -> Self::Output;
    fn ctz(self) -> Self::Output;
    fn popcnt(self) -> Self::Output;
    fn shl(self, rhs: RHS) -> Self::Output;
    fn shr(self, rhs: RHS) -> Self::Output;
    fn rotl(self, rhs: RHS) -> Self::Output;
    fn rotr(self, rhs: RHS) -> Self::Output;
    fn rem(self, rhs: RHS) -> Result<Self::Output, TrapCause>;
}

macro_rules! impl_integer {
    ($t: ty) => {
        impl IntegerOps<$t> for $t {
            type Output = $t;

            fn clz(self) -> $t {
                self.leading_zeros() as $t
            }

            fn ctz(self) -> $t {
                self.trailing_zeros() as $t
            }

            fn popcnt(self) -> $t {
                self.count_ones() as $t
            }

            fn shl(self, rhs: $t) -> Self::Output {
                self.wrapping_shl(rhs as u32) as $t
            }

            fn shr(self, rhs: $t) -> Self::Output {
                self.wrapping_shr(rhs as u32) as $t
            }

            fn rotl(self, rhs: $t) -> $t {
                self.rotate_left(rhs as u32)
            }

            fn rotr(self, rhs: $t) -> $t {
                self.rotate_right(rhs as u32)
            }

            fn rem(self, rhs: $t) -> Result<$t, TrapCause> {
                if rhs == 0 {
                    Err(TrapCause::IntegerDivideByZero)
                } else {
                    Ok(self.overflowing_rem(rhs).0)
                }
            }
        }
    };
}

impl_integer!(u32);
impl_integer!(u64);
impl_integer!(i32);
impl_integer!(i64);