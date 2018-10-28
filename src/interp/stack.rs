use crate::{runtime::ModuleAddr, interp::Value};

pub enum StackItem {
    Val(Value),

    // TODO: Figure out what to put here.
    Label,
    Activation(ModuleAddr, Vec<Value>),
}

pub struct Stack(Vec<StackItem>);

impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::new())
    }

    pub fn push(&mut self, item: StackItem) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.0.pop()
    }
}
