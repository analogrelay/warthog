use crate::{hosting::Host, interp::Thread, FromValue, Instruction, Trap, Value};

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

        I32Clz => clz::<u32>(thread)?,
        I32Ctz => ctz::<u32>(thread)?,
        I32Popcnt => popcnt::<u32>(thread)?,
        I32Add => add::<u32>(thread)?,
        I32Sub => sub::<u32>(thread)?,
        I32Mul => mul::<u32>(thread)?,
        I32Div_S => div::<i32>(thread)?,
        I32Div_U => div::<u32>(thread)?,
        I32Rem_S => rem::<i32>(thread)?,
        I32Rem_U => rem::<u32>(thread)?,
        I32And => and::<u32>(thread)?,
        I32Or => or::<u32>(thread)?,
        I32Xor => xor::<u32>(thread)?,
        I32Shl => shl::<u32>(thread)?,
        I32Shr_S => shr::<i32>(thread)?,
        I32Shr_U => shr::<u32>(thread)?,
        I32Rotl => rotl::<u32>(thread)?,
        I32Rotr => rotr::<u32>(thread)?,

        x => return Err(format!("Instruction not implemented: {}", x).into()),
    };

    Ok(())
}

macro_rules! impl_panic {
    ($name: ident) => {
        fn $name<T>(_thread: &mut Thread) -> Result<(), Trap> {
            unimplemented!(concat!("not yet implemented: ", stringify!($name)))
        }
    };
}

macro_rules! impl_arith {
    ($name: ident) => {
        fn $name<T>(thread: &mut Thread) -> Result<(), Trap>
        where
            T: FromValue + $crate::value::ArithmeticOps<T>,
            Value: From<T>,
        {
            let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
            let res = left.$name(right)?;
            thread.stack_mut().push(res);
            Ok(())
        }
    };
}

macro_rules! impl_int_unop {
    ($name: ident) => {
        fn $name<T>(thread: &mut Thread) -> Result<(), Trap>
        where
            T: FromValue + $crate::value::IntegerOps<T>,
            Value: From<T>,
        {
            let val = thread.stack_mut().pop_as::<T>()?;
            let res = val.$name()?;
            thread.stack_mut().push(res);
            Ok(())
        }
    };
}

impl_int_unop!(clz);
impl_int_unop!(ctz);
impl_int_unop!(popcnt);
impl_arith!(add);
impl_arith!(sub);
impl_arith!(mul);
impl_arith!(div);

fn rem<T>(thread: &mut Thread) -> Result<(), Trap>
where
    T: FromValue + crate::value::IntegerOps<T>,
    Value: From<T>,
{
    let (left, right) = thread.stack_mut().pop_pair_as::<T, T>()?;
    let res = left.rem(right)?;
    thread.stack_mut().push(res);
    Ok(())
}

impl_panic!(and);
impl_panic!(or);
impl_panic!(xor);
impl_panic!(shl);
impl_panic!(shr);
impl_panic!(rotl);
impl_panic!(rotr);
