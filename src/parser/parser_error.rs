use std::io;

use crate::parser::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError(ParserErrorKind, (usize, usize), Option<String>);

impl ParserError {
    pub fn new<I: Into<ParserErrorKind>>(err: I, range: (usize, usize)) -> ParserError {
        ParserError(err.into(), range, None)
    }

    pub fn detailed<I: Into<ParserErrorKind>, S: Into<String>>(
        err: I,
        range: (usize, usize),
        detail: S,
    ) -> ParserError {
        ParserError(err.into(), range, Some(detail.into()))
    }

    pub fn kind(&self) -> &ParserErrorKind {
        &self.0
    }

    pub fn range(&self) -> (usize, usize) {
        self.1
    }

    pub fn detail(&self) -> Option<&str> {
        match self.2 {
            Some(ref s) => Some(s.as_str()),
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    InvalidUtf8,
    InvalidEscape,
    UnexpectedEof,
    IoError(io::ErrorKind),
    UnexpectedCharacter(char),
    UnexpectedToken(TokenKind),
    UnexpectedAtom(String),
}

impl From<std::io::Error> for ParserErrorKind {
    fn from(e: std::io::Error) -> ParserErrorKind {
        ParserErrorKind::IoError(e.kind())
    }
}

impl<'a> From<&'a std::io::Error> for ParserErrorKind {
    fn from(e: &std::io::Error) -> ParserErrorKind {
        ParserErrorKind::IoError(e.kind())
    }
}

impl From<std::string::FromUtf8Error> for ParserErrorKind {
    fn from(_: std::string::FromUtf8Error) -> ParserErrorKind {
        ParserErrorKind::InvalidUtf8
    }
}
