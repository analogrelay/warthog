use std::collections::VecDeque;

use crate::parser::{
    sexpr::{SExpr, SVal},
    symbol_table::SymbolTable,
    ParserError, ParserErrorKind, TextRange,
};

macro_rules! expect_msg {
    ($expect: expr) => {
        format!("Expected {} but found <end-of-file>", $expect)
    };
    ($expect: expr, $actual: expr) => {
        format!("Expected {} but found '{}'", $expect, $actual)
    };
}

pub fn expect_id(
    body: &mut VecDeque<SExpr>,
    symbol_table: &SymbolTable,
    expectation: &'static str,
) -> Result<usize, ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Integer(i), _)) => Ok(i as usize),
        Some(SExpr(SVal::Identifier(id), range)) => match symbol_table.get(id.as_str()) {
            Some(x) => Ok(x),
            None => Err(err!(
                range,
                ParserErrorKind::UndeclaredIdentifier(id.clone()),
                format!("Use of undeclared identifier: {}", id)
            )),
        },
        Some(SExpr(x, range)) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            expect_msg!(expectation, x),
        )),
        None => Err(err!(
            TextRange::empty(), // TODO: Figure out the start point?
            ParserErrorKind::UnexpectedEof,
            expect_msg!(expectation),
        )),
    }
}

pub fn expect_str(
    body: &mut VecDeque<SExpr>,
    expectation: &'static str,
) -> Result<String, ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Str(s), _)) => Ok(s),
        Some(SExpr(x, range)) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            expect_msg!(expectation, x),
        )),
        None => Err(err!(
            TextRange::empty(), // TODO: Figure out the start point?
            ParserErrorKind::UnexpectedEof,
            expect_msg!(expectation),
        )),
    }
}

pub fn expect_float(
    body: &mut VecDeque<SExpr>,
    expectation: &'static str,
) -> Result<f64, ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Float(i), _)) => Ok(i),
        Some(SExpr(x, range)) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            expect_msg!(expectation, x),
        )),
        None => Err(err!(
            TextRange::empty(), // TODO: Figure out the start point?
            ParserErrorKind::UnexpectedEof,
            expect_msg!(expectation),
        )),
    }
}

pub fn expect_int(
    body: &mut VecDeque<SExpr>,
    expectation: &'static str,
) -> Result<u64, ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Integer(i), _)) => Ok(i),
        Some(SExpr(x, range)) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            format!("Expected an Integer but found: {:?}", x)
        )),
        None => Err(err!(
            TextRange::empty(), // TODO: Figure out the start point?
            ParserErrorKind::UnexpectedEof,
            expect_msg!(expectation),
        )),
    }
}

pub fn try_pop_keyword_expr(
    body: &mut VecDeque<SExpr>,
    keyword: &str,
) -> Option<(VecDeque<SExpr>, TextRange)> {
    // Need to end the immutable borrows before actually popping the values
    // So we capture the match result in a boolean.
    let is_match = if let Some(SExpr(SVal::Expr(items), _)) = body.front() {
        match items.front() {
            Some(SExpr(SVal::Atom(s), _)) if s == keyword => true,
            _ => false,
        }
    } else {
        false
    };

    if is_match {
        let (mut items, range) = body.pop_front().unwrap().consume_expr().unwrap();
        debug_assert_eq!(keyword, items.pop_front().unwrap().keyword().unwrap());

        Some((items, range))
    } else {
        None
    }
}

/// Unwraps a "keyword expression" which is an s-expression with an atom at the start,
/// like `(foo 42)`
pub fn unwrap_keyword_expr(sexpr: SExpr) -> Result<Option<(SExpr, VecDeque<SExpr>)>, ParserError> {
    match sexpr {
        SExpr(SVal::Expr(mut items), range) => {
            let first = items.pop_front().ok_or_else(|| {
                err!(
                    range,
                    ParserErrorKind::UnexpectedToken,
                    "Expected s-expression to have at least one value."
                )
            })?;
            Ok(Some((first, items)))
        }
        SExpr(_, _) => Ok(None),
    }
}

pub fn expect_keyword_expr(sexpr: SExpr) -> Result<(SExpr, VecDeque<SExpr>), ParserError> {
    let range = sexpr.range().clone();
    unwrap_keyword_expr(sexpr)?.ok_or_else(|| {
        err!(
            range,
            ParserErrorKind::UnexpectedToken,
            "Expected an s-expression."
        )
    })
}

// Test helpers
#[cfg(test)]
use crate::script::ScriptCommand;

#[cfg(test)]
pub fn single_command(inp: &str) -> Result<ScriptCommand, ParserError> {
    let script = crate::parser::parse(&mut inp.as_bytes())?;
    let mut drainer = script.commands().iter();
    let first = drainer.next().cloned().unwrap();
    assert_eq!(None, drainer.next());

    Ok(first)
}
