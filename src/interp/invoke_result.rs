use std::borrow::Cow;

use crate::Value;

pub enum InvokeResult {
    Empty,
    Return(Vec<Value>),
    Trap(Cow<'static, str>),
}

impl InvokeResult {
    pub fn trap<S: Into<Cow<'static, str>>>(failure: S) -> InvokeResult {
        InvokeResult::Trap(failure.into())
    }

    pub fn is_trap(&self) -> bool {
        match self {
            InvokeResult::Trap(_) => true,
            _ => false,
        }
    }
}
