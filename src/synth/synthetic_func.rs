use crate::{interp::Thread, module::FuncType, Value};

pub struct SyntheticFunc {
    pub typ: FuncType,
    pub imp: fn(&mut Thread) -> Value,
}

impl SyntheticFunc {
    pub fn new(typ: FuncType, imp: fn(&mut Thread) -> Value) -> SyntheticFunc {
        SyntheticFunc { typ, imp }
    }

    pub fn invoke(&self, thread: &mut Thread) -> Value {
        (self.imp)(thread)
    }
}
