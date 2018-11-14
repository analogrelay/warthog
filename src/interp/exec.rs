use crate::{
    interp::{Thread, Trap},
    module::{Instruction, Signedness, ValType},
    runtime::Host,
    Value,
};

pub fn execute(thread: &mut Thread, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
    match inst {
        Instruction::Const(val) => thread.push(val.clone()),
        Instruction::Call(func_idx) => {
            let module_addr = match thread.stack().module() {
                Some(m) => m,
                None => return Err(thread.throw("No module in scope."))
            };
            let func = host.resolve_func(module_addr, func_idx);
            let values = thread.invoke(host, func)?;

            // Push the result values on to the stack
            for value in values {
                thread.push(value)
            }
        },
        Instruction::GetLocal(local_idx) => {
            let val = match thread.stack().local(local_idx) {
                Some(l) => l,
                None => return Err(thread.throw(format!("No such local: {}", local_idx))),
            };
            thread.push(val);
        }
        Instruction::Add(t) => add(thread, t)?,
        Instruction::Sub(t) => sub(thread, t)?,
        Instruction::Mul(t) => mul(thread, t)?,
        Instruction::Div(t, Signedness::Signed) => div_s(thread, t)?,
        Instruction::Div(t, Signedness::Unsigned) => div_u(thread, t)?,
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

        Instruction::Drop => { thread.pop()?; },
        x => return Err(thread.throw(format!("Instruction not implemented: {}", x))),
    };

    Ok(())
}

macro_rules! binop {
    ($name: ident, $(($typ: ident, $x: ident, $y: ident) => $bl: expr),+) => {
        fn $name(thread: &mut Thread, typ: ValType) -> Result<(), Trap> {
            let x = thread.pop()?;
            let y = thread.pop()?;

            let res = match (typ, x, y) {
                $(
                    (ValType::$typ, Value::$typ($x), Value::$typ($y)) => {
                        $bl as Result<Value, ::std::borrow::Cow<'static, str>>
                    }
                ),+
                (t, x, y) => return Err(thread.throw(format!(
                    concat!("Type mismatch. Unable to ", stringify!($name), " {} and {} using {}.", stringify!($name)),
                    x.typ(),
                    y.typ(),
                    t
                ))),
            };

            match res {
                Ok(r) => {
                    thread.push(r);
                    Ok(())
                },
                Err(e) => {
                    Err(thread.throw(e))
                }
            }
        }
    };
}

macro_rules! unop {
    ($name: ident, $(($typ: ident, $x: ident) => $bl: expr),+) => {
        fn $name(thread: &mut Thread, typ: ValType) -> Result<(), Trap> {
            let x = thread.pop()?;

            let res = match (typ, x) {
                $(
                    (ValType::$typ, Value::$typ($x)) => {
                        $bl as Result<Value, ::std::borrow::Cow<'static, str>>
                    }
                ),+
                (t, x) => return Err(thread.throw(format!(
                    concat!("Type mismatch. Unable to ", stringify!($name), " {} using {}.", stringify!($name)),
                    x.typ(),
                    t
                ))),
            };

            match res {
                Ok(r) => {
                    thread.push(r);
                    Ok(())
                },
                Err(e) => {
                    Err(thread.throw(e))
                }
            }
        }
    };
}

binop!(add,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_add(y))),
    (Integer64, x, y) => Ok(Value::Integer64(x.wrapping_add(y)))
);
binop!(sub,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_sub(y))),
    (Integer64, x, y) => Ok(Value::Integer64(x.wrapping_sub(y)))
);
binop!(mul,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_mul(y))),
    (Integer64, x, y) => Ok(Value::Integer64(x.wrapping_mul(y)))
);
binop!(div_s,
    (Integer32, x, y) => int32::signed_div_helper(x, y),
    (Integer64, x, y) => int64::signed_div_helper(x, y)
);
binop!(div_u,
    (Integer32, x, y) => int32::unsigned_div_helper(x, y),
    (Integer64, x, y) => int64::unsigned_div_helper(x, y)
);
binop!(rem_s,
    (Integer32, x, y) => int32::signed_rem_helper(x, y),
    (Integer64, x, y) => int64::signed_rem_helper(x, y)
);
binop!(rem_u,
    (Integer32, x, y) => int32::unsigned_rem_helper(x, y),
    (Integer64, x, y) => int64::unsigned_rem_helper(x, y)
);
binop!(and,
    (Integer32, x, y) => Ok(Value::Integer32(x & y)),
    (Integer64, x, y) => Ok(Value::Integer64(x & y))
);
binop!(or,
    (Integer32, x, y) => Ok(Value::Integer32(x | y)),
    (Integer64, x, y) => Ok(Value::Integer64(x | y))
);
binop!(xor,
    (Integer32, x, y) => Ok(Value::Integer32(x ^ y)),
    (Integer64, x, y) => Ok(Value::Integer64(x ^ y))
);
binop!(shl,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_shl(y as u32))),
    (Integer64, x, y) => Ok(Value::Integer64(x.wrapping_shl(y as u32)))
);
binop!(shr_u,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_shr(y))),
    (Integer64, x, y) => Ok(Value::Integer64(x.wrapping_shr(y as u32)))
);
binop!(shr_s,
    (Integer32, x, y) => Ok(Value::Integer32((x as i32).wrapping_shr(y) as u32)),
    (Integer64, x, y) => Ok(Value::Integer64((x as i64).wrapping_shr(y as u32) as u64))
);
binop!(rotl,
    (Integer32, x, y) => Ok(Value::Integer32(x.rotate_left(y as u32))),
    (Integer64, x, y) => Ok(Value::Integer64(x.rotate_left(y as u32)))
);
binop!(rotr,
    (Integer32, x, y) => Ok(Value::Integer32(x.rotate_right(y as u32))),
    (Integer64, x, y) => Ok(Value::Integer64(x.rotate_right(y as u32)))
);

// Comparison ops always return i32
binop!(eq,
    (Integer32, x, y) => Ok(Value::Integer32(if x == y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x == y { 1 } else { 0 }))
);
binop!(ne,
    (Integer32, x, y) => Ok(Value::Integer32(if x != y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x != y { 1 } else { 0 }))
);
binop!(gt_s,
    (Integer32, x, y) => Ok(Value::Integer32(if (x as i32) > (y as i32) { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if (x as i64) > (y as i64) { 1 } else { 0 }))
);
binop!(gt_u,
    (Integer32, x, y) => Ok(Value::Integer32(if x > y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x > y { 1 } else { 0 }))
);
binop!(ge_s,
    (Integer32, x, y) => Ok(Value::Integer32(if (x as i32) >= (y as i32) { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if (x as i64) >= (y as i64) { 1 } else { 0 }))
);
binop!(ge_u,
    (Integer32, x, y) => Ok(Value::Integer32(if x >= y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x >= y { 1 } else { 0 }))
);
binop!(lt_s,
    (Integer32, x, y) => Ok(Value::Integer32(if (x as i32) < (y as i32) { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if (x as i64) < (y as i64) { 1 } else { 0 }))
);
binop!(lt_u,
    (Integer32, x, y) => Ok(Value::Integer32(if x < y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x < y { 1 } else { 0 }))
);
binop!(le_s,
    (Integer32, x, y) => Ok(Value::Integer32(if (x as i32) <= (y as i32) { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if (x as i64) <= (y as i64) { 1 } else { 0 }))
);
binop!(le_u,
    (Integer32, x, y) => Ok(Value::Integer32(if x <= y { 1 } else { 0 })),
    (Integer64, x, y) => Ok(Value::Integer32(if x <= y { 1 } else { 0 }))
);

unop!(clz,
    (Integer32, x) => Ok(Value::Integer32(x.leading_zeros())),
    (Integer64, x) => Ok(Value::Integer64(x.leading_zeros() as u64))
);
unop!(ctz,
    (Integer32, x) => Ok(Value::Integer32(x.trailing_zeros())),
    (Integer64, x) => Ok(Value::Integer64(x.trailing_zeros() as u64))
);
unop!(popcnt,
    (Integer32, x) => Ok(Value::Integer32(x.count_ones())),
    (Integer64, x) => Ok(Value::Integer64(x.count_ones() as u64))
);

// Comparison ops already return i32
unop!(eqz,
    (Integer32, x) => Ok(Value::Integer32(if x == 0 { 1 } else { 0 })),
    (Integer64, x) => Ok(Value::Integer32(if x == 0 { 1 } else { 0 }))
);

macro_rules! div_helpers {
    ($name: ident, $unsigned: ty, $signed: ty, $valtyp: ident) => {
        mod $name {
            use std::borrow::Cow;
            use crate::Value;

            pub fn signed_div_helper(x: $unsigned, y: $unsigned) -> Result<Value, Cow<'static, str>> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    match (x as $signed).checked_div(y as $signed) {
                        Some(y) => Ok(Value::$valtyp(y as $unsigned)),
                        None => Err("integer overflow".into()),
                    }
                }
            }

            pub fn unsigned_div_helper(x: $unsigned, y: $unsigned) -> Result<Value, Cow<'static, str>> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    match x.checked_div(y) {
                        Some(y) => Ok(Value::$valtyp(y)),
                        None => Err("integer overflow".into()),
                    }
                }
            }

            pub fn signed_rem_helper(x: $unsigned, y: $unsigned) -> Result<Value, Cow<'static, str>> {
                if y == 0 {
                    Err("integer divide by zero".into())
                } else {
                    Ok(Value::$valtyp(
                        (x as $signed).overflowing_rem(y as $signed).0 as $unsigned,
                    ))
                }
            }

            pub fn unsigned_rem_helper(x: $unsigned, y: $unsigned) -> Result<Value, Cow<'static, str>> {
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
    }
}

div_helpers!(int32, u32, i32, Integer32);
div_helpers!(int64, u64, i64, Integer64);
