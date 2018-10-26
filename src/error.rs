#[derive(Debug)]
pub enum Error {
    InvalidModule,
    UnsupportedVersion(u32),
    Utf8Error(std::string::FromUtf8Error),
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
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
