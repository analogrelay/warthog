use crate::{runtime::ModuleAddr, Value};

pub struct StackFrame {
    values: Vec<Value>,
    module: ModuleAddr,
    locals: Vec<Value>,
}

impl StackFrame {
    pub fn new(module: ModuleAddr, locals: Vec<Value>) -> StackFrame {
        StackFrame {
            values: Vec::new(),
            module,
            locals,
        }
    }

    pub fn module(&self) -> ModuleAddr {
        self.module
    }

    pub fn push(&mut self, value: Value) {
        // Don't push nils, just drop them.
        if value != Value::Nil {
            self.values.push(value)
        }
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn new() -> Stack {
        Stack(vec![StackFrame::new(ModuleAddr::NULL, Vec::new())])
    }

    /// Pushes a new stack frame on to the stack
    pub fn enter(&mut self, module: ModuleAddr, locals: Vec<Value>) {
        self.0.push(StackFrame::new(module, locals))
    }

    /// Gets the active stack frame
    pub fn frame(&self) -> &StackFrame {
        // We can unwrap because we ensure there is always at least one item
        self.0.last().unwrap()
    }

    /// Gets the active stack frame
    pub fn frame_mut(&mut self) -> &mut StackFrame {
        // We can unwrap because we ensure there is always at least one item
        self.0.last_mut().unwrap()
    }

    /// Pops the current frame (and all values associated with it) off the stack
    pub fn exit(&mut self) {
        if self.0.len() == 1 {
            panic!("There is no current frame to exit!");
        } else {
            self.0.pop();
        }
    }

    /// Pushes a value on to the current stack frame.
    pub fn push<V: Into<Value>>(&mut self, value: V) {
        self.frame_mut().push(value.into());
    }

    /// Pops a value on from the current stack frame.
    pub fn pop(&mut self) -> Option<Value> {
        self.frame_mut().pop()
    }

    /// Gets the address of the module currently at the top of the stack.
    pub fn module(&self) -> ModuleAddr {
        self.frame().module()
    }
}
