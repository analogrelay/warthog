use std::sync::Arc;

use crate::{
    hosting::Host,
    interp::{Thread, Trap},
    module::{FuncType, MemoryType},
    Value,
};

pub trait ExternalModule {
    fn name(&self) -> &str;
    fn funcs(&self) -> &[Arc<ExternalFunc>];
    fn mems(&self) -> &[ExternalMemory];
}

pub type HostFunc =
    fn(host: &mut Host, thread: &mut Thread, values: &[Value]) -> Result<Vec<Value>, Trap>;

#[derive(Clone)]
pub struct ExternalFunc {
    name: String,
    typ: FuncType,
    imp: HostFunc,
}

impl ExternalFunc {
    pub fn new<S: Into<String>>(name: S, typ: FuncType, imp: HostFunc) -> ExternalFunc {
        ExternalFunc {
            name: name.into(),
            typ,
            imp,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typ(&self) -> &FuncType {
        &self.typ
    }

    pub fn invoke(&self, host: &mut Host, thread: &mut Thread) -> Result<Vec<Value>, Trap> {
        // Pop values off the stack
        let values = {
            let mut vals = Vec::new();
            for param in self.typ.params().iter() {
                match thread.pop()? {
                    v if v.typ() != *param => {
                        return Err(thread.throw(format!(
                            "Type mismatch. Function expects '{}' but '{}' is on top of the stack.",
                            param,
                            v.typ()
                        )))
                    }
                    v => vals.push(v),
                }
            }
            vals
        };

        (self.imp)(host, thread, &values)
    }
}

pub struct ExternalMemory {
    name: String,
    typ: MemoryType,
}

impl ExternalMemory {
    pub fn new<S: Into<String>>(
        name: S,
        min_size: usize,
        max_size: Option<usize>,
    ) -> ExternalMemory {
        ExternalMemory {
            name: name.into(),
            typ: MemoryType::new(min_size, max_size),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typ(&self) -> &MemoryType {
        &self.typ
    }
}
