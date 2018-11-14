use std::borrow::Cow;

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
            let func = host.resolve_func(thread.stack().module(), func_idx);
            let results = thread.invoke(host, func);
            panic!("Call instruction needs to handle return values");
        }
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
                    concat!("Type mismatch. Unable to add {} and {} using {}.", stringify!($name)),
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

binop!(add,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_add(y))));
binop!(sub,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_sub(y))));
binop!(mul,
    (Integer32, x, y) => Ok(Value::Integer32(x.wrapping_mul(y))));
binop!(div_s,
    (Integer32, x, y) => i32_div_helper(x, y));
binop!(div_u,
    (Integer32, x, y) => u32_div_helper(x, y));
binop!(rem_s,
    (Integer32, x, y) => i32_rem_helper(x, y));
binop!(rem_u,
    (Integer32, x, y) => u32_rem_helper(x, y));

fn i32_div_helper(x: i32, y: i32) -> Result<Value, Cow<'static, str>> {
    if y == 0 {
        Err("integer divide by zero".into())
    } else {
        match x.checked_div(y) {
            Some(y) => Ok(Value::Integer32(y)),
            None => Err("integer overflow".into()),
        }
    }
}

fn u32_div_helper(x: i32, y: i32) -> Result<Value, Cow<'static, str>> {
    if y == 0 {
        Err("integer divide by zero".into())
    } else {
        match (x as u32).checked_div(y as u32) {
            Some(y) => Ok(Value::Integer32(y as i32)),
            None => Err("integer overflow".into()),
        }
    }
}

fn i32_rem_helper(x: i32, y: i32) -> Result<Value, Cow<'static, str>> {
    if y == 0 {
        Err("integer divide by zero".into())
    } else {
        Ok(Value::Integer32(x.wrapping_rem(y)))
    }
}

fn u32_rem_helper(x: i32, y: i32) -> Result<Value, Cow<'static, str>> {
    if y == 0 {
        Err("integer divide by zero".into())
    } else {
        Ok(Value::Integer32(x.wrapping_rem(y)))
    }
}
