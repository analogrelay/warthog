use crate::text::{
    parser::{assert, module, utils},
    sexpr::SExpr,
    ParserError, ParserErrorKind, ScriptCommand,
};

pub fn parse_command(sexpr: SExpr) -> Result<ScriptCommand, ParserError> {
    let (kwd, body) = utils::expect_keyword_expr(sexpr)?;

    match kwd.keyword().unwrap() {
        "module" => Ok(module::parse_module(body)?),
        "assert_return" => Ok(assert::parse_assert_return(body, kwd.start(), kwd.end())?),
        "assert_trap" => Ok(assert::parse_assert_trap(body, kwd.start(), kwd.end())?),
        x => Err(err!(
            (kwd.start(), kwd.end()),
            ParserErrorKind::UnexpectedAtom(x.to_string()),
            format!("Unexpected keyword: '{}'.", x)
        )),
    }
}

#[cfg(test)]
mod test {
    use crate::text::{parser::utils, ParserError, ParserErrorKind};

    #[test]
    pub fn error_empty_sexpr() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedToken,
                (0, 1),
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
                (0, 1),
                "Expected an s-expression."
            ),
            utils::single_command("42").unwrap_err()
        );
    }
}
