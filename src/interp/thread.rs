use crate::{
    interp::{InvokeResult, Stack},
    module::{Instruction, ValType},
    runtime::{FuncAddr, FuncImpl, Host},
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

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    /// Runs the function specified by the [`FuncAddr`] in the context of this thread.
    pub fn invoke(&mut self, host: &mut Host, func: FuncAddr) -> InvokeResult {
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
                            return InvokeResult::trap(format!(
                                "Type mismatch. Expected: {}, Actual: {}",
                                param,
                                val.typ()
                            ));
                        }
                        locals.push(val);
                    } else {
                        return InvokeResult::trap("Stack underflow!");
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
                    return e;
                }

                // Pop the result
                // In WASM v1, there is only zero or one result.
                let mut results = Vec::with_capacity(typ.results().len());
                for result in typ.results() {
                    if let Some(val) = self.stack.pop() {
                        if val.typ() != *result {
                            return InvokeResult::trap(format!(
                                "Type mismatch. Expected: {}, Actual: {}",
                                result,
                                val.typ()
                            ));
                        }
                        results.push(val);
                    } else {
                        return InvokeResult::trap("Stack underflow!");
                    }
                }

                // Validate that the stack is empty
                let result = if !self.stack.frame_empty() {
                    InvokeResult::trap("Stack is not empty at end of function invocation!")
                } else {
                    InvokeResult::Return(results)
                };

                // Exit the stack frame
                self.stack.exit();

                result
            }
        }
    }

    pub fn run(&mut self, host: &mut Host, code: &[Instruction]) -> Result<(), InvokeResult> {
        for inst in code {
            self.execute(host, inst)?;
        }
        Ok(())
    }

    fn execute(&mut self, host: &mut Host, inst: &Instruction) -> Result<(), InvokeResult> {
        match inst {
            Instruction::Const(val) => self.stack.push(val.clone()),
            Instruction::Call(func_idx) => {
                let func = host.resolve_func(self.stack.module(), *func_idx);
                let results = self.invoke(host, func);
                panic!("Call instruction needs to handle return values");
            }
            Instruction::GetLocal(local_idx) => {
                let val = match self.stack.local(*local_idx) {
                    Some(l) => l,
                    None => return Err(InvokeResult::trap(format!("No such local: {}", local_idx))),
                };
                self.stack.push(val);
            }
            Instruction::Add(ValType::Integer32) => {
                let x = self.pop()?;
                let y = self.pop()?;

                let res = match (x, y) {
                    (Value::Integer32(x), Value::Integer32(y)) => Value::Integer32(x + y),
                    _ => {
                        return Err(InvokeResult::trap(format!(
                            "Type mismatch. Unable to add {} and {} using i32.add",
                            x.typ(),
                            y.typ()
                        )))
                    }
                };
                self.stack.push(res);
            }
            x => {
                return Err(InvokeResult::trap(format!(
                    "Instruction not implemented: {}",
                    x
                )))
            }
        };

        Ok(())
    }

    fn pop(&mut self) -> Result<Value, InvokeResult> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(InvokeResult::trap("Stack underflow!")),
        }
    }
}
