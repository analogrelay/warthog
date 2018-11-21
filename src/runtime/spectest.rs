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

fn print_i32(value: u32) -> Result<Vec<Value>, Trap> {
    println!("{} : {}", value, ValType::Integer32);

    Ok(Vec::new())
}
