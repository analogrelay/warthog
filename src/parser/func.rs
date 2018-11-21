use std::collections::VecDeque;

use crate::{
    builder::FuncBuilder,
    parser::{
        func_type, instruction, sexpr::SExpr, symbol_table::SymbolTable, utils, ParserError,
        ParserErrorKind,
    },
};

pub fn parse_func(mut body: VecDeque<SExpr>) -> Result<FuncBuilder, ParserError> {
    let mut func = FuncBuilder::new();

    // TODO: Support identifiers
    if try_parse_import(&mut body, &mut func)? {
        func.type_use = func_type::parse_typeuse(&mut body, None)?;
    } else {
        let mut locals = SymbolTable::new();
        parse_export(&mut body, &mut func)?;
        func.type_use = func_type::parse_typeuse(&mut body, Some(&mut locals))?;
        instruction::parse_instructions(&mut body, &mut func.body, &locals)?;
    }

    Ok(func)
}

fn try_parse_import(
    rest: &mut VecDeque<SExpr>,
    func: &mut FuncBuilder,
) -> Result<bool, ParserError> {
    if let Some((mut body, _)) = utils::try_pop_keyword_expr(rest, "import") {
        // Read the names
        let (module, _) = utils::expect_str(&mut body, "a module name")?;
        let (func_name, _) = utils::expect_str(&mut body, "a function name")?;
        func.import = Some((module, func_name));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_export(rest: &mut VecDeque<SExpr>, func: &mut FuncBuilder) -> Result<(), ParserError> {
    if let Some((mut body, _)) = utils::try_pop_keyword_expr(rest, "export") {
        // Read the name
        func.export = Some(utils::expect_str(&mut body, "an export name")?.0);

        match body.pop_front() {
            None => Ok(()),
            Some(tok) => Err(err!(
                tok.range(),
                ParserErrorKind::UnexpectedToken,
                format!("'export' block does not expect a {:?}", tok)
            )),
        }
    } else {
        // Nothing to do
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{FuncBuilder, ModuleBuilder},
        module::{Instruction, ValType},
        parser::utils,
        script::ScriptCommand,
        Value,
    };

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
            utils::single_command("(module (func (export \"add\") (param i32) (param i32) (result i32) i32.const 1400 i32.const 13 call 0))").unwrap());
    }

    #[test]
    pub fn named_locals() {
        let expected_module = ModuleBuilder::new()
            .func(
                FuncBuilder::new()
                    .export_as("add")
                    .param(ValType::Integer32)
                    .param(ValType::Integer32)
                    .result(ValType::Integer32)
                    .body(vec![
                        Instruction::GetLocal(0),
                        Instruction::GetLocal(1),
                        Instruction::Call(0),
                    ]),
            ).build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            utils::single_command("(module (func (export \"add\") (param $x i32) (param $y i32) (result i32) get_local $x get_local $y call 0))").unwrap());
    }
}
