use std::{collections::VecDeque, io};

use crate::{
    builder::{FuncBuilder, ModuleBuilder},
    module::ValType,
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
    let (kwd, body) = expect_keyword_expr(sexpr)?;

    match kwd.keyword().unwrap() {
        "module" => Ok(parse_module(body)?),
        x => Err(err!(
            (kwd.start(), kwd.end()),
            ParserErrorKind::UnexpectedAtom(x.to_string()),
            format!("Unexpected keyword: '{}'.", x)
        )),
    }
}

fn parse_module(mut body: VecDeque<SExpr>) -> Result<ScriptCommand, ParserError> {
    let mut module = ModuleBuilder::new();

    let mut imports_allowed = true;

    while let Some(nexpr) = body.pop_front() {
        let (start, end) = (nexpr.start(), nexpr.end());
        let (kwd, body) = expect_keyword_expr(nexpr)?;

        match kwd.keyword().unwrap() {
            "func" => module.add_func(parse_func(body)?),
            x => {
                return Err(err!(
                    (kwd.start(), kwd.end()),
                    ParserErrorKind::UnexpectedAtom(x.to_string()),
                    format!("Unexpected keyword: '{}'.", x)
                ))
            }
        }
    }

    Ok(ScriptCommand::Module(module.build()))
}

fn parse_func(mut body: VecDeque<SExpr>) -> Result<FuncBuilder, ParserError> {
    let mut func = FuncBuilder::new();

    // TODO: Support identifiers

    parse_export(&mut body, &mut func)?;
    parse_typeuse(&mut body, &mut func)?;

    Ok(func)
}

fn parse_typeuse(rest: &mut VecDeque<SExpr>, func: &mut FuncBuilder) -> Result<(), ParserError> {
    if let Some((mut body, start, end)) = pop_keyword_expr(rest, "type") {
        // Read the ID
        match body.pop_front() {
            Some(ex) => {
                let (id, _, _) = ex.consume_int()?;
                func.type_id = Some(id as usize);
            }
            None => {
                return Err(err!(
                    (start, end),
                    ParserErrorKind::UnexpectedToken,
                    "'type' block is empty, expected a type index or identifier!"
                ))
            }
        }
    }

    parse_func_type_segment(rest, "param", &mut func.params);
    parse_func_type_segment(rest, "result", &mut func.results);

    Ok(())
}

fn parse_func_type_segment(
    rest: &mut VecDeque<SExpr>,
    keyword: &str,
    list: &mut Vec<ValType>,
) -> Result<(), ParserError> {
    // Read while we have the specified keyword
    while let Some((mut body, start, end)) = pop_keyword_expr(rest, keyword) {
        // TODO: Support identifiers
        for expr in body {
            let (typ, start, end) = expr.consume_atom()?;
            let valtyp = match typ.as_str() {
                "i64" => ValType::Integer64,
                "i32" => ValType::Integer32,
                "f64" => ValType::Float64,
                "f32" => ValType::Float32,
                _ => {
                    let msg = format!("'{}' is not a valid value type.", typ);
                    return Err(err!(
                        (start, end),
                        ParserErrorKind::UnexpectedAtom(typ),
                        msg
                    ))
                }
            };
            list.push(valtyp);
        }
    }
    
    Ok(())
}

fn parse_export(rest: &mut VecDeque<SExpr>, func: &mut FuncBuilder) -> Result<(), ParserError> {
    if let Some((mut body, start, end)) = pop_keyword_expr(rest, "export") {
        // Read the name
        match body.pop_front() {
            Some(ex) => {
                let (name, _, _) = ex.consume_str()?;
                func.export = Some(name);
            }
            None => {
                return Err(err!(
                    (start, end),
                    ParserErrorKind::UnexpectedToken,
                    "'export' block is empty, expected a name!"
                ))
            }
        }

        match body.pop_front() {
            None => Ok(()),
            Some(tok) => Err(err!(
                (tok.start(), tok.end()),
                ParserErrorKind::UnexpectedToken,
                format!("'export' block does not expect a {:?}", tok)
            )),
        }
    } else {
        // Nothing to do
        Ok(())
    }
}

fn pop_keyword_expr(
    body: &mut VecDeque<SExpr>,
    keyword: &str,
) -> Option<(VecDeque<SExpr>, usize, usize)> {
    // Need to end the immutable borrows before actually popping the values
    // So we capture the match result in a boolean.
    let is_match = if let Some(SExpr(SVal::Expr(items), _, _)) = body.front() {
        match items.front() {
            Some(SExpr(SVal::Atom(s), _, _)) if s == keyword => true,
            _ => false,
        }
    } else {
        false
    };

    if is_match {
        let (mut items, start, end) = body.pop_front().unwrap().consume_expr().unwrap();
        debug_assert_eq!(keyword, items.pop_front().unwrap().keyword().unwrap());

        Some((items, start, end))
    } else {
        None
    }
}

/// Unwraps a "keyword expression" which is an s-expression with an atom at the start,
/// like `(foo 42)`
fn unwrap_keyword_expr(sexpr: SExpr) -> Result<Option<(SExpr, VecDeque<SExpr>)>, ParserError> {
    match sexpr {
        SExpr(SVal::Expr(mut items), start, end) => {
            let first = items.pop_front().ok_or_else(|| {
                err!(
                    (start, end),
                    ParserErrorKind::UnexpectedToken,
                    "Expected s-expression to have at least one value."
                )
            })?;
            Ok(Some((first, items)))
        }
        SExpr(_, start, end) => Ok(None),
    }
}

fn expect_keyword_expr(sexpr: SExpr) -> Result<(SExpr, VecDeque<SExpr>), ParserError> {
    let (start, end) = (sexpr.start(), sexpr.end());

    unwrap_keyword_expr(sexpr)?.ok_or_else(|| {
        err!(
            (start, end),
            ParserErrorKind::UnexpectedToken,
            "Expected an s-expression."
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        builder::{FuncBuilder, ModuleBuilder},
        module::{FuncType, Instruction, ValType},
        Value,
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
                    .param(ValType::Integer32)
                    .param(ValType::Integer32)
                    .result(ValType::Integer32)
                    .body(vec![
                        Instruction::Const(Value::Integer32(1400)),
                        Instruction::Const(Value::Integer32(13)),
                        Instruction::Call(0),
                    ]),
            ).build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            single_command("(module (func (export \"add\") (param i32) (param i32) (result i32) i32.const 1400 i32.const 13 call 0))").unwrap());
    }

    fn single_command(inp: &str) -> Result<ScriptCommand, ParserError> {
        let mut parser = parse(&mut inp.as_bytes())?;
        let mut drainer = parser.drain(..);
        let first = drainer.next().unwrap();
        assert_eq!(None, drainer.next());

        Ok(first)
    }
}
