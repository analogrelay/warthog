// Elevate certain warnings to errors in debug and ALL warnings to errors in release
#![deny(unconditional_recursion)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
// Quiet down some warnings when running tests.
#![cfg_attr(test, allow(dead_code))]

extern crate byteorder;
extern crate leb128;

#[macro_use]
extern crate lazy_static;

mod error;
mod memory;
mod utils;
mod value;

pub mod builder;
pub mod interp;
pub mod module;
pub mod reader;
pub mod runtime;
pub mod synth;
pub mod text;

pub use crate::error::Error;
pub use crate::memory::Memory;
pub use crate::value::Value;

pub const PAGE_SIZE: usize = 65536;
