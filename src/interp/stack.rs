use crate::{runtime::{FuncAddr, ModuleAddr}, Value};

pub struct StackFrame {
    values: Vec<Value>,
    module: Option<ModuleAddr>,
    _func: Option<FuncAddr>,
    locals: Vec<Value>,
}

impl StackFrame {
    pub fn new(module: Option<ModuleAddr>, func: Option<FuncAddr>, locals: Vec<Value>) -> StackFrame {
        StackFrame {
            values: Vec::new(),
            module,
            _func: func,
            locals: locals,
        }
    }

    pub fn module(&self) -> Option<ModuleAddr> {
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

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn local(&self, idx: usize) -> Option<Value> {
        if idx < self.locals.len() {
            Some(self.locals[idx])
        } else {
            None
        }
    }
}

pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn new() -> Stack {
        Stack(vec![StackFrame::new(None, None, Vec::new())])
    }

    /// Pushes a new stack frame on to the stack
    pub fn enter(&mut self, module: ModuleAddr, func: Option<FuncAddr>, locals: Vec<Value>) {
        self.0.push(StackFrame::new(Some(module), func, locals))
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
    pub fn module(&self) -> Option<ModuleAddr> {
        self.frame().module()
    }

    /// Returns a boolean indicating if the current frame's stack is empty
    pub fn frame_empty(&self) -> bool {
        self.frame().is_empty()
    }

    /// Gets the value of the local with the specified ID, if any
    pub fn local(&self, idx: usize) -> Option<Value> {
        self.frame().local(idx)
    }
}
