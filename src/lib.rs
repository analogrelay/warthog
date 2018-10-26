extern crate byteorder;
extern crate leb128;

mod error;
mod utils;

pub mod module;
pub mod reader;

pub use crate::error::Error;
