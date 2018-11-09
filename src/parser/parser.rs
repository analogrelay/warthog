use std::{collections::VecDeque, io};

use crate::{
    builder::{FuncBuilder, ModuleBuilder},
    parser::{
        sexpr::{SExpr, SExprParser, SVal},
        ParserError, ParserErrorKind, ScriptCommand,
    },
};

pub fn parse<R: io::Read>(reader: &mut R) -> Result<Vec<ScriptCommand>, ParserError> {
    let mut parser = SExprParser::new(reader);
    let mut commands = Vec::new();
    while let Some(sexpr) = parser.parse()? {
        commands.push(parse_command(sexpr)?);
    }
    Ok(commands)
}

fn parse_command(sexpr: SExpr) -> Result<ScriptCommand, ParserError> {
    let (kwd, body) = unwrap_keyword_expr(sexpr)?;

    match kwd.keyword().unwrap() {
        "module" => Ok(parse_module(body)?),
        x => Err(err!(
            (kwd.start(), kwd.end()),
            ParserErrorKind::UnexpectedAtom(x.to_string()),
            format!("Unexpected keyword: '{}'.", x)
        )),
    }
}

fn parse_module(body: VecDeque<SExpr>) -> Result<ScriptCommand, ParserError> {
    let mut module = ModuleBuilder::new();

    Ok(ScriptCommand::Module(module.build()))
}

/// Unwraps a "keyword expression" which is an s-expression with an atom at the start,
/// like `(foo 42)`
fn unwrap_keyword_expr(sexpr: SExpr) -> Result<(SExpr, VecDeque<SExpr>), ParserError> {
    match sexpr {
        SExpr(SVal::Expr(mut items), start, end) => {
            let first = items.pop_front().ok_or_else(|| {
                err!(
                    (start, end),
                    ParserErrorKind::UnexpectedToken,
                    "Expected s-expression to have at least one value."
                )
            })?;
            Ok((first, items))
        }
        SExpr(_, start, end) => Err(err!(
            (start, end),
            ParserErrorKind::UnexpectedToken,
            "Expected an s-expression."
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        builder::{FuncBuilder, ModuleBuilder},
        module::{Instruction, ValType, Value},
    };

    #[test]
    pub fn error_empty_sexpr() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedToken,
                (0, 1),
                "Expected s-expression to have at least one value."
            ),
            single_command("()").unwrap_err()
        );
    }

    #[test]
    pub fn error_bare_int() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedToken,
                (0, 1),
                "Expected an s-expression."
            ),
            single_command("42").unwrap_err()
        );
    }

    #[test]
    pub fn empty_module() {
        let expected_module = ModuleBuilder::new().build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            single_command("(module)").unwrap()
        );
    }

    #[test]
    pub fn simple_exported_func() {
        let expected_module = ModuleBuilder::new()
            .func(
                FuncBuilder::new()
                    .export_as("add")
                    .typ(FuncType::new(vec![ValType::Integer32, ValType::Integer32], vec![ValType::Integer32]))
                    .body(vec![
                        Instruction::Const(Value::Integer32(1400)),
                        Instruction::Const(Value::Integer32(13)),
                        Instruction::Call(0),
                    ]),
            ).build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            single_command("(module (func (export \"add\") (param i32) (param i32) (result i32) i32.const 1400 i32.const 13 call 0)").unwrap());
    }

    fn single_command(inp: &str) -> Result<ScriptCommand, ParserError> {
        let mut parser = parse(&mut inp.as_bytes())?;
        let mut drainer = parser.drain(..);
        let first = drainer.next().unwrap();
        assert_eq!(None, drainer.next());

        Ok(first)
    }
}
