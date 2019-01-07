use std::fmt;

use crate::Trap;

pub enum Error {
    InvalidModule,
    ModuleNotFound { module: String },
    ExportNotFound { module: String, name: String },
    ExportTypeMismatch { module: String, name: String },
    UnsupportedVersion { version: u32 },
    LayoutError,
    Utf8Error(std::string::FromUtf8Error),
    IoError(String),
    UnknownOpcode(u8),
    Trap(Trap),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(format!("{}", e))
    }
}

impl<'a> From<&'a std::io::Error> for Error {
    fn from(e: &std::io::Error) -> Error {
        Error::IoError(format!("{}", e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::Utf8Error(e)
    }
}

impl From<leb128::read::Error> for Error {
    fn from(_: leb128::read::Error) -> Error {
        Error::InvalidModule
    }
}

impl From<std::alloc::LayoutErr> for Error {
    fn from(_: std::alloc::LayoutErr) -> Error {
        Error::LayoutError
    }
}

impl From<Trap> for Error {
    fn from(t: Trap) -> Error {
        Error::Trap(t)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            InvalidModule => write!(f, "invalid module"),
            ModuleNotFound { module: m } => write!(f, "module not found: {}", m),
            ExportNotFound { module: m, name: n } => write!(f, "export not found: {}.{}", m, n),
            ExportTypeMismatch { module: m, name: n } => write!(f, "export type mismatch: {}.{}", m, n),
            UnsupportedVersion { version: v } => write!(f, "unsupported version: {}", v),
            LayoutError => write!(f, "memory layout invalid"),
            Utf8Error(_) => write!(f, "utf-8 conversion error"),
            IoError(s) => write!(f, "i/o error: {}", s),
            UnknownOpcode(o) => write!(f, "unknown opcode: 0x{:2X}", o),
            Trap(t) => write!(f, "trap: {}", t),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}