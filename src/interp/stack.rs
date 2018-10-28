use crate::runtime::ModuleAddr;

pub enum StackItem {
    Integer32(u32),
    Integer64(u64),
    Float32(f32),
    Float64(f64),

    // TODO: Figure out what to put here.
    Label,
    Activation(ModuleAddr),
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
