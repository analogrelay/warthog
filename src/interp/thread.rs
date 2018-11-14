use std::borrow::Cow;

use crate::{
    interp::{exec, Stack, Trap},
    module::{Expr, Instruction, ValType},
    runtime::{FuncAddr, FuncImpl, Host, ModuleAddr},
    Value,
};

pub struct Thread {
    stack: Stack,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            stack: Stack::new(),
        }
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
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
        self.stack.enter(module, Vec::new());

        // Evaluate the expression
        // Don't use '?' here because we need to clear the stack frame first
        // (i.e. the "finally" of the pseudo "try-catch-finally" block)
        let val = match self.run(host, expr.instructions()) {
            Ok(()) => self.pop(),
            Err(e) => {
                self.stack.exit();
                return Err(e);
            }
        };

        let result = if !self.stack.frame_empty() {
            Err(self.throw("Stack is not empty at end of function invocation!"))
        } else {
            val
        };

        self.stack.exit();
        result
    }

    /// Runs the function specified by [`func`] in the context of this thread.
    pub fn invoke(&mut self, host: &mut Host, func: FuncAddr) -> Result<Vec<Value>, Trap> {
        // Resolve the function
        let func_inst = host.get_func(func);
        match func_inst.imp() {
            FuncImpl::Synthetic(synth_fn) => synth_fn.invoke(host, self),
            FuncImpl::Local {
                typ, module, code, ..
            } => {
                // Pop parameters
                let mut locals = Vec::with_capacity(typ.params().len() + code.locals().len());
                for param in typ.params() {
                    if let Some(val) = self.stack.pop() {
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

                self.stack.enter(module.clone(), locals);
                if let Err(e) = self.run(host, code.body()) {
                    self.stack.exit();
                    return Err(e);
                }

                // Pop the result
                // In WASM v1, there is only zero or one result.
                let mut results = Vec::with_capacity(typ.results().len());
                for result in typ.results() {
                    if let Some(val) = self.stack.pop() {
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
                let result = if !self.stack.frame_empty() {
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

    /// Creates a new [`Trap`], capturing the current stack frame.
    pub fn throw<S: Into<Cow<'static, str>>>(&self, message: S) -> Trap {
        // TODO: Capture stack frame ;)
        Trap::new(message)
    }

    /// Tries to pop a value off the stack for the current frame, traps if there is no current value.
    pub fn pop(&mut self) -> Result<Value, Trap> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(self.throw("Stack underflow!")),
        }
    }

    pub fn push(&mut self, v: Value) {
        self.stack.push(v)
    }

    fn execute(&mut self, host: &mut Host, inst: Instruction) -> Result<(), Trap> {
        exec::execute(self, host, inst)
    }
}
