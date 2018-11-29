use crate::{
    interp::{Thread},
    module::{Instruction, Signedness},
    hosting::Host,
    Value, Trap, ValType,
};

macro_rules! binop {
    ($name: ident, $(($typ: ident, $x: ident, $y: ident) => $bl: expr),+) => {
        fn $name(thread: &mut Thread, typ: ValType) -> Result<(), Trap> {
            let c2 = thread.stack_mut().pop()?;
            let c1 = thread.stack_mut().pop()?;

            let res = match (typ, c1, c2) {
                $(
                    (ValType::$typ, Value::$typ($x), Value::$typ($y)) => {
                        $bl as Result<Value, Trap>
                    },
                    (ValType::$typ, x, y) => return Err(format!(
                        "Type mismatch. Expected arguments of type '{}' but found arguments of type '{}' and '{}'.",
                        ValType::$typ,
                        x.typ(),
                        y.typ()
                    ).into())
                ),+,
                (t, _, _) => return Err(format!(
                    concat!("'", stringify!($name), "' is not yet implemented for operands of type '{}'"),
                    t
                ).into())
            };

            res.map(|r|{ thread.push(r); () })
        }
    };
}

macro_rules! unop {
    ($name: ident, $(($typ: ident, $x: ident) => $bl: expr),+) => {
        fn $name(thread: &mut Thread, typ: ValType) -> Result<(), Trap> {
            let x = thread.stack_mut().pop()?;

            let res = match (typ, x) {
                $(
                    (ValType::$typ, Value::$typ($x)) => {
                        $bl as Result<Value, Trap>
                    }
                ),+
                (t, x) => return Err(format!(
                    "Type mismatch. Expected argument of type '{}' but found argument of type '{}'.",
                    t,
                    x.typ()
                ).into()),
            };

            match res {
                Ok(r) => {
                    thread.push(r);
                    Ok(())
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
    };
}


pub fn execute(thread: &mut Thread, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
    match inst {
        Instruction::Const(val) => thread.push(val.clone()),
        Instruction::Call(func_idx) => {
            let module_addr = thread.stack().current().frame().module();
            let func = host.resolve_func(module_addr, func_idx);
            let values = thread.invoke(host, func)?;

            // Push the result values on to the stack
            for value in values {
                thread.push(value)
            }
        }
        Instruction::GetLocal(local_idx) => {
            let val = match thread.stack().current().local(local_idx) {
                Some(l) => l,
                None => return Err(format!("No such local: {}", local_idx).into()),
            };
            thread.push(val);
        }
        Instruction::Add(t) => add(thread, t)?,
        Instruction::Sub(t) => sub(thread, t)?,
        Instruction::Mul(t) => mul(thread, t)?,
        Instruction::Div(t, Signedness::Signed) => div_s(thread, t)?,
        Instruction::Div(t, Signedness::Unsigned) => div_u(thread, t)?,
        Instruction::FDiv(t) => fdiv(thread, t)?,
        Instruction::Rem(t, Signedness::Signed) => rem_s(thread, t)?,
        Instruction::Rem(t, Signedness::Unsigned) => rem_u(thread, t)?,
        Instruction::And(t) => and(thread, t)?,
        Instruction::Or(t) => or(thread, t)?,
        Instruction::Xor(t) => xor(thread, t)?,
        Instruction::Shl(t) => shl(thread, t)?,
        Instruction::Shr(t, Signedness::Signed) => shr_s(thread, t)?,
        Instruction::Shr(t, Signedness::Unsigned) => shr_u(thread, t)?,
        Instruction::Rotl(t) => rotl(thread, t)?,
        Instruction::Rotr(t) => rotr(thread, t)?,
        Instruction::Clz(t) => clz(thread, t)?,
        Instruction::Ctz(t) => ctz(thread, t)?,
        Instruction::Popcnt(t) => popcnt(thread, t)?,
        Instruction::Eqz(t) => eqz(thread, t)?,
        Instruction::Eq(t) => eq(thread, t)?,
        Instruction::Ne(t) => ne(thread, t)?,
        Instruction::Gt(t, Signedness::Signed) => gt_s(thread, t)?,
        Instruction::Ge(t, Signedness::Signed) => ge_s(thread, t)?,
        Instruction::Lt(t, Signedness::Signed) => lt_s(thread, t)?,
        Instruction::Le(t, Signedness::Signed) => le_s(thread, t)?,
        Instruction::Gt(t, Signedness::Unsigned) => gt_u(thread, t)?,
        Instruction::Ge(t, Signedness::Unsigned) => ge_u(thread, t)?,
        Instruction::Lt(t, Signedness::Unsigned) => lt_u(thread, t)?,
        Instruction::Le(t, Signedness::Unsigned) => le_u(thread, t)?,

        Instruction::Wrap => wrap(thread, ValType::I64)?,
        Instruction::Extend(Signedness::Signed) => extend_s(thread, ValType::I32)?,
        Instruction::Extend(Signedness::Unsigned) => extend_u(thread, ValType::I32)?,

        Instruction::Drop => {
            thread.stack_mut().pop()?;
        }
        x => return Err(format!("Instruction not implemented: {}", x).into()),
    };

    Ok(())
}
binop!(add,
    (I32, c1, c2) => Ok(Value::I32(c1.wrapping_add(c2))),
    (I64, c1, c2) => Ok(Value::I64(c1.wrapping_add(c2))),
    (F32, c1, c2) => Ok(Value::F32(c1 + c2))
);
binop!(sub,
    (I32, c1, c2) => Ok(Value::I32(c1.wrapping_sub(c2))),
    (I64, c1, c2) => Ok(Value::I64(c1.wrapping_sub(c2))),
    (F32, c1, c2) => Ok(Value::F32(c1 - c2))
);
binop!(mul,
    (I32, c1, c2) => Ok(Value::I32(c1.wrapping_mul(c2))),
    (I64, c1, c2) => Ok(Value::I64(c1.wrapping_mul(c2))),
    (F32, c1, c2) => Ok(Value::F32(c1 * c2))
);
binop!(div_s,
    (I32, c1, c2) => int32::signed_div_helper(c1, c2),
    (I64, c1, c2) => int64::signed_div_helper(c1, c2)
);
binop!(div_u,
    (I32, c1, c2) => int32::unsigned_div_helper(c1, c2),
    (I64, c1, c2) => int64::unsigned_div_helper(c1, c2)
);
binop!(fdiv,
    (F32, c1, c2) => Ok(Value::F32(c1 / c2))
);
binop!(rem_s,
    (I32, c1, c2) => int32::signed_rem_helper(c1, c2),
    (I64, c1, c2) => int64::signed_rem_helper(c1, c2)
);
binop!(rem_u,
    (I32, c1, c2) => int32::unsigned_rem_helper(c1, c2),
    (I64, c1, c2) => int64::unsigned_rem_helper(c1, c2)
);
binop!(and,
    (I32, c1, c2) => Ok(Value::I32(c1 & c2)),
    (I64, c1, c2) => Ok(Value::I64(c1 & c2))
);
binop!(or,
    (I32, c1, c2) => Ok(Value::I32(c1 | c2)),
    (I64, c1, c2) => Ok(Value::I64(c1 | c2))
);
binop!(xor,
    (I32, c1, c2) => Ok(Value::I32(c1 ^ c2)),
    (I64, c1, c2) => Ok(Value::I64(c1 ^ c2))
);
binop!(shl,
    (I32, c1, c2) => Ok(Value::I32(c1.wrapping_shl(c2 as u32))),
    (I64, c1, c2) => Ok(Value::I64(c1.wrapping_shl(c2 as u32)))
);
binop!(shr_u,
    (I32, c1, c2) => Ok(Value::I32(c1.wrapping_shr(c2))),
    (I64, c1, c2) => Ok(Value::I64(c1.wrapping_shr(c2 as u32)))
);
binop!(shr_s,
    (I32, c1, c2) => Ok(Value::I32((c1 as i32).wrapping_shr(c2) as u32)),
    (I64, c1, c2) => Ok(Value::I64((c1 as i64).wrapping_shr(c2 as u32) as u64))
);
binop!(rotl,
    (I32, c1, c2) => Ok(Value::I32(c1.rotate_left(c2 as u32))),
    (I64, c1, c2) => Ok(Value::I64(c1.rotate_left(c2 as u32)))
);
binop!(rotr,
    (I32, c1, c2) => Ok(Value::I32(c1.rotate_right(c2 as u32))),
    (I64, c1, c2) => Ok(Value::I64(c1.rotate_right(c2 as u32)))
);

// Comparison ops alwac2s return i32
binop!(eq,
    (I32, c1, c2) => Ok(Value::I32(if c1 == c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 == c2 { 1 } else { 0 }))
);
binop!(ne,
    (I32, c1, c2) => Ok(Value::I32(if c1 != c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 != c2 { 1 } else { 0 }))
);
binop!(gt_s,
    (I32, c1, c2) => Ok(Value::I32(if (c1 as i32) > (c2 as i32) { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if (c1 as i64) > (c2 as i64) { 1 } else { 0 }))
);
binop!(gt_u,
    (I32, c1, c2) => Ok(Value::I32(if c1 > c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 > c2 { 1 } else { 0 }))
);
binop!(ge_s,
    (I32, c1, c2) => Ok(Value::I32(if (c1 as i32) >= (c2 as i32) { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if (c1 as i64) >= (c2 as i64) { 1 } else { 0 }))
);
binop!(ge_u,
    (I32, c1, c2) => Ok(Value::I32(if c1 >= c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 >= c2 { 1 } else { 0 }))
);
binop!(lt_s,
    (I32, c1, c2) => Ok(Value::I32(if (c1 as i32) < (c2 as i32) { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if (c1 as i64) < (c2 as i64) { 1 } else { 0 }))
);
binop!(lt_u,
    (I32, c1, c2) => Ok(Value::I32(if c1 < c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 < c2 { 1 } else { 0 }))
);
binop!(le_s,
    (I32, c1, c2) => Ok(Value::I32(if (c1 as i32) <= (c2 as i32) { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if (c1 as i64) <= (c2 as i64) { 1 } else { 0 }))
);
binop!(le_u,
    (I32, c1, c2) => Ok(Value::I32(if c1 <= c2 { 1 } else { 0 })),
    (I64, c1, c2) => Ok(Value::I32(if c1 <= c2 { 1 } else { 0 }))
);

unop!(clz,
    (I32, c1) => Ok(Value::I32(c1.leading_zeros())),
    (I64, c1) => Ok(Value::I64(c1.leading_zeros() as u64))
);
unop!(ctz,
    (I32, c1) => Ok(Value::I32(c1.trailing_zeros())),
    (I64, c1) => Ok(Value::I64(c1.trailing_zeros() as u64))
);
unop!(popcnt,
    (I32, c1) => Ok(Value::I32(c1.count_ones())),
    (I64, c1) => Ok(Value::I64(c1.count_ones() as u64))
);

// Comparison ops alreadc2 return i32
unop!(eqz,
    (I32, c1) => Ok(Value::I32(if c1 == 0 { 1 } else { 0 })),
    (I64, c1) => Ok(Value::I32(if c1 == 0 { 1 } else { 0 }))
);

unop!(wrap, (I64, c1) => Ok(Value::I32(c1 as u32)));
unop!(extend_u, (I32, c1) => Ok(Value::I64(c1 as u64)));
unop!(extend_s, (I32, c1) => Ok(Value::I64((c1 as i32) as i64 as u64)));

macro_rules! div_helpers {
    ($name: ident, $unsigned: ty, $signed: ty, $valtyp: ident) => {
        mod $name {
            use crate::{Value, Trap};
        
            pub fn signed_div_helper(
                x: $unsigned,
                y: $unsigned,
            ) -> Result<Value, Trap> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    match (x as $signed).checked_div(y as $signed) {
                        Some(y) => Ok(Value::$valtyp(y as $unsigned)),
                        None => Err("integer overflow".into()),
                    }
                }
            }
        
            pub fn unsigned_div_helper(
                x: $unsigned,
                y: $unsigned,
            ) -> Result<Value, Trap> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    match x.checked_div(y) {
                        Some(y) => Ok(Value::$valtyp(y)),
                        None => Err("integer overflow".into()),
                    }
                }
            }
        
            pub fn signed_rem_helper(
                x: $unsigned,
                y: $unsigned,
            ) -> Result<Value, Trap> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    Ok(Value::$valtyp(
                        (x as $signed).overflowing_rem(y as $signed).0 as $unsigned,
                    ))
                }
            }
        
            pub fn unsigned_rem_helper(
                x: $unsigned,
                y: $unsigned,
            ) -> Result<Value, Trap> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    match x.checked_rem(y) {
                        Some(y) => Ok(Value::$valtyp(y)),
                        None => Err("integer overflow".into()),
                    }
                }
            }
        }
    };
}

div_helpers!(int32, u32, i32, I32);
div_helpers!(int64, u64, i64, I64);
