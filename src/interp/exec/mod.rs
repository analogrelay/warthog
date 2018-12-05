use crate::{hosting::Host, interp::Thread, Instruction, Trap};

mod numops;

pub fn execute(thread: &mut Thread, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
    use crate::Instruction::*;

    match inst {
        I32Const(v) => thread.push(v),
        I64Const(v) => thread.push(v),
        F32Const(v) => thread.push(v),
        F64Const(v) => thread.push(v),
        Call(func_idx) => {
            let module_addr = thread.stack().current().frame().module();
            let func = host.resolve_func(module_addr, func_idx as usize);
            let values = thread.invoke(host, func)?;

            // Push the result values on to the stack
            for value in values {
                thread.push(value)
            }
        }
        GetLocal(local_idx) => {
            let val = match thread.stack().current().local(local_idx as usize) {
                Some(l) => l,
                None => return Err(format!("No such local: {}", local_idx).into()),
            };
            thread.push(val);
        }
        _ => numops::exec(thread, inst)?,
    };

    Ok(())
}
