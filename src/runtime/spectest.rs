use std::sync::Arc;

use crate::{
    hosting::{ExternalFunc, ExternalMemory, ExternalModule, Host},
    types::{ValType, FuncType},
    Trap, Value,
};

pub struct SpecTest {
    funcs: Vec<Arc<ExternalFunc>>,
}

impl SpecTest {
    pub fn new() -> SpecTest {
        SpecTest {
            funcs: vec![Arc::new(ExternalFunc::new(
                "print_i32",
                FuncType::new(vec![ValType::I32], vec![]),
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

fn print_i32(_host: &mut Host, _values: &[Value]) -> Result<Vec<Value>, Trap> {
    unimplemented!();
    // let value = match values.iter().next() {
    //     Some(Value::I32(v)) => v,
    //     Some(v) => {
    //         return Err(format!("Type mismatch, expected an i32 but found a {}", v.typ()).into())
    //     }
    //     None => return Err("Stack underflow!".into()),
    // };
    // println!("{} : {}", value, ValType::I32);

    // Ok(Vec::new())
}
