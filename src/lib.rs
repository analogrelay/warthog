// Elevate certain warnings to errors in debug and ALL warnings to errors in release
#![deny(warnings)]
// Quiet down some warnings when running tests.
#![cfg_attr(test, allow(dead_code))]

extern crate byteorder;
extern crate leb128;

#[macro_use]
extern crate lazy_static;

mod error;
mod location;
mod memory;
mod sparse_vec;
mod utils;
mod value;

pub mod builder;
pub mod hosting;
pub mod interp;
pub mod module;
pub mod parser;
pub mod reader;
pub mod runtime;
pub mod script;

pub use crate::error::Error;
pub use crate::location::Location;
pub use crate::memory::Memory;
pub use crate::value::Value;

pub(crate) use crate::sparse_vec::SparseVec;

pub const PAGE_SIZE: usize = 65536;
