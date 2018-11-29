use crate::{
    hosting::{FuncAddr, FuncImpl, Host, ModuleAddr},
    interp::{exec, ExecutionStack},
    module::{Expr, Instruction, ValType},
    Trap, Value,
};

pub struct Thread {
    stack: ExecutionStack,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            stack: ExecutionStack::new(),
        }
    }

    pub fn stack(&self) -> &ExecutionStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut ExecutionStack {
        &mut self.stack
    }

    /// Evaluates the expression specified by [`expr`] in the context of the provided module
    pub fn eval(
        &mut self,
        module: ModuleAddr,
        expr: &Expr,
        host: &mut Host,
    ) -> Result<Value, Trap> {
        // Push a stack frame
        self.stack.enter(module, None, Vec::new());

        // Evaluate the expression
        let val = match self.run(host, expr.instructions()) {
            Ok(()) => self.pop()?,
            Err(e) => {
                self.stack.exit();
                return Err(e);
            }
        };

        let result = if !self.stack.current().is_empty() {
            Err(self.throw("Stack is not empty at end of function invocation!"))
        } else {
            Ok(val)
        };

        self.stack.exit();
        result
    }

    /// Calls the specified function, providing the values represented by [`values`] as parameters
    ///
    /// This method enters a new stack frame, pushes [`values`] to the stack, then invokes
    /// the requested function. Because this enters a new stack frame before evaluating the expressions,
    /// the stack will have **two** new frames by the time the function code actually runs
    pub fn call(
        &mut self,
        host: &mut Host,
        module: ModuleAddr,
        func: FuncAddr,
        mut values: Vec<Value>,
    ) -> Result<Vec<Value>, Trap> {
        self.stack_mut().enter(module, None, Vec::new());

        // Push the values on to the stack
        for value in values.drain(..).rev() {
            self.push(value);
        }

        let res = self.invoke(host, func);

        self.stack_mut().exit();

        res
    }

    /// Runs the function specified by [`func`] in the context of this thread.
    pub fn invoke(&mut self, host: &mut Host, func: FuncAddr) -> Result<Vec<Value>, Trap> {
        // Resolve the function
        let func_inst = host.get_func(func);
        match func_inst.imp() {
            FuncImpl::External(synth_fn) => synth_fn.invoke(host, self).map_err(|e| self.throw(e)),
            FuncImpl::Local(code, _) => {
                // Pop parameters
                let mut locals =
                    Vec::with_capacity(func_inst.typ().params().len() + code.locals().len());
                for param in func_inst.typ().params() {
                    if let Some(val) = self.stack.current_mut().pop() {
                        if val.typ() != *param {
                            return Err(self.throw(format!(
                                "Type mismatch. Expected: {}, Actual: {}",
                                param,
                                val.typ()
                            )));
                        }
                        locals.push(val);
                    } else {
                        return Err(self.throw("Stack underflow!"));
                    }
                }

                // Initialize locals
                for local in code.locals() {
                    let v = match local {
                        ValType::Nil => unreachable!(),
                        ValType::Integer32 => Value::Integer32(0),
                        ValType::Integer64 => Value::Integer64(0),
                        ValType::Float32 => Value::Float32(0.0),
                        ValType::Float64 => Value::Float64(0.0),
                    };
                    locals.push(v);
                }

                self.stack
                    .enter(func_inst.module().clone(), Some(func), locals);
                if let Err(e) = self.run(host, code.body()) {
                    self.stack.exit();
                    return Err(e);
                }

                // Pop the result
                // In WASM v1, there is only zero or one result.
                let mut results = Vec::with_capacity(func_inst.typ().results().len());
                for result in func_inst.typ().results() {
                    if let Some(val) = self.stack.current_mut().pop() {
                        if val.typ() != *result {
                            return Err(self.throw(format!(
                                "Type mismatch. Expected: {}, Actual: {}",
                                result,
                                val.typ()
                            )));
                        }
                        results.push(val);
                    } else {
                        return Err(self.throw("Stack underflow!"));
                    }
                }

                // Validate that the stack is empty
                let result = if !self.stack.current().is_empty() {
                    Err(self.throw("Stack is not empty at end of function invocation!"))
                } else {
                    Ok(results)
                };

                // Exit the stack frame
                self.stack.exit();

                result
            }
        }
    }

    pub fn run(&mut self, host: &mut Host, code: &[Instruction]) -> Result<(), Trap> {
        for inst in code {
            self.execute(host, inst.clone())?;
        }
        Ok(())
    }

    /// Tries to pop a value off the stack for the current frame, traps if there is no current value.
    pub fn pop(&mut self) -> Result<Value, Trap> {
        match self.stack.current_mut().pop() {
            Some(v) => Ok(v),
            None => Err(self.throw("Stack underflow!")),
        }
    }

    pub fn push(&mut self, v: Value) {
        self.stack.current_mut().push(v)
    }

    fn execute(&mut self, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
        exec::execute(self, host, inst).map_err(|e| self.throw(e))
    }

    /// Creates a new [`Trap`], capturing the current stack frame.
    fn throw<T: Into<Trap>>(&self, trap: T) -> Trap {
        let mut trap = trap.into();
        trap.try_set_stack(self.stack.trace());
        trap
    }
}
