use std::io;

#[derive(Debug)]
pub enum ScriptError {
    IoError(io::ErrorKind),
    UnexpectedCharacter(u8),
}

impl From<io::Error> for ScriptError {
    fn from(e: io::Error) -> ScriptError {
        ScriptError::IoError(e.kind())
    }
}