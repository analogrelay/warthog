use std::collections::VecDeque;

use crate::{
    module::{Instruction, ValType},
    text::{
        parser::{symbol_table::SymbolTable, utils},
        sexpr::{SExpr, SVal},
        ParserError, ParserErrorKind,
    },
    Value,
};

pub fn parse_instructions(
    rest: &mut VecDeque<SExpr>,
    list: &mut Vec<Instruction>,
    locals: &SymbolTable,
) -> Result<(), ParserError> {
    loop {
        match rest.pop_front() {
            Some(SExpr(SVal::Expr(body), _, _)) => unfold_instructions(body, list, locals)?,
            Some(SExpr(SVal::Atom(inst), _, _)) => list.push(parse_instruction(inst, rest, locals)?),
            Some(SExpr(val, start, end)) => {
                return Err(err!(
                    (start, end),
                    ParserErrorKind::UnexpectedToken,
                    format!("Expected an instruction, but found: '{:?}'.", val),
                ))
            }
            None => return Ok(()),
        }
    }
}

fn unfold_instructions(
    mut body: VecDeque<SExpr>,
    list: &mut Vec<Instruction>,
    locals: &SymbolTable,
) -> Result<(), ParserError> {
    // Parse the current instruction and stash it away.
    let first = match body.pop_front() {
        Some(SExpr(SVal::Atom(inst), _, _)) => parse_instruction(inst, &mut body, locals)?,
        Some(SExpr(val, start, end)) => {
            return Err(err!(
                (start, end),
                ParserErrorKind::UnexpectedToken,
                format!("Expected an instruction, but found: '{:?}'.", val),
            ))
        }
        None => return Ok(()),
    };

    // Parse the rest of the instructions, then put the first item in front.
    parse_instructions(&mut body, list, locals)?;

    // Push the first instruction on the end.
    list.push(first);

    Ok(())
}

fn parse_instruction(name: String, rest: &mut VecDeque<SExpr>, locals: &SymbolTable) -> Result<Instruction, ParserError> {
    match name.as_str() {
        "i32.const" => Ok(Instruction::Const(Value::Integer32(
            utils::pop_int(rest)? as i32
        ))),
        "call" => Ok(Instruction::Call(utils::pop_int(rest)? as usize)),
        "get_local" => Ok(Instruction::GetLocal(utils::pop_id(rest, locals)? as usize)),
        "i32.mul" => Ok(Instruction::Mul(ValType::Integer32)),
        "i32.add" => Ok(Instruction::Add(ValType::Integer32)),
        x => panic!("Instruction not yet implemented: {}.", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        module::{Instruction, ValType},
        text::sexpr::SExprParser,
    };

    macro_rules! test_inst {
        ($name: ident, $s: expr, $vec:expr) => {
            #[test]
            pub fn $name() {
                assert_eq!($vec, parse_instrs($s));
            }
        };
    }

    test_inst!(
        i32_const,
        "i32.const 42",
        vec![Instruction::Const(Value::Integer32(42))]
    );
    test_inst!(call, "call 42", vec![Instruction::Call(42)]);
    test_inst!(i32_mul, "i32.mul", vec![Instruction::Mul(ValType::Integer32)]);
    test_inst!(i32_add, "i32.add", vec![Instruction::Add(ValType::Integer32)]);
    test_inst!(get_local, "get_local 42", vec![Instruction::GetLocal(42)]);
    test_inst!(named_get_local, "get_local $0", vec![Instruction::GetLocal(0)]);

    // Folded instructions
    test_inst!(
        folded_call,
        "(call 42 (i32.const 1) (i32.const 2))",
        vec![
            Instruction::Const(Value::Integer32(1)),
            Instruction::Const(Value::Integer32(2)),
            Instruction::Call(42)
        ]
    );
    test_inst!(
        complex_fold,
        "(i32.mul (i32.add (get_local 0) (i32.const 2)) (i32.const 3))",
        vec![
            Instruction::GetLocal(0),
            Instruction::Const(Value::Integer32(2)),
            Instruction::Add(ValType::Integer32),
            Instruction::Const(Value::Integer32(3)),
            Instruction::Mul(ValType::Integer32),
        ]
    );

    fn parse_instrs(inp: &str) -> Vec<Instruction> {
        let bytes = inp.as_bytes();
        let mut parser = SExprParser::new(bytes);
        let mut tokens = VecDeque::new();
        while let Some(sexpr) = parser.parse().unwrap() {
            tokens.push_back(sexpr);
        }

        // Preload some well-defined symbols into the symbol table
        let mut locals = SymbolTable::new();
        for x in 0..10 {
            locals.assign(format!("${}", x));
        }

        let mut instrs = Vec::new();
        parse_instructions(&mut tokens, &mut instrs, &locals).unwrap();
        instrs
    }
}
