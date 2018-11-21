use std::collections::VecDeque;

use crate::parser::{
    sexpr::{SExpr, SVal},
    symbol_table::SymbolTable,
    ParserError, ParserErrorKind, TextRange,
};

macro_rules! expect_msg {
    ($expect: expr) => {
        format!("Expected {} but found <end-of-file>.", $expect)
    };
    ($expect: expr, $actual: expr) => {
        format!("Expected {} but found '{}'.", $expect, $actual)
    };
}

pub fn expect_id(
    body: &mut VecDeque<SExpr>,
    symbol_table: &SymbolTable,
    expectation: &'static str,
) -> Result<(usize, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Integer(i), r)) => Ok((i as usize, r)),
        Some(SExpr(SVal::Identifier(id), range)) => match symbol_table.get(id.as_str()) {
            Some(x) => Ok((x, range)),
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

pub fn expect_atom(
    body: &mut VecDeque<SExpr>,
    expectation: &'static str,
) -> Result<(String, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Atom(s), r)) => Ok((s, r)),
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
) -> Result<(String, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Str(s), r)) => Ok((s, r)),
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
) -> Result<(f64, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Float(i), r)) => Ok((i, r)),
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

pub fn expect_expr(
    body: &mut VecDeque<SExpr>,
    expectation: &'static str,
) -> Result<(VecDeque<SExpr>, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Expr(v), r)) => Ok((v, r)),
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
) -> Result<(u64, TextRange), ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Integer(i), r)) => Ok((i, r)),
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

pub fn extract_body(expr: SExpr) -> Result<(VecDeque<SExpr>, TextRange), ParserError> {
    match expr {
        SExpr(SVal::Expr(b), r) => Ok((b, r)),
        SExpr(v, range) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            expect_msg!("an expression", v)
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
