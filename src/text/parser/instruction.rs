use std::collections::VecDeque;

use crate::{
    module::Instruction,
    text::{sexpr::SExpr, ParserError},
};

pub fn parse_instructions(
    _rest: &mut VecDeque<SExpr>,
    _list: &mut Vec<Instruction>,
) -> Result<(), ParserError> {
    unimplemented!();
}
