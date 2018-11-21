macro_rules! err {
    ($p: expr, $e: expr) => {
        ParserError::new($e, $p)
    };
    ($p: expr, $e: expr,) => {
        ParserError::new($e, $p)
    };
    ($p: expr, $e: expr, $det: expr) => {
        ParserError::detailed($e, $p, $det)
    };
    ($p: expr, $e: expr, $det: expr,) => {
        ParserError::detailed($e, $p, $det)
    };
}

mod assert;
mod command;
mod func;
mod instruction;
mod module;
mod symbol_table;
mod text_location;
mod utils;

mod parser_error;
mod sexpr;

pub use self::parser_error::{ParserError, ParserErrorKind};
pub use self::text_location::{TextLocation, TextRange};

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
