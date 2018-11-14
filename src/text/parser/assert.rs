use std::collections::VecDeque;

use crate::text::{
    parser::{instruction, symbol_table::SymbolTable, utils},
    sexpr::SExpr,
    ParserError, ParserErrorKind, ScriptAction, ScriptCommand,
};

pub fn parse_assert_trap(
    mut body: VecDeque<SExpr>,
    start: usize,
    end: usize,
) -> Result<ScriptCommand, ParserError> {
    // Parse the action
    let action = match body.pop_front() {
        Some(expr) => parse_action(expr)?,
        None => {
            return Err(err!(
                (start, end),
                ParserErrorKind::IncompleteDeclaration,
                "Expected 'assert_return' declaration to have an action."
            ))
        }
    };

    // Parse the failure
    Ok(ScriptCommand::AssertTrap(
        action,
        utils::pop_str(&mut body)?,
    ))
}

pub fn parse_assert_return(
    mut body: VecDeque<SExpr>,
    start: usize,
    end: usize,
) -> Result<ScriptCommand, ParserError> {
    // Parse the action
    let action = match body.pop_front() {
        Some(expr) => parse_action(expr)?,
        None => {
            return Err(err!(
                (start, end),
                ParserErrorKind::IncompleteDeclaration,
                "Expected 'assert_return' declaration to have an action."
            ))
        }
    };

    // Parse the result
    if let Some(expr_body) = body.pop_front() {
        let expr = instruction::parse_expr(expr_body, SymbolTable::empty())?;
        Ok(ScriptCommand::AssertReturn(action, Some(expr)))
    } else {
        Ok(ScriptCommand::AssertReturn(action, None))
    }
}

fn parse_action(expr: SExpr) -> Result<ScriptAction, ParserError> {
    let (kwd, mut body) = utils::expect_keyword_expr(expr)?;

    match kwd.keyword().unwrap() {
        "invoke" => {
            let name = utils::pop_str(&mut body)?;
            let mut exprs = Vec::new();
            while let Some(expr) = body.pop_front() {
                exprs.push(instruction::parse_expr(expr, SymbolTable::empty())?);
            }

            Ok(ScriptAction::Invoke(name, exprs))
        }
        "get" => {
            let name = utils::pop_str(&mut body)?;
            Ok(ScriptAction::Get(name))
        }
        x => {
            return Err(err!(
                (kwd.start(), kwd.end()),
                ParserErrorKind::UnexpectedAtom(x.to_string()),
                format!("Unexpected keyword: '{}'.", x)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        module::{Expr, Instruction},
        text::{parser::utils, ScriptAction, ScriptCommand},
        Value,
    };

    #[test]
    pub fn assert_return_invoke() {
        let expected = ScriptCommand::AssertReturn(
            ScriptAction::Invoke(
                "add".to_owned(),
                vec![
                    Expr::new(vec![Instruction::Const(Value::Integer32(1))]),
                    Expr::new(vec![Instruction::Const(Value::Integer32(2))]),
                ],
            ),
            Some(Expr::new(vec![Instruction::Const(Value::Integer32(3))])),
        );
        assert_eq!(
            expected,
            utils::single_command(
                "(assert_return (invoke \"add\" (i32.const 1) (i32.const 2)) (i32.const 3))"
            ).unwrap()
        );
    }

    #[test]
    pub fn assert_return_get() {
        let expected = ScriptCommand::AssertReturn(
            ScriptAction::Get("pi".to_owned()),
            Some(Expr::new(vec![Instruction::Const(Value::Integer32(3))])),
        );
        assert_eq!(
            expected,
            utils::single_command("(assert_return (get \"pi\") (i32.const 3))").unwrap()
        );
    }
}
