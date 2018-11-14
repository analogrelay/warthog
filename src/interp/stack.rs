use crate::{
    runtime::{FuncAddr, ModuleAddr},
    Value,
};

#[derive(Clone, PartialEq)]
pub struct StackFrame {
    module: Option<ModuleAddr>,
    func: Option<FuncAddr>,
}

impl StackFrame {
    pub fn new(module: Option<ModuleAddr>, func: Option<FuncAddr>) -> StackFrame {
        StackFrame { module, func }
    }

    pub fn module(&self) -> Option<ModuleAddr> {
        self.module
    }

    pub fn func(&self) -> Option<FuncAddr> {
        self.func
    }
}

/// Represents the context under which a function executes.
/// 
/// The execution context contains the following items:
/// * The operand stack for the invocation.
/// * The values of the locals currently in scope.
/// * A [`StackFrame`] representing the current location in the program.
pub struct ExecutionContext {
    values: Vec<Value>,
    locals: Vec<Value>,
    frame: StackFrame,
}

impl ExecutionContext {
    /// Creates a new execution context with the specified [`StackFrame`] and a list of local values.
    pub fn new(frame: StackFrame, locals: Vec<Value>) -> ExecutionContext {
        ExecutionContext {
            values: Vec::new(),
            frame,
            locals,
        }
    }

    /// Gets the [`StackFrame`] associated with this execution context.
    pub fn frame(&self) -> &StackFrame {
        &self.frame
    }

    /// Pushes a new value on to the operand stack for this execution context.
    pub fn push(&mut self, value: Value) {
        // Don't push nils, just drop them.
        if value != Value::Nil {
            self.values.push(value)
        }
    }

    /// Pops a new value off the operand stack for this execution context.
    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }

    /// Gets a boolean indicating if the operand stack for this execution context is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Gets the value of the local with the specified index.
    pub fn local(&self, idx: usize) -> Option<Value> {
        if idx < self.locals.len() {
            Some(self.locals[idx])
        } else {
            None
        }
    }
}

pub struct ExecutionStack(Vec<ExecutionContext>);

impl ExecutionStack {
    pub fn new() -> ExecutionStack {
        ExecutionStack(vec![ExecutionContext::new(StackFrame::new(None, None), Vec::new())])
    }

    /// Gets a reference to the active [`ExecutionContext`]
    pub fn current(&self) -> &ExecutionContext {
        // We can unwrap because we ensure there is always at least one item
        self.0.last().unwrap()
    }

    /// Gets a mutable reference to the active [`ExecutionContext`].
    pub fn current_mut(&mut self) -> &mut ExecutionContext {
        // We can unwrap because we ensure there is always at least one item
        self.0.last_mut().unwrap()
    }

    /// Pushes a new [`ExecutionContext`] on to the stack
    pub fn enter(&mut self, module: ModuleAddr, func: Option<FuncAddr>, locals: Vec<Value>) {
        self.0.push(ExecutionContext::new(StackFrame::new(Some(module), func), locals))
    }

    /// Pops the current [`ExecutionContext`] (and all values associated with it) off the stack
    pub fn exit(&mut self) {
        if self.0.len() == 1 {
            panic!("There is no current frame to exit!");
        } else {
            self.0.pop();
        }
    }
}
