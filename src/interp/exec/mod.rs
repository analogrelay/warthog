use crate::{hosting::Host, interp::Thread, Instruction, Trap};

mod numops;

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
        _ => numops::exec(thread, inst)?,
    };

    Ok(())
}
