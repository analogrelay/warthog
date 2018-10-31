use std::io;

#[derive(Debug)]
pub enum ScriptError {
    InvalidUtf8,
    IoError(io::ErrorKind),
    UnexpectedCharacter(u8),
    UnexpectedEof,
}

impl From<std::io::Error> for ScriptError {
    fn from(e: std::io::Error) -> ScriptError {
        ScriptError::IoError(e.kind())
    }
}

impl<'a> From<&'a std::io::Error> for ScriptError {
    fn from(e: &std::io::Error) -> ScriptError {
        ScriptError::IoError(e.kind())
    }
}