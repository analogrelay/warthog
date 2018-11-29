// Elevate certain warnings to errors in debug and ALL warnings to errors in release
#![deny(warnings)]
// Quiet down some warnings when running tests.
#![cfg_attr(test, allow(dead_code))]

extern crate byteorder;
extern crate leb128;

macro_rules! type_to_valtype {
    (i32) => ($crate::ValType::I32),
    (i64) => ($crate::ValType::I64),
    (u32) => ($crate::ValType::I32),
    (u64) => ($crate::ValType::I64),
    (f32) => ($crate::ValType::F32),
    (f64) => ($crate::ValType::F64),
}

mod error;
mod location;
mod memory;
mod sparse_vec;
mod trap;
mod utils;
mod value;

pub mod builder;
pub mod hosting;
pub mod interp;
pub mod module;
pub mod reader;
pub mod runtime;

pub use crate::error::Error;
pub use crate::location::Location;
pub use crate::memory::Memory;
pub use crate::trap::Trap;
pub use crate::value::{FromValue, ValType, Value};

pub(crate) use crate::sparse_vec::SparseVec;

pub const PAGE_SIZE: usize = 65536;
