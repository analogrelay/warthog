// Elevate certain warnings to errors in debug and ALL warnings to errors in release
#![deny(unconditional_recursion)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

extern crate byteorder;
extern crate leb128;

mod error;
mod memory;
mod utils;
mod value;

pub mod builder;
pub mod interp;
pub mod module;
pub mod parser;
pub mod reader;
pub mod runtime;
pub mod synth;

pub use crate::error::Error;
pub use crate::memory::Memory;
pub use crate::value::Value;

pub const PAGE_SIZE: usize = 65536;
