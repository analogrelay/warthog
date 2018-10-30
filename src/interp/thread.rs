use crate::{
    interp::Stack,
    module::Instruction,
    runtime::{FuncAddr, FuncImpl, Host},
};

pub struct Thread {
    host: Host,
    stack: Stack,
}

impl Thread {
    pub fn new(host: Host) -> Thread {
        Thread {
            host,
            stack: Stack::new(),
        }
    }

    pub fn host_mut(&mut self) -> &mut Host {
        &mut self.host
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    /// Runs the function specified by the [`FuncAddr`] in the context of this thread.
    pub fn invoke(&mut self, func: FuncAddr) {
        // Resolve the function
        let func_inst = self.host.get_func(func);
        match func_inst.imp() {
            FuncImpl::Synthetic(synth_fn) => {
                let result = synth_fn.invoke(self);
                self.stack.push(result);
            }
            FuncImpl::Local { module, code, .. } => {
                // TODO: Initialize locals
                self.stack.enter(module.clone(), Vec::new());
                self.run(code.body.as_slice());
            }
        };
    }

    fn run(&mut self, code: &[Instruction]) {
        for inst in code {
            self.execute(inst);
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Const(val) => self.stack.push(val.clone()),
            Instruction::Call(func_idx) => {
                let func = self.host.resolve_func(self.stack.module(), *func_idx);
                self.invoke(func);
            }
        }
    }
}
