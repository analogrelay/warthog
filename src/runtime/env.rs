use std::sync::Arc;

use crate::{
    hosting::{ExternalFunc, ExternalMemory, ExternalModule, Host},
    interp::Thread,
    module::{FuncType, ValType},
    Trap, Value,
};

pub struct Env {
    funcs: Vec<Arc<ExternalFunc>>,
    mems: Vec<ExternalMemory>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            funcs: vec![Arc::new(ExternalFunc::new(
                "print",
                FuncType::new(vec![ValType::Integer32, ValType::Integer32], vec![]),
                print,
            ))],
            mems: vec![ExternalMemory::new("memory", 256, Some(256))],
        }
    }
}

impl ExternalModule for Env {
    fn name(&self) -> &str {
        "env"
    }

    fn funcs(&self) -> &[Arc<ExternalFunc>] {
        &self.funcs
    }

    fn mems(&self) -> &[ExternalMemory] {
        &self.mems
    }
}

fn print(host: &mut Host, thread: &mut Thread, values: &[Value]) -> Result<Vec<Value>, Trap> {
    let (count, start) = (
        values[0].unwrap_u32() as usize,
        values[1].unwrap_u32() as usize,
    );

    let module = thread.stack().current().frame().module();
    let end = start + count;

    // Get memory 0 for the current frame
    let mem_addr = host.resolve_mem(module, 0);
    let mem_inst = host.get_mem(mem_addr);
    let mem = mem_inst.memory();

    // Safe as long as other threads (which don't even exist in WASM yet)
    // aren't accessing memory. When threading exists, WASM will provide
    // it's own synchronization primitives.
    unsafe {
        // Read the memory sequence in as a UTF-8 string
        let s = std::str::from_utf8(&mem.data()[start..end]).unwrap();
        println!("{}", s);
    }

    Ok(Vec::new())
}
