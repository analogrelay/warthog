use std::io;

#[derive(Debug)]
pub enum ScriptError {
    InvalidUtf8,
    IoError(io::ErrorKind),
    UnexpectedCharacter(u8),
}

impl<'a> From<&'a io::Error> for ScriptError {
    fn from(e: &io::Error) -> ScriptError {
        ScriptError::IoError(e.kind())
    }
}