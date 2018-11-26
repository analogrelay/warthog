use std::sync::Arc;

use crate::{
    hosting::{ExternalFunc, ExternalMemory, ExternalModule, Host},
    interp::{Thread, Trap},
    module::{FuncType, ValType},
    Value,
};

pub struct SpecTest {
    funcs: Vec<Arc<ExternalFunc>>,
}

impl SpecTest {
    pub fn new() -> SpecTest {
        SpecTest {
            funcs: vec![Arc::new(ExternalFunc::new(
                "print_i32",
                FuncType::new(vec![ValType::Integer32], vec![]),
                print_i32,
            ))],
        }
    }
}

impl ExternalModule for SpecTest {
    fn name(&self) -> &str {
        "spectest"
    }

    fn funcs(&self) -> &[Arc<ExternalFunc>] {
        &self.funcs
    }

    fn mems(&self) -> &[ExternalMemory] {
        &[]
    }
}

fn print_i32(_host: &mut Host, thread: &mut Thread, values: &[Value]) -> Result<Vec<Value>, Trap> {
    let value = match values.iter().next() {
        Some(Value::Integer32(v)) => v,
        Some(v) => {
            return Err(thread.throw(format!(
                "Type mismatch, expected an i32 but found a {}",
                v.typ()
            )))
        }
        None => return Err(thread.throw("Stack underflow!")),
    };
    println!("{} : {}", value, ValType::Integer32);

    Ok(Vec::new())
}
