use std::collections::VecDeque;

use crate::{
    builder::FuncBuilder,
    module::ValType,
    parser::{
        instruction,
        sexpr::{SExpr, SVal},
        symbol_table::SymbolTable,
        utils, ParserError, ParserErrorKind,
    },
};

pub fn parse_func(mut body: VecDeque<SExpr>) -> Result<FuncBuilder, ParserError> {
    let mut func = FuncBuilder::new();
    let mut locals = SymbolTable::new();

    // TODO: Support identifiers
    if try_parse_import(&mut body, &mut func)? {
        parse_typeuse(&mut body, &mut func, &mut locals)?;
    } else {
        parse_export(&mut body, &mut func)?;
        parse_typeuse(&mut body, &mut func, &mut locals)?;
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
        let module = utils::expect_str(&mut body, "a module name")?;
        let func_name = utils::expect_str(&mut body, "a function name")?;
        func.import = Some((module, func_name));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_export(rest: &mut VecDeque<SExpr>, func: &mut FuncBuilder) -> Result<(), ParserError> {
    if let Some((mut body, _)) = utils::try_pop_keyword_expr(rest, "export") {
        // Read the name
        func.export = Some(utils::expect_str(&mut body, "an export name")?);

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

fn parse_typeuse(
    rest: &mut VecDeque<SExpr>,
    func: &mut FuncBuilder,
    locals: &mut SymbolTable,
) -> Result<(), ParserError> {
    if let Some((mut body, range)) = utils::try_pop_keyword_expr(rest, "type") {
        // Read the ID
        match body.pop_front() {
            Some(ex) => {
                let (id, _) = ex.consume_int()?;
                func.type_id = Some(id as usize);
            }
            None => {
                return Err(err!(
                    range,
                    ParserErrorKind::UnexpectedToken,
                    "'type' block is empty, expected a type index or identifier!"
                ))
            }
        }
    }

    parse_func_type_segment(rest, "param", &mut func.params, Some(locals))?;
    parse_func_type_segment(rest, "result", &mut func.results, None)?;

    Ok(())
}

fn parse_func_type_segment(
    rest: &mut VecDeque<SExpr>,
    keyword: &str,
    list: &mut Vec<ValType>,
    mut locals: Option<&mut SymbolTable>,
) -> Result<(), ParserError> {
    // Read while we have the specified keyword
    while let Some((mut body, _)) = utils::try_pop_keyword_expr(rest, keyword) {
        for expr in body.drain(..) {
            match expr {
                SExpr(SVal::Atom(atom), range) => {
                    let valtyp = match atom.as_str() {
                        "i64" => ValType::Integer64,
                        "i32" => ValType::Integer32,
                        "f64" => ValType::Float64,
                        "f32" => ValType::Float32,
                        x => {
                            let msg = format!("'{}' is not a valid value type.", x);
                            return Err(err!(
                                range,
                                ParserErrorKind::UnexpectedAtom(x.to_string()),
                                msg
                            ));
                        }
                    };
                    list.push(valtyp)
                }
                SExpr(SVal::Identifier(id), range) => {
                    // Assign the next identifier
                    match locals.as_mut() {
                        Some(l) => {
                            l.assign(id);
                        }
                        None => {
                            return Err(err!(
                                range,
                                ParserErrorKind::UnexpectedToken,
                                format!("Identifiers are not permitted in this definition.")
                            ))
                        }
                    }
                }
                x => {
                    return Err(err!(
                        x.range(),
                        ParserErrorKind::UnexpectedToken,
                        format!("Expected an Atom or Identifier, but found: '{:?}'", x)
                    ));
                }
            }
        }
    }

    Ok(())
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
