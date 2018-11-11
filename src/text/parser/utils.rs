use std::collections::VecDeque;

use crate::text::{
    sexpr::{SExpr, SVal},
    ParserError, ParserErrorKind,
};

pub fn pop_int(body: &mut VecDeque<SExpr>) -> Result<i64, ParserError> {
    match body.pop_front() {
        Some(SExpr(SVal::Integer(i), _, _)) => Ok(i),
        Some(SExpr(x, start, end)) => Err(err!(
            (start, end),
            ParserErrorKind::UnexpectedToken,
            format!("Expected an Integer but found: {:?}", x)
        )),
        None => Err(err!(
            0, // TODO: Figure out the start point?
            ParserErrorKind::UnexpectedEof,
            format!("Unexpected end-of-file when attempting to read an integer")
        )),
    }
}

pub fn try_pop_keyword_expr(
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
pub fn unwrap_keyword_expr(sexpr: SExpr) -> Result<Option<(SExpr, VecDeque<SExpr>)>, ParserError> {
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
        SExpr(_, _, _) => Ok(None),
    }
}

pub fn expect_keyword_expr(sexpr: SExpr) -> Result<(SExpr, VecDeque<SExpr>), ParserError> {
    let (start, end) = (sexpr.start(), sexpr.end());

    unwrap_keyword_expr(sexpr)?.ok_or_else(|| {
        err!(
            (start, end),
            ParserErrorKind::UnexpectedToken,
            "Expected an s-expression."
        )
    })
}

// Test helpers
#[cfg(test)]
use crate::text::ScriptCommand;

#[cfg(test)]
pub fn single_command(inp: &str) -> Result<ScriptCommand, ParserError> {
    let mut parser = crate::text::parse(&mut inp.as_bytes())?;
    let mut drainer = parser.drain(..);
    let first = drainer.next().unwrap();
    assert_eq!(None, drainer.next());

    Ok(first)
}