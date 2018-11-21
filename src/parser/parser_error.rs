use std::{fmt, io};

use crate::parser::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError(ParserErrorKind, TextRange, Option<String>);

impl ParserError {
    pub fn new<I: Into<ParserErrorKind>, R: Into<TextRange>>(err: I, range: R) -> ParserError {
        ParserError(err.into(), range.into(), None)
    }

    pub fn detailed<I: Into<ParserErrorKind>, R: Into<TextRange>, S: Into<String>>(
        err: I,
        range: R,
        detail: S,
    ) -> ParserError {
        ParserError(err.into(), range.into(), Some(detail.into()))
    }

    pub fn kind(&self) -> &ParserErrorKind {
        &self.0
    }

    pub fn range(&self) -> &TextRange {
        &self.1
    }

    pub fn detail(&self) -> Option<&str> {
        match self.2 {
            Some(ref s) => Some(s.as_str()),
            None => None,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref detail) = self.2 {
            write!(f, "{}: {}", self.1, detail)
        } else {
            write!(f, "{}: {:?}", self.1, self.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    InvalidUtf8,
    InvalidEscape,
    UnexpectedEof,
    UnexpectedToken,
    IncompleteDeclaration,
    IoError(io::ErrorKind),
    UnexpectedAtom(String),
    UnexpectedCharacter(char),
    UnknownInstruction(String),
    UndeclaredIdentifier(String),
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
