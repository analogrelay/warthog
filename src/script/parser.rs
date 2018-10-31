use std::str;

use crate::script::{Script, ScriptError};

pub fn parse_str(content: &str) -> Result<Script, ScriptError> {
    parse(content.as_bytes())
}

/// Parses the provided script document into an instance of [`Script`]
///
/// Expects that the entire document has been loaded into the provided string.
pub fn parse(content: &[u8]) -> Result<Script, ScriptError> {
    unimplemented!()
}