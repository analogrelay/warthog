use std::collections::VecDeque;

use crate::{
    builder::TypeUse,
    module::ValType,
    parser::{
        sexpr::{SExpr, SVal},
        symbol_table::SymbolTable,
        utils, ParserError, ParserErrorKind,
    },
};

pub fn parse_typeuse(
    rest: &mut VecDeque<SExpr>,
    locals: Option<&mut SymbolTable>,
) -> Result<TypeUse, ParserError> {
    let id = match utils::try_pop_keyword_expr(rest, "type") {
        // TODO: Type identifiers
        Some((mut body, _)) => Some(utils::expect_int(&mut body, "a type index")?.0 as usize),
        _ => None,
    };

    let params = parse_func_type_segment(rest, "param", locals)?;
    let results = parse_func_type_segment(rest, "result", None)?;

    Ok(TypeUse {
        id,
        params,
        results,
    })
}

fn parse_func_type_segment(
    rest: &mut VecDeque<SExpr>,
    keyword: &str,
    mut locals: Option<&mut SymbolTable>,
) -> Result<Vec<ValType>, ParserError> {
    let mut list = Vec::new();
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

    Ok(list)
}
