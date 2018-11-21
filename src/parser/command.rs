use crate::{
    parser::{assert, module, sexpr::SExpr, utils, ParserError, ParserErrorKind},
    script::ScriptCommand,
};

pub fn parse_command(sexpr: SExpr) -> Result<ScriptCommand, ParserError> {
    let (kwd, body) = utils::expect_keyword_expr(sexpr)?;

    match kwd.keyword().unwrap() {
        "module" => Ok(module::parse_module(body)?),
        "assert_return" => Ok(assert::parse_assert_return(body, kwd.range())?),
        "assert_trap" => Ok(assert::parse_assert_trap(body, kwd.range())?),
        x => Err(err!(
            kwd.range(),
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
                ParserErrorKind::UnexpectedToken,
                TextRange(TextLocation::start(), TextLocation::start().advance(2)),
                "Expected s-expression to have at least one value."
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
                "Expected an s-expression."
            ),
            utils::single_command("42").unwrap_err()
        );
    }
}
