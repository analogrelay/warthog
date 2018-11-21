use crate::{
    parser::{assert, module, sexpr::SExpr, utils, ParserError, ParserErrorKind},
    script::ScriptCommand,
};

pub fn parse_command(sexpr: SExpr) -> Result<ScriptCommand, ParserError> {
    let (mut body, range) = utils::extract_body(sexpr)?;
    let (kwd, kwd_range) = utils::expect_atom(&mut body, "a keyword")?;

    match kwd.as_str() {
        "module" => Ok(module::parse_module(body)?),
        "assert_return" => Ok(assert::parse_assert_return(body, range)?),
        "assert_trap" => Ok(assert::parse_assert_trap(body, range)?),
        x => Err(err!(
            kwd_range,
            ParserErrorKind::UnexpectedAtom(x.to_string()),
            format!("Unexpected keyword: '{}'.", x)
        )),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{utils, ParserError, ParserErrorKind, TextLocation, TextRange};

    #[test]
    pub fn error_empty_sexpr() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                TextRange(TextLocation::start(), TextLocation::start()),
                "Expected a keyword but found <end-of-file>."
            ),
            utils::single_command("()").unwrap_err()
        );
    }

    #[test]
    pub fn error_bare_int() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedToken,
                TextRange(TextLocation::start(), TextLocation::start().advance(2)),
                "Expected an expression but found '42'."
            ),
            utils::single_command("42").unwrap_err()
        );
    }
}
