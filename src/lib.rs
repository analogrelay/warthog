extern crate byteorder;
extern crate leb128;

mod error;
mod utils;

pub mod interp;
pub mod module;
pub mod reader;
pub mod runtime;
pub mod synth;

pub use crate::error::Error;

pub const PAGE_SIZE: usize = 65536;
