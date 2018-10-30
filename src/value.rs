use std::fmt;

use crate::module::ValType;

#[derive(PartialEq, Clone, Copy)]
pub enum Value {
    Nil,
    Integer32(i32),
    Integer64(i64),
    Float32(f32),
    Float64(f64),
}

impl Value {
    pub fn typ(&self) -> ValType {
        match self {
            Value::Nil => ValType::Nil,
            Value::Integer32(_) => ValType::Integer32,
            Value::Integer64(_) => ValType::Integer64,
            Value::Float32(_) => ValType::Float32,
            Value::Float64(_) => ValType::Float64,
        }
    }

    pub fn unwrap_i32(&self) -> i32 {
        if let Value::Integer32(v) = self {
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
            Value::Integer32(x) => write!(f, "{}", x),
            Value::Integer64(x) => write!(f, "{}", x),
            Value::Float32(x) => write!(f, "{}", x),
            Value::Float64(x) => write!(f, "{}", x),
        }
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Value {
        Value::Integer32(v as i32)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Value {
        Value::Integer64(v as i64)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Value {
        Value::Integer32(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Value {
        Value::Integer64(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Value {
        Value::Float32(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Float64(v)
    }
}
