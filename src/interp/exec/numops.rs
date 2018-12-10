use std::{cmp, ops};

use crate::{interp::Thread, value, FromValue, Instruction, Trap, Value};

pub fn exec(thread: &mut Thread, inst: Instruction) -> Result<(), Trap> {
    use crate::Instruction::*;

    match inst {
        I32Eqz => eqz::<u32>(thread),
        I32Eq => eq::<u32>(thread),
        I32Ne => ne::<u32>(thread),
        I32LtS => lt::<i32>(thread),
        I32LtU => lt::<u32>(thread),
        I32GtS => gt::<i32>(thread),
        I32GtU => gt::<u32>(thread),
        I32LeS => le::<i32>(thread),
        I32LeU => le::<u32>(thread),
        I32GeS => ge::<i32>(thread),
        I32GeU => ge::<u32>(thread),

        I64Eqz => eqz::<u64>(thread),
        I64Eq => eq::<u64>(thread),
        I64Ne => ne::<u64>(thread),
        I64LtS => lt::<i64>(thread),
        I64LtU => lt::<u64>(thread),
        I64GtS => gt::<i64>(thread),
        I64GtU => gt::<u64>(thread),
        I64LeS => le::<i64>(thread),
        I64LeU => le::<u64>(thread),
        I64GeS => ge::<i64>(thread),
        I64GeU => ge::<u64>(thread),

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
        I32DivS => div::<i32>(thread),
        I32DivU => div::<u32>(thread),
        I32RemS => rem::<i32>(thread),
        I32RemU => rem::<u32>(thread),
        I32And => and::<u32>(thread),
        I32Or => or::<u32>(thread),
        I32Xor => xor::<u32>(thread),
        I32Shl => shl::<u32>(thread),
        I32ShrS => shr::<i32>(thread),
        I32ShrU => shr::<u32>(thread),
        I32Rotl => rotl::<u32>(thread),
        I32Rotr => rotr::<u32>(thread),

        I64Clz => clz::<u64>(thread),
        I64Ctz => ctz::<u64>(thread),
        I64Popcnt => popcnt::<u64>(thread),
        I64Add => add::<u64>(thread),
        I64Sub => sub::<u64>(thread),
        I64Mul => mul::<u64>(thread),
        I64DivS => div::<i64>(thread),
        I64DivU => div::<u64>(thread),
        I64RemS => rem::<i64>(thread),
        I64RemU => rem::<u64>(thread),
        I64And => and::<u64>(thread),
        I64Or => or::<u64>(thread),
        I64Xor => xor::<u64>(thread),
        I64Shl => shl::<u64>(thread),
        I64ShrS => shr::<i64>(thread),
        I64ShrU => shr::<u64>(thread),
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

        F64Abs => abs::<f64>(thread),
        F64Neg => neg::<f64>(thread),
        F64Ceil => ceil::<f64>(thread),
        F64Floor => floor::<f64>(thread),
        F64Trunc => trunc::<f64>(thread),
        F64Nearest => nearest::<f64>(thread),
        F64Sqrt => sqrt::<f64>(thread),
        F64Add => add::<f64>(thread),
        F64Sub => sub::<f64>(thread),
        F64Mul => mul::<f64>(thread),
        F64Div => div::<f64>(thread),
        F64Min => min::<f64>(thread),
        F64Max => max::<f64>(thread),
        F64Copysign => copysign::<f64>(thread),

        I32WrapI64 => convert::<u32, u64>(thread),
        I32TruncF32S => convert::<i32, f32>(thread),
        I32TruncF32U => convert::<u32, f32>(thread),
        I32TruncF64S => convert::<i32, f64>(thread),
        I32TruncF64U => convert::<u32, f64>(thread),
        I64ExtendI32S => convert::<i64, i32>(thread),
        I64ExtendI32U => convert::<u64, u32>(thread),
        I64TruncF32S => convert::<i64, f32>(thread),
        I64TruncF32U => convert::<u64, f32>(thread),
        I64TruncF64S => convert::<i64, f64>(thread),
        I64TruncF64U => convert::<u64, f64>(thread),
        F32ConvertI32S => convert::<f32, i32>(thread),
        F32ConvertI32U => convert::<f32, u32>(thread),
        F32ConvertI64S => convert::<f32, i64>(thread),
        F32ConvertI64U => convert::<f32, u64>(thread),
        F32DemoteF64 => convert::<f32, f64>(thread),
        F64ConvertI32S => convert::<f64, i32>(thread),
        F64ConvertI32U => convert::<f64, u32>(thread),
        F64ConvertI64S => convert::<f64, i64>(thread),
        F64ConvertI64U => convert::<f64, u64>(thread),
        F64PromoteF32 => convert::<f64, f32>(thread),
        I32ReinterpretF32 => reinterpret::<u32, f32>(thread),
        I64ReinterpretF64 => reinterpret::<u64, f64>(thread),
        F32ReinterpretI32 => reinterpret::<f32, u32>(thread),
        F64ReinterpretI64 => reinterpret::<f64, u64>(thread),

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
    let res: T = val.convert_into()?;
    thread.stack_mut().push(res);
    Ok(())
}

impl_binop!(notry, copysign, value::ops::FloatOps);
impl_binop!(notry, max, value::ops::FloatOps);
impl_binop!(notry, min, value::ops::FloatOps);
impl_unop!(notry, sqrt, value::ops::FloatOps);
impl_unop!(notry, nearest, value::ops::FloatOps);
impl_unop!(notry, trunc, value::ops::FloatOps);
impl_unop!(notry, floor, value::ops::FloatOps);
impl_unop!(notry, ceil, value::ops::FloatOps);
impl_unop!(notry, neg, value::ops::FloatOps);
impl_unop!(notry, abs, value::ops::FloatOps);

fn reinterpret<T, U>(thread: &mut Thread) -> Result<(), Trap>
where
    U: FromValue,
    U: value::ops::ReinterpretInto<T>,
    Value: From<T>,
{
    let val = thread.stack_mut().pop_as::<U>()?;
    let res: T = val.reinterpret_into();
    thread.stack_mut().push(res);
    Ok(())
}
