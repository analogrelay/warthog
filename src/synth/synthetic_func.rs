use crate::{interp::Thread, module::FuncType, Value};

pub type HostFunc = fn(thread: &mut Thread, values: &[Value]) -> Value;

pub struct SyntheticFunc {
    pub typ: FuncType,
    pub imp: HostFunc,
}

impl SyntheticFunc {
    pub fn new(typ: FuncType, imp: HostFunc) -> SyntheticFunc {
        SyntheticFunc { typ, imp }
    }

    pub fn invoke(&self, thread: &mut Thread) -> Value {
        // Pop values off the stack
        let values: Vec<_> = {
            let stack = thread.stack_mut();
            self.typ
                .parameters
                .iter()
                .rev()
                .map(|t| {
                    let v = stack.pop().expect("expected a value on the stack!");
                    debug_assert!(
                        v.typ() == *t,
                        "expected the value to be a {}, but it was a {}",
                        t,
                        v.typ()
                    );
                    v
                }).collect()
        };

        (self.imp)(thread, &values)
    }
}
