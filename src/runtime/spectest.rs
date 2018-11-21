use std::sync::Arc;

use crate::{
    hosting::{ExternalFunc, ExternalMemory, ExternalModule},
    interp::Trap,
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
                |_, _, values| print_i32(values),
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

fn print_i32(values: &[Value]) -> Result<Vec<Value>, Trap> {
    let value = values[0].unwrap_u32() as usize;

    println!("{} : {}", value, values[0].typ());

    Ok(Vec::new())
}
