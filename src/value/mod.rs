use std::fmt;

use crate::{TrapCause, types::ValType};

pub mod ops;

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

impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::I32(x) => write!(f, "{:X}", x),
            Value::I64(x) => write!(f, "{:X}", x),
            Value::F32(x) => write!(f, "{:X}", x.to_bits()),
            Value::F64(x) => write!(f, "{:X}", x.to_bits()),
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

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::I32(if b { 1 } else { 0 })
    }
}

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
                    x => Err(TrapCause::TypeMismatch {
                        expected: ValType::$v,
                        actual: x.typ(),
                    }),
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
