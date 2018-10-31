#[derive(Debug)]
pub enum Error {
    InvalidModule,
    ModuleNotFound { module: String },
    ExportNotFound { module: String, name: String },
    ExportTypeMismatch { module: String, name: String },
    UnsupportedVersion { version: u32 },
    LayoutError,
    Utf8Error(std::string::FromUtf8Error),
    IoError(String),
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
    fn from(l: std::alloc::LayoutErr) -> Error {
        Error::LayoutError
    }
}
