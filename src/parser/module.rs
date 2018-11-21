use std::collections::VecDeque;

use crate::{
    builder::ModuleBuilder,
    module::{Export, Import, MemberDesc},
    parser::{
        func, func_type,
        sexpr::{SExpr, SVal},
        utils, ParserError, ParserErrorKind,
    },
    script::ScriptCommand,
};

pub fn parse_module(mut body: VecDeque<SExpr>) -> Result<ScriptCommand, ParserError> {
    let mut module = ModuleBuilder::new();

    while let Some(SExpr(SVal::Expr(mut body), _)) = body.pop_front() {
        let (kwd, range) = utils::expect_atom(&mut body, "a keyword")?;

        match kwd.as_str() {
            "import" => parse_import(body, &mut module)?,
            "export" => parse_export(body, &mut module)?,
            "func" => {
                module.add_func(func::parse_func(body)?);
            }
            x => {
                return Err(err!(
                    range,
                    ParserErrorKind::UnexpectedAtom(x.to_string()),
                    format!("Unexpected keyword: '{}'.", x)
                ));
            }
        }
    }

    Ok(ScriptCommand::Module(module.build()))
}

fn parse_export(mut body: VecDeque<SExpr>, module: &mut ModuleBuilder) -> Result<(), ParserError> {
    // Read the names
    let (name, _) = utils::expect_str(&mut body, "an export name")?;

    // Read the description
    let desc = parse_member_desc(body, module)?;

    module.exports.push(Export::new(name, desc));
    Ok(())
}

fn parse_import(mut body: VecDeque<SExpr>, module: &mut ModuleBuilder) -> Result<(), ParserError> {
    // Read the names
    let (module_name, _) = utils::expect_str(&mut body, "a module name")?;
    let (import_name, _) = utils::expect_str(&mut body, "an import name")?;

    // Read the description
    let desc = parse_member_desc(body, module)?;

    module
        .imports
        .push(Import::new(module_name, import_name, desc));
    Ok(())
}

fn parse_member_desc(
    mut body: VecDeque<SExpr>,
    module: &mut ModuleBuilder,
) -> Result<MemberDesc, ParserError> {
    let (mut exprs, _) = utils::expect_expr(&mut body, "a member description")?;
    let (kwd, range) = utils::expect_atom(&mut exprs, "a keyword")?;

    match kwd.as_str() {
        "func" => {
            // TODO: Identifiers
            let type_use = func_type::parse_typeuse(&mut exprs, None)?;
            let type_id = module.add_type(type_use);
            Ok(MemberDesc::Function(type_id))
        }
        x => Err(err!(
            range,
            ParserErrorKind::UnexpectedAtom(x.to_string()),
            format!("Unexpected keyword: '{}'.", x)
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{builder::ModuleBuilder, parser::utils, script::ScriptCommand};

    #[test]
    pub fn empty_module() {
        let expected_module = ModuleBuilder::new().build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            utils::single_command("(module)").unwrap()
        );
    }
}
