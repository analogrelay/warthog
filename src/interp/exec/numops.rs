use std::{cmp, ops};

use crate::{interp::Thread, value, FromValue, Instruction, Trap, Value};

pub fn exec(thread: &mut Thread, inst: Instruction) -> Result<(), Trap> {
    use crate::Opcode::*;

    match inst.opcode {
        I32Eqz => eqz::<u32>(thread),
        I32Eq => eq::<u32>(thread),
        I32Ne => ne::<u32>(thread),
        I32Lt_S => lt::<i32>(thread),
        I32Lt_U => lt::<u32>(thread),
        I32Gt_S => gt::<i32>(thread),
        I32Gt_U => gt::<u32>(thread),
        I32Le_S => le::<i32>(thread),
        I32Le_U => le::<u32>(thread),
        I32Ge_S => ge::<i32>(thread),
        I32Ge_U => ge::<u32>(thread),

        I64Eqz => eqz::<u64>(thread),
        I64Eq => eq::<u64>(thread),
        I64Ne => ne::<u64>(thread),
        I64Lt_S => lt::<i64>(thread),
        I64Lt_U => lt::<u64>(thread),
        I64Gt_S => gt::<i64>(thread),
        I64Gt_U => gt::<u64>(thread),
        I64Le_S => le::<i64>(thread),
        I64Le_U => le::<u64>(thread),
        I64Ge_S => ge::<i64>(thread),
        I64Ge_U => ge::<u64>(thread),

        F32Eq => eq::<f32>(thread),
        F32Ne => ne::<f32>(thread),
        F32Lt => lt::<f32>(thread),
        F32Gt => gt::<f32>(thread),
        F32Le => le::<f32>(thread),
        F32Ge => ge::<f32>(thread),

        F64Eq => eq::<f64>(thread),
        F64Ne => ne::<f64>(thread),
        F64Lt => lt::<f64>(thread),
        F64Gt => gt::<f64>(thread),
        F64Le => le::<f64>(thread),
        F64Ge => ge::<f64>(thread),

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

        I64Clz => clz::<u64>(thread),
        I64Ctz => ctz::<u64>(thread),
        I64Popcnt => popcnt::<u64>(thread),
        I64Add => add::<u64>(thread),
        I64Sub => sub::<u64>(thread),
        I64Mul => mul::<u64>(thread),
        I64Div_S => div::<i64>(thread),
        I64Div_U => div::<u64>(thread),
        I64Rem_S => rem::<i64>(thread),
        I64Rem_U => rem::<u64>(thread),
        I64And => and::<u64>(thread),
        I64Or => or::<u64>(thread),
        I64Xor => xor::<u64>(thread),
        I64Shl => shl::<u64>(thread),
        I64Shr_S => shr::<i64>(thread),
        I64Shr_U => shr::<u64>(thread),
        I64Rotl => rotl::<u64>(thread),
        I64Rotr => rotr::<u64>(thread),

        F32Abs => abs::<f32>(thread),
        F32Neg => neg::<f32>(thread),
        F32Ceil => ceil::<f32>(thread),
        F32Floor => floor::<f32>(thread),
        F32Trunc => trunc::<f32>(thread),
        F32Nearest => nearest::<f32>(thread),
        F32Sqrt => sqrt::<f32>(thread),
        F32Add => add::<f32>(thread),
        F32Sub => sub::<f32>(thread),
        F32Mul => mul::<f32>(thread),
        F32Div => div::<f32>(thread),
        F32Min => min::<f32>(thread),
        F32Max => max::<f32>(thread),
        F32Copysign => copysign::<f32>(thread),

        I32Wrap_I64 => convert::<u32, u64>(thread),
        I32Trunc_S_F32 => unimplemented!("I32Trunc_S_F32"),
        I32Trunc_U_F32 => unimplemented!("I32Trunc_U_F32"),
        I32Trunc_S_F64 => unimplemented!("I32Trunc_S_F64"),
        I32Trunc_U_F64 => unimplemented!("I32Trunc_U_F64"),
        I64Extend_S_I32 => convert::<i64, i32>(thread),
        I64Extend_U_I32 => convert::<u64, u32>(thread),
        I64Trunc_S_F32 => unimplemented!("I64Trunc_S_F32"),
        I64Trunc_U_F32 => unimplemented!("I64Trunc_U_F32"),
        I64Trunc_S_F64 => unimplemented!("I64Trunc_S_F64"),
        I64Trunc_U_F64 => unimplemented!("I64Trunc_U_F64"),
        F32Convert_S_I32 => unimplemented!("F32Convert_S_I32"),
        F32Convert_U_I32 => unimplemented!("F32Convert_U_I32"),
        F32Convert_S_I64 => unimplemented!("F32Convert_S_I64"),
        F32Convert_U_I64 => unimplemented!("F32Convert_U_I64"),
        F32Demote_F64 => unimplemented!("F32Demote_F64"),
        F64Convert_S_I32 => unimplemented!("F64Convert_S_I32"),
        F64Convert_U_I32 => unimplemented!("F64Convert_U_I32"),
        F64Convert_S_I64 => unimplemented!("F64Convert_S_I64"),
        F64Convert_U_I64 => unimplemented!("F64Convert_U_I64"),
        F64Promote_F32 => unimplemented!("F64Promote_F32"),
        I32Reinterpret_F32 => unimplemented!("I32Reinterpret_F32"),
        I64Reinterpret_F64 => unimplemented!("I64Reinterpret_F64"),
        F32Reinterpret_I32 => unimplemented!("F32Reinterpret_I32"),
        F64Reinterpret_I64 => unimplemented!("F64Reinterpret_I64"),

        x => return Err(format!("Instruction not implemented: {}", x).into()),
    }
}

#[allow(unused_macros)]
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

impl_unop!(notry, clz, value::ops::IntegerOps);
impl_unop!(notry, ctz, value::ops::IntegerOps);
impl_unop!(notry, popcnt, value::ops::IntegerOps);
impl_binop!(notry, add, value::ops::ArithmeticOps);
impl_binop!(notry, sub, value::ops::ArithmeticOps);
impl_binop!(notry, mul, value::ops::ArithmeticOps);
impl_binop!(try, div, value::ops::ArithmeticOps);
impl_binop!(try, rem, value::ops::IntegerOps);
impl_binop!(notry, and, bitand, ops::BitAnd);
impl_binop!(notry, or, bitor, ops::BitOr);
impl_binop!(notry, xor, bitxor, ops::BitXor);
impl_binop!(notry, shl, value::ops::IntegerOps);
impl_binop!(notry, shr, value::ops::IntegerOps);
impl_binop!(notry, rotl, value::ops::IntegerOps);
impl_binop!(notry, rotr, value::ops::IntegerOps);

fn eqz<T>(thread: &mut Thread) -> Result<(), Trap>
where
    T: FromValue,   // Convert Value into T
    T: cmp::Eq,     // Compare Ts
    T: From<u8>,    // Need to be able to convert '0' into T
    Value: From<T>, // Convert T back to Value
{
    let val = thread.stack_mut().pop_as::<T>()?;
    let res = val == 0.into();
    thread.stack_mut().push(res);
    Ok(())
}

fn eq<T>(thread: &mut Thread) -> Result<(), Trap>
where
    T: FromValue,      // Convert Value into T
    T: cmp::PartialEq, // Compare Ts
    Value: From<T>,    // Convert T back to Value
{
    let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
    let res = left == right;
    thread.stack_mut().push(res);
    Ok(())
}

fn ne<T>(thread: &mut Thread) -> Result<(), Trap>
where
    T: FromValue,      // Convert Value into T
    T: cmp::PartialEq, // Compare Ts
    Value: From<T>,    // Convert T back to Value
{
    let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
    let res = left != right;
    thread.stack_mut().push(res);
    Ok(())
}

macro_rules! impl_ord {
    ($name: ident, $($true_ord: ident),*) => {
        fn $name<T>(thread: &mut Thread) -> Result<(), Trap>
        where
            T: FromValue,   // Convert Value into T
            T: cmp::PartialOrd,    // Compare Ts
            Value: From<T>, // Convert T back to Value
        {
            let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
            let res = match left.partial_cmp(&right) {
                $(
                    Some(cmp::Ordering::$true_ord) => true,
                )*
                _ => false
            };
            thread.stack_mut().push(res);
            Ok(())
        }
    };
}

impl_ord!(lt, Less);
impl_ord!(gt, Greater);
impl_ord!(le, Less, Equal);
impl_ord!(ge, Greater, Equal);

fn convert<T, U>(thread: &mut Thread) -> Result<(), Trap>
where
    U: FromValue,
    U: value::ops::ConvertInto<T>,
    Value: From<T>,
{
    let val = thread.stack_mut().pop_as::<U>()?;
    let res: T = val.convert_into();
    thread.stack_mut().push(res);
    Ok(())
}

impl_unop!(notry, copysign, value::ops::FloatOps);
impl_binop!(notry, max, value::ops::FloatOps);
impl_binop!(notry, min, value::ops::FloatOps);
impl_unop!(notry, sqrt, value::ops::FloatOps);
impl_unop!(notry, nearest, value::ops::FloatOps);
impl_unop!(notry, trunc, value::ops::FloatOps);
impl_unop!(notry, floor, value::ops::FloatOps);
impl_unop!(notry, ceil, value::ops::FloatOps);
impl_unop!(notry, neg, value::ops::FloatOps);
impl_unop!(notry, abs, value::ops::FloatOps);
