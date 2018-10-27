extern crate byteorder;
extern crate leb128;

mod error;
mod utils;

pub mod module;
pub mod reader;
pub mod runtime;

pub use crate::error::Error;
