use crate::{
    interp::{Thread, Trap},
    module::FuncType,
    runtime::Host,
    Value,
};

pub type HostFunc =
    fn(host: &mut Host, thread: &mut Thread, values: &[Value]) -> Result<Vec<Value>, Trap>;

pub struct SyntheticFunc {
    pub name: String,
    pub typ: FuncType,
    pub imp: HostFunc,
}

impl SyntheticFunc {
    pub fn new<S: Into<String>>(name: S, typ: FuncType, imp: HostFunc) -> SyntheticFunc {
        SyntheticFunc {
            name: name.into(),
            typ,
            imp,
        }
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
