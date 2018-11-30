// Elevate certain warnings to errors in debug and ALL warnings to errors in release
// #![deny(warnings)]
// Quiet down some warnings when running tests.
#![cfg_attr(test, allow(dead_code))]

extern crate byteorder;
extern crate leb128;

// This module has to be imported first because macros are processed
// in a single pass.
mod macros;

mod error;
pub mod instruction;
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
pub use crate::instruction::Instruction;
pub use crate::location::Location;
pub use crate::memory::Memory;
pub use crate::trap::Trap;
pub use crate::value::{FromValue, ValType, Value};

pub(crate) use crate::sparse_vec::SparseVec;

pub const PAGE_SIZE: usize = 65536;
