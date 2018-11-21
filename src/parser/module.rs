use std::collections::VecDeque;

use crate::{
    builder::ModuleBuilder,
    parser::{func, sexpr::SExpr, utils, ParserError, ParserErrorKind},
    script::ScriptCommand,
};

pub fn parse_module(mut body: VecDeque<SExpr>) -> Result<ScriptCommand, ParserError> {
    let mut module = ModuleBuilder::new();

    while let Some(nexpr) = body.pop_front() {
        let (kwd, body) = utils::expect_keyword_expr(nexpr)?;

        match kwd.keyword().unwrap() {
            "func" => module.add_func(func::parse_func(body)?),
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

#[cfg(test)]
mod tests {
    use crate::{
        builder::ModuleBuilder,
        parser::utils, 
        script::ScriptCommand,
    };

    #[test]
    pub fn empty_module() {
        let expected_module = ModuleBuilder::new().build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            utils::single_command("(module)").unwrap()
        );
    }
}
