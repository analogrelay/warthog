mod assert;
mod command;
mod func;
mod instruction;
mod module;
mod symbol_table;
mod utils;

use std::io;

use crate::text::{sexpr::SExprParser, ParserError, ScriptCommand};

pub fn parse<R: io::Read>(reader: &mut R) -> Result<Vec<ScriptCommand>, ParserError> {
    let mut parser = SExprParser::new(reader);
    let mut commands = Vec::new();
    while let Some(sexpr) = parser.parse()? {
        commands.push(command::parse_command(sexpr)?);
    }
    Ok(commands)
}
