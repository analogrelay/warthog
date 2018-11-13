mod command;
mod func;
mod instruction;
mod module;
mod utils;
mod symbol_table;

use std::io;

use crate::text::{
    ScriptCommand,
    sexpr::SExprParser,
    ParserError
};

pub fn parse<R: io::Read>(reader: &mut R) -> Result<Vec<ScriptCommand>, ParserError> {
    let mut parser = SExprParser::new(reader);
    let mut commands = Vec::new();
    while let Some(sexpr) = parser.parse()? {
        commands.push(command::parse_command(sexpr)?);
    }
    Ok(commands)
}