use std::io;

#[derive(Debug)]
pub struct ScriptError(ScriptErrorKind, usize, Option<String>);

impl ScriptError {
    pub fn new<I: Into<ScriptErrorKind>>(err: I, position: usize) -> ScriptError {
        ScriptError(err.into(), position, None)
    }

    pub fn detailed<I: Into<ScriptErrorKind>, S: Into<String>>(err: I, position: usize, detail: S) -> ScriptError {
        ScriptError(err.into(), position, Some(detail.into()))
    }

    pub fn kind(&self) -> ScriptErrorKind {
        self.0
    }

    pub fn position(&self) -> usize {
        self.1
    }

    pub fn detail(&self) -> Option<&str> {
        match self.2 {
            Some(ref s) => Some(s.as_str()),
            None => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptErrorKind {
    InvalidUtf8,
    InvalidEscape,
    UnexpectedEof,
    IoError(io::ErrorKind),
    UnexpectedCharacter(char),
}

impl From<std::io::Error> for ScriptErrorKind {
    fn from(e: std::io::Error) -> ScriptErrorKind {
        ScriptErrorKind::IoError(e.kind())
    }
}

impl<'a> From<&'a std::io::Error> for ScriptErrorKind {
    fn from(e: &std::io::Error) -> ScriptErrorKind {
        ScriptErrorKind::IoError(e.kind())
    }
}

impl From<std::string::FromUtf8Error> for ScriptErrorKind {
    fn from(_: std::string::FromUtf8Error) -> ScriptErrorKind {
        ScriptErrorKind::InvalidUtf8
    }
}