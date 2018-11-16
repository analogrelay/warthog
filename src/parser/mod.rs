macro_rules! err {
    ($p: expr, $e: expr) => {
        ParserError::new($e, crate::parser::ToRange::into_range($p))
    };
    ($p: expr, $e: expr,) => {
        ParserError::new($e, crate::parser::ToRange::into_range($p))
    };
    ($p: expr, $e: expr, $det: expr) => {
        ParserError::detailed($e, crate::parser::ToRange::into_range($p), $det)
    };
    ($p: expr, $e: expr, $det: expr,) => {
        ParserError::detailed($e, crate::parser::ToRange::into_range($p), $det)
    };
}

mod assert;
mod command;
mod func;
mod instruction;
mod module;
mod symbol_table;
mod utils;

mod parser_error;
mod sexpr;

pub use self::parser_error::{ParserError, ParserErrorKind};

use std::io;

use crate::{parser::sexpr::SExprParser, script::Script};

pub fn parse<R: io::Read>(reader: &mut R) -> Result<Script, ParserError> {
    let mut parser = SExprParser::new(reader);
    let mut commands = Vec::new();
    while let Some(sexpr) = parser.parse()? {
        commands.push(command::parse_command(sexpr)?);
    }
    Ok(Script::new(commands))
}

trait ToRange {
    fn into_range(self) -> (usize, usize);
}

impl ToRange for usize {
    fn into_range(self) -> (usize, usize) {
        (self, self)
    }
}

impl ToRange for (usize, usize) {
    fn into_range(self) -> (usize, usize) {
        self
    }
}
