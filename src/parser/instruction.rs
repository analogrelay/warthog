use std::collections::VecDeque;

use crate::{
    module::{Expr, Instruction, Signedness, ValType},
    parser::{
        sexpr::{SExpr, SVal},
        symbol_table::SymbolTable,
        utils, ParserError, ParserErrorKind, TextRange,
    },
    Value,
};

pub fn parse_expr(token: SExpr, locals: &SymbolTable) -> Result<Expr, ParserError> {
    match token {
        SExpr(SVal::Expr(mut expr_body), _) => {
            let mut instrs = Vec::new();
            parse_instructions(&mut expr_body, &mut instrs, locals)?;
            Ok(Expr::new(instrs))
        }
        SExpr(x, range) => Err(err!(
            range,
            ParserErrorKind::UnexpectedToken,
            format!("Expected an Expr but found: {:?}", x)
        )),
    }
}

pub fn parse_instructions(
    rest: &mut VecDeque<SExpr>,
    list: &mut Vec<Instruction>,
    locals: &SymbolTable,
) -> Result<(), ParserError> {
    loop {
        match rest.pop_front() {
            Some(SExpr(SVal::Expr(body), _)) => unfold_instructions(body, list, locals)?,
            Some(SExpr(SVal::Atom(inst), range)) => {
                list.push(parse_instruction(inst, range, rest, locals)?)
            }
            Some(SExpr(val, range)) => {
                return Err(err!(
                    range,
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
        Some(SExpr(SVal::Atom(inst), range)) => parse_instruction(inst, range, &mut body, locals)?,
        Some(SExpr(val, range)) => {
            return Err(err!(
                range,
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

fn parse_instruction(
    name: String,
    range: TextRange,
    rest: &mut VecDeque<SExpr>,
    locals: &SymbolTable,
) -> Result<Instruction, ParserError> {
    // Check if this is a numeric instruction
    if let Some(idx) = name.find('.') {
        match name.split_at(idx) {
            ("i32", x) => parse_numeric_instruction(ValType::Integer32, &x[1..], rest),
            ("i64", x) => parse_numeric_instruction(ValType::Integer64, &x[1..], rest),
            ("f32", x) => parse_numeric_instruction(ValType::Float32, &x[1..], rest),
            ("f64", x) => parse_numeric_instruction(ValType::Float64, &x[1..], rest),
            (_, _) => Err(err!(
                range,
                ParserErrorKind::UnknownInstruction(name.to_owned()),
                format!("Unknown instruction: {}", name)
            )),
        }
    } else {
        match name.as_str() {
            "call" => Ok(Instruction::Call(
                utils::expect_int(rest, "a function index")? as usize,
            )),
            "get_local" => Ok(Instruction::GetLocal(
                utils::expect_id(rest, locals, "a local name")? as usize,
            )),
            x => panic!("Instruction not yet implemented: {}.", x),
        }
    }
}

fn parse_numeric_instruction(
    valtyp: ValType,
    suffix: &str,
    rest: &mut VecDeque<SExpr>,
) -> Result<Instruction, ParserError> {
    let is_int = match valtyp {
        ValType::Integer32 | ValType::Integer64 => true,
        _ => false,
    };

    match (valtyp, is_int, suffix) {
        (_, _, "const") => {
            let val = match valtyp {
                ValType::Nil => unreachable!(),
                ValType::Integer32 => {
                    Value::Integer32(utils::expect_int(rest, "an integer")? as u32)
                }
                ValType::Integer64 => Value::Integer64(utils::expect_int(rest, "an integer")?),
                ValType::Float32 => Value::Float32(utils::expect_float(rest, "a float")? as f32),
                ValType::Float64 => Value::Float64(utils::expect_float(rest, "a float")?),
            };
            Ok(Instruction::Const(val))
        }

        (_, true, "clz") => Ok(Instruction::Clz(valtyp)),
        (_, true, "ctz") => Ok(Instruction::Ctz(valtyp)),
        (_, true, "popcnt") => Ok(Instruction::Popcnt(valtyp)),
        (_, _, "add") => Ok(Instruction::Add(valtyp)),
        (_, _, "sub") => Ok(Instruction::Sub(valtyp)),
        (_, _, "mul") => Ok(Instruction::Mul(valtyp)),
        (_, true, "div_u") => Ok(Instruction::Div(valtyp, Signedness::Unsigned)),
        (_, true, "div_s") => Ok(Instruction::Div(valtyp, Signedness::Signed)),
        (_, true, "rem_u") => Ok(Instruction::Rem(valtyp, Signedness::Unsigned)),
        (_, true, "rem_s") => Ok(Instruction::Rem(valtyp, Signedness::Signed)),
        (_, true, "and") => Ok(Instruction::And(valtyp)),
        (_, true, "or") => Ok(Instruction::Or(valtyp)),
        (_, true, "xor") => Ok(Instruction::Xor(valtyp)),
        (_, true, "shl") => Ok(Instruction::Shl(valtyp)),
        (_, true, "shr_u") => Ok(Instruction::Shr(valtyp, Signedness::Unsigned)),
        (_, true, "shr_s") => Ok(Instruction::Shr(valtyp, Signedness::Signed)),
        (_, true, "rotl") => Ok(Instruction::Rotl(valtyp)),
        (_, true, "rotr") => Ok(Instruction::Rotr(valtyp)),

        (_, false, "abs") => Ok(Instruction::Abs(valtyp)),
        (_, false, "neg") => Ok(Instruction::Neg(valtyp)),
        (_, false, "ceil") => Ok(Instruction::Ceil(valtyp)),
        (_, false, "floor") => Ok(Instruction::Floor(valtyp)),
        (_, false, "trunc") => Ok(Instruction::FTrunc(valtyp)),
        (_, false, "nearest") => Ok(Instruction::Nearest(valtyp)),
        (_, false, "sqrt") => Ok(Instruction::Sqrt(valtyp)),
        (_, false, "div") => Ok(Instruction::FDiv(valtyp)),
        (_, false, "min") => Ok(Instruction::Min(valtyp)),
        (_, false, "max") => Ok(Instruction::Max(valtyp)),
        (_, false, "copysign") => Ok(Instruction::Copysign(valtyp)),

        (_, true, "eqz") => Ok(Instruction::Eqz(valtyp)),
        (_, _, "eq") => Ok(Instruction::Eq(valtyp)),
        (_, _, "ne") => Ok(Instruction::Ne(valtyp)),
        (_, true, "lt_s") => Ok(Instruction::Lt(valtyp, Signedness::Signed)),
        (_, true, "lt_u") => Ok(Instruction::Lt(valtyp, Signedness::Unsigned)),
        (_, true, "gt_s") => Ok(Instruction::Gt(valtyp, Signedness::Signed)),
        (_, true, "gt_u") => Ok(Instruction::Gt(valtyp, Signedness::Unsigned)),
        (_, true, "le_s") => Ok(Instruction::Le(valtyp, Signedness::Signed)),
        (_, true, "le_u") => Ok(Instruction::Le(valtyp, Signedness::Unsigned)),
        (_, true, "ge_s") => Ok(Instruction::Ge(valtyp, Signedness::Signed)),
        (_, true, "ge_u") => Ok(Instruction::Ge(valtyp, Signedness::Unsigned)),
        (_, false, "lt") => Ok(Instruction::FLt(valtyp)),
        (_, false, "gt") => Ok(Instruction::FGt(valtyp)),
        (_, false, "le") => Ok(Instruction::FLe(valtyp)),
        (_, false, "ge") => Ok(Instruction::FGe(valtyp)),

        (ValType::Integer32, _, "wrap/i64") => Ok(Instruction::Wrap),
        (x, true, "trunc_s/f32") => Ok(Instruction::Trunc(x, Signedness::Signed, ValType::Float32)),
        (x, true, "trunc_u/f32") => Ok(Instruction::Trunc(
            x,
            Signedness::Unsigned,
            ValType::Float32,
        )),
        (x, true, "trunc_s/f64") => Ok(Instruction::Trunc(x, Signedness::Signed, ValType::Float64)),
        (x, true, "trunc_u/f64") => Ok(Instruction::Trunc(
            x,
            Signedness::Unsigned,
            ValType::Float64,
        )),
        (ValType::Integer64, _, "extend_s/i32") => Ok(Instruction::Extend(Signedness::Signed)),
        (ValType::Integer64, _, "extend_u/i32") => Ok(Instruction::Extend(Signedness::Unsigned)),
        (x, false, "convert_s/i32") => Ok(Instruction::Convert(
            x,
            Signedness::Signed,
            ValType::Integer32,
        )),
        (x, false, "convert_u/i32") => Ok(Instruction::Convert(
            x,
            Signedness::Unsigned,
            ValType::Integer32,
        )),
        (x, false, "convert_s/i64") => Ok(Instruction::Convert(
            x,
            Signedness::Signed,
            ValType::Integer64,
        )),
        (x, false, "convert_u/i64") => Ok(Instruction::Convert(
            x,
            Signedness::Unsigned,
            ValType::Integer64,
        )),
        (ValType::Float32, _, "demote/f64") => Ok(Instruction::Demote),
        (ValType::Float64, _, "promote/f32") => Ok(Instruction::Promote),
        (ValType::Integer32, _, "reinterpret/f32") => {
            Ok(Instruction::Reinterpret(ValType::Integer32))
        }
        (ValType::Integer64, _, "reinterpret/f64") => {
            Ok(Instruction::Reinterpret(ValType::Integer64))
        }
        (ValType::Float32, _, "reinterpret/i32") => Ok(Instruction::Reinterpret(ValType::Float32)),
        (ValType::Float64, _, "reinterpret/i64") => Ok(Instruction::Reinterpret(ValType::Float64)),
        (_, _, x) => panic!("Instruction not yet implemented: '{}.{}'.", valtyp, x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        module::{Instruction, Signedness, ValType},
        parser::sexpr::SExprParser,
    };

    macro_rules! test_inst {
        ($name: ident, $s: expr, $($inst: ident),+) => {
            #[test]
            pub fn $name() {
                assert_eq!(vec![$(Instruction::$inst),+], parse_instrs($s));
            }
        };
        ($name: ident, $s: expr, $($inst: ident($($e: expr),+)),+) => {
            #[test]
            pub fn $name() {
                assert_eq!(vec![$(Instruction::$inst($($e),+)),+], parse_instrs($s));
            }
        };
        ($name: ident, $p: expr, $s: expr, $($inst: ident),+) => {
            #[test]
            pub fn $name() {
                assert_eq!(vec![$(Instruction::$inst),+], parse_instrs(concat!($p, $s)));
            }
        };
        ($name: ident, $p: expr, $s: expr, $($inst: ident($($e: expr),+)),+) => {
            #[test]
            pub fn $name() {
                assert_eq!(vec![$(Instruction::$inst($($e),+)),+], parse_instrs(concat!($p, $s)));
            }
        };
    }

    macro_rules! int_insts {
        ($name: ident, $p: expr, $valtyp: ident) => {
            mod $name {
                use super::*;
                test_inst!(constant, $p, ".const 42", Const(Value::$valtyp(42)));
                test_inst!(clz, $p, ".clz", Clz(ValType::$valtyp));
                test_inst!(ctz, $p, ".ctz", Ctz(ValType::$valtyp));
                test_inst!(popcnt, $p, ".popcnt", Popcnt(ValType::$valtyp));
                test_inst!(add, $p, ".add", Add(ValType::$valtyp));
                test_inst!(sub, $p, ".sub", Sub(ValType::$valtyp));
                test_inst!(mul, $p, ".mul", Mul(ValType::$valtyp));
                test_inst!(
                    div_u,
                    $p,
                    ".div_u",
                    Div(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(
                    div_s,
                    $p,
                    ".div_s",
                    Div(ValType::$valtyp, Signedness::Signed)
                );
                test_inst!(
                    rem_u,
                    $p,
                    ".rem_u",
                    Rem(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(
                    rem_s,
                    $p,
                    ".rem_s",
                    Rem(ValType::$valtyp, Signedness::Signed)
                );
                test_inst!(and, $p, ".and", And(ValType::$valtyp));
                test_inst!(or, $p, ".or", Or(ValType::$valtyp));
                test_inst!(xor, $p, ".xor", Xor(ValType::$valtyp));
                test_inst!(shl, $p, ".shl", Shl(ValType::$valtyp));
                test_inst!(
                    shr_u,
                    $p,
                    ".shr_u",
                    Shr(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(
                    shr_s,
                    $p,
                    ".shr_s",
                    Shr(ValType::$valtyp, Signedness::Signed)
                );
                test_inst!(rotl, $p, ".rotl", Rotl(ValType::$valtyp));
                test_inst!(rotr, $p, ".rotr", Rotr(ValType::$valtyp));
                test_inst!(eq, $p, ".eq", Eq(ValType::$valtyp));
                test_inst!(ne, $p, ".ne", Ne(ValType::$valtyp));
                test_inst!(lt_s, $p, ".lt_s", Lt(ValType::$valtyp, Signedness::Signed));
                test_inst!(
                    lt_u,
                    $p,
                    ".lt_u",
                    Lt(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(gt_s, $p, ".gt_s", Gt(ValType::$valtyp, Signedness::Signed));
                test_inst!(
                    gt_u,
                    $p,
                    ".gt_u",
                    Gt(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(le_s, $p, ".le_s", Le(ValType::$valtyp, Signedness::Signed));
                test_inst!(
                    le_u,
                    $p,
                    ".le_u",
                    Le(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(ge_s, $p, ".ge_s", Ge(ValType::$valtyp, Signedness::Signed));
                test_inst!(
                    ge_u,
                    $p,
                    ".ge_u",
                    Ge(ValType::$valtyp, Signedness::Unsigned)
                );
                test_inst!(
                    trunc_s_f32,
                    $p,
                    ".trunc_s/f32",
                    Trunc(ValType::$valtyp, Signedness::Signed, ValType::Float32)
                );
                test_inst!(
                    trunc_u_f32,
                    $p,
                    ".trunc_u/f32",
                    Trunc(ValType::$valtyp, Signedness::Unsigned, ValType::Float32)
                );
                test_inst!(
                    trunc_s_f64,
                    $p,
                    ".trunc_s/f64",
                    Trunc(ValType::$valtyp, Signedness::Signed, ValType::Float64)
                );
                test_inst!(
                    trunc_u_f64,
                    $p,
                    ".trunc_u/f64",
                    Trunc(ValType::$valtyp, Signedness::Unsigned, ValType::Float64)
                );
            }
        };
    }

    macro_rules! float_insts {
        ($name: ident, $p: expr, $valtyp: ident) => {
            mod $name {
                use super::*;
                test_inst!(constant, $p, ".const 42.0", Const(Value::$valtyp(42.0)));
                test_inst!(abs, $p, ".abs", Abs(ValType::$valtyp));
                test_inst!(neg, $p, ".neg", Neg(ValType::$valtyp));
                test_inst!(ceil, $p, ".ceil", Ceil(ValType::$valtyp));
                test_inst!(floor, $p, ".floor", Floor(ValType::$valtyp));
                test_inst!(trunc, $p, ".trunc", FTrunc(ValType::$valtyp));
                test_inst!(nearest, $p, ".nearest", Nearest(ValType::$valtyp));
                test_inst!(sqrt, $p, ".sqrt", Sqrt(ValType::$valtyp));
                test_inst!(add, $p, ".add", Add(ValType::$valtyp));
                test_inst!(sub, $p, ".sub", Sub(ValType::$valtyp));
                test_inst!(mul, $p, ".mul", Mul(ValType::$valtyp));
                test_inst!(div, $p, ".div", FDiv(ValType::$valtyp));
                test_inst!(min, $p, ".min", Min(ValType::$valtyp));
                test_inst!(max, $p, ".max", Max(ValType::$valtyp));
                test_inst!(copysign, $p, ".copysign", Copysign(ValType::$valtyp));
                test_inst!(eq, $p, ".eq", Eq(ValType::$valtyp));
                test_inst!(ne, $p, ".ne", Ne(ValType::$valtyp));
                test_inst!(lt, $p, ".lt", FLt(ValType::$valtyp));
                test_inst!(gt, $p, ".gt", FGt(ValType::$valtyp));
                test_inst!(le, $p, ".le", FLe(ValType::$valtyp));
                test_inst!(ge, $p, ".ge", FGe(ValType::$valtyp));
                test_inst!(
                    convert_s_i32,
                    $p,
                    ".convert_s/i32",
                    Convert(ValType::$valtyp, Signedness::Signed, ValType::Integer32)
                );
                test_inst!(
                    convert_u_i32,
                    $p,
                    ".convert_u/i32",
                    Convert(ValType::$valtyp, Signedness::Unsigned, ValType::Integer32)
                );
                test_inst!(
                    convert_s_i64,
                    $p,
                    ".convert_s/i64",
                    Convert(ValType::$valtyp, Signedness::Signed, ValType::Integer64)
                );
                test_inst!(
                    convert_u_i64,
                    $p,
                    ".convert_u/i64",
                    Convert(ValType::$valtyp, Signedness::Unsigned, ValType::Integer64)
                );
            }
        };
    }

    int_insts!(int32, "i32", Integer32);
    int_insts!(int64, "i64", Integer64);
    float_insts!(float32, "f32", Float32);
    float_insts!(float64, "f64", Float64);

    test_inst!(wrap, "i32.wrap/i64", Wrap);
    test_inst!(extend_s, "i64.extend_s/i32", Extend(Signedness::Signed));
    test_inst!(extend_u, "i64.extend_u/i32", Extend(Signedness::Unsigned));
    test_inst!(demote, "f32.demote/f64", Demote);
    test_inst!(promote, "f64.promote/f32", Promote);
    test_inst!(
        i32_reinterpet,
        "i32.reinterpret/f32",
        Reinterpret(ValType::Integer32)
    );
    test_inst!(
        i64_reinterpet,
        "i64.reinterpret/f64",
        Reinterpret(ValType::Integer64)
    );
    test_inst!(
        f32_reinterpet,
        "f32.reinterpret/i32",
        Reinterpret(ValType::Float32)
    );
    test_inst!(
        f64_reinterpet,
        "f64.reinterpret/i64",
        Reinterpret(ValType::Float64)
    );

    test_inst!(call, "call 42", Call(42));
    test_inst!(get_local, "get_local 42", GetLocal(42));

    test_inst!(named_get_local, "get_local $0", GetLocal(0));

    // Folded instructions
    test_inst!(
        folded_call,
        "(call 42 (i32.const 1) (i32.const 2))",
        Const(Value::Integer32(1)),
        Const(Value::Integer32(2)),
        Call(42)
    );

    test_inst!(
        complex_fold,
        "(i32.mul (i32.add (get_local 0) (i32.const 2)) (i32.const 3))",
        GetLocal(0),
        Const(Value::Integer32(2)),
        Add(ValType::Integer32),
        Const(Value::Integer32(3)),
        Mul(ValType::Integer32)
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
