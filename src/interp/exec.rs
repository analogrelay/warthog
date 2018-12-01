use std::ops;

use crate::{hosting::Host, interp::Thread, value, FromValue, Instruction, Trap, Value};

pub fn execute(thread: &mut Thread, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
    use crate::Opcode::*;

    match inst.opcode {
        I32Const | I64Const | F32Const | F64Const => thread.push(inst.unwrap_const()),
        Call => {
            let func_idx = inst.unwrap_idx() as usize;
            let module_addr = thread.stack().current().frame().module();
            let func = host.resolve_func(module_addr, func_idx);
            let values = thread.invoke(host, func)?;

            // Push the result values on to the stack
            for value in values {
                thread.push(value)
            }
        }
        GetLocal => {
            let local_idx = inst.unwrap_idx() as usize;
            let val = match thread.stack().current().local(local_idx) {
                Some(l) => l,
                None => return Err(format!("No such local: {}", local_idx).into()),
            };
            thread.push(val);
        }
        _ => exec_numeric(thread, inst)?,
    };

    Ok(())
}

fn exec_numeric(thread: &mut Thread, inst: Instruction) -> Result<(), Trap> {
    use crate::Opcode::*;

    match inst.opcode {
        I32Clz => clz::<u32>(thread),
        I32Ctz => ctz::<u32>(thread),
        I32Popcnt => popcnt::<u32>(thread),
        I32Add => add::<u32>(thread),
        I32Sub => sub::<u32>(thread),
        I32Mul => mul::<u32>(thread),
        I32Div_S => div::<i32>(thread),
        I32Div_U => div::<u32>(thread),
        I32Rem_S => rem::<i32>(thread),
        I32Rem_U => rem::<u32>(thread),
        I32And => and::<u32>(thread),
        I32Or => or::<u32>(thread),
        I32Xor => xor::<u32>(thread),
        I32Shl => shl::<u32>(thread),
        I32Shr_S => shr::<i32>(thread),
        I32Shr_U => shr::<u32>(thread),
        I32Rotl => rotl::<u32>(thread),
        I32Rotr => rotr::<u32>(thread),

        x => return Err(format!("Instruction not implemented: {}", x).into()),
    }
}

macro_rules! impl_panic {
    ($name: ident) => {
        fn $name<T>(_thread: &mut Thread) -> Result<(), Trap> {
            unimplemented!(concat!("not yet implemented: ", stringify!($name)))
        }
    };
}

macro_rules! maybe_try {
    ($e: expr, true) => {
        $e?
    };
    ($e: expr, false) => {
        $e
    };
}

macro_rules! impl_binop {
    // We use repetition over ident instead of path
    // because path doesn't allow affixing a generic arg on the end
    (try, $name: ident, $($path_elem: ident)::*) => {
        impl_binop_core!(true, $name, $name, $($path_elem)::*);
    };
    (notry, $name: ident, $($path_elem: ident)::*) => {
        impl_binop_core!(false, $name, $name, $($path_elem)::*);
    };
    (try, $name: ident, $method: ident, $($path_elem: ident)::*) => {
        impl_binop_core!(true, $name, $method, $($path_elem)::*);
    };
    (notry, $name: ident, $method: ident, $($path_elem: ident)::*) => {
        impl_binop_core!(false, $name, $method, $($path_elem)::*);
    };
}

macro_rules! impl_binop_core {
    ($try: ident, $name: ident, $method:ident, $($path_elem: ident)::*) => {
        fn $name<T>(thread: &mut Thread) -> Result<(), Trap>
        where
            T: FromValue + $($path_elem)::*,
            Value: From<<T as $($path_elem)::*>::Output>,
        {
            let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
            let res = maybe_try!(left.$method(right), $try);
            thread.stack_mut().push(res);
            Ok(())
        }
    };
}

macro_rules! impl_unop {
    // We use repetition over ident instead of path
    // because path doesn't allow affixing a generic arg on the end
    (try, $name: ident, $($path_elem: ident)::*) => {
        impl_unop_core!(true, $name, $name, $($path_elem)::*);
    };
    (notry, $name: ident, $($path_elem: ident)::*) => {
        impl_unop_core!(false, $name, $name, $($path_elem)::*);
    };
    (try, $name: ident, $method: ident, $($path_elem: ident)::*) => {
        impl_unop_core!(true, $name, $method, $($path_elem)::*);
    };
    (notry, $name: ident, $method: ident, $($path_elem: ident)::*) => {
        impl_unop_core!(false, $name, $method, $($path_elem)::*);
    };
}

macro_rules! impl_unop_core {
    ($try: ident, $name: ident, $method: ident, $($path_elem: ident)::*) => {
        fn $name<T>(thread: &mut Thread) -> Result<(), Trap>
        where
            T: FromValue + $($path_elem)::*<T>,
            Value: From<<T as $($path_elem)::*>::Output>,
        {
            let val = thread.stack_mut().pop_as::<T>()?;
            let res = maybe_try!(val.$method(), $try);
            thread.stack_mut().push(res);
            Ok(())
        }
    };
}

impl_unop!(notry, clz, value::IntegerOps);
impl_unop!(notry, ctz, value::IntegerOps);
impl_unop!(notry, popcnt, value::IntegerOps);
impl_binop!(notry, add, value::ArithmeticOps);
impl_binop!(notry, sub, value::ArithmeticOps);
impl_binop!(notry, mul, value::ArithmeticOps);
impl_binop!(try, div, value::ArithmeticOps);
impl_binop!(try, rem, value::IntegerOps);
impl_binop!(notry, and, bitand, ops::BitAnd);
impl_binop!(notry, or, bitor, ops::BitOr);
impl_binop!(notry, xor, bitxor, ops::BitXor);
impl_binop!(notry, shl, value::IntegerOps);
impl_binop!(notry, shr, value::IntegerOps);
impl_binop!(notry, rotl, value::IntegerOps);
impl_binop!(notry, rotr, value::IntegerOps);
