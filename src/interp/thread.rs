use crate::{
    interp::{Stack, StackItem},
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

    /// Runs the function specified by the [`FuncAddr`] in the context of this thread.
    pub fn invoke(&mut self, func: FuncAddr) {
        // Resolve the function
        let func_inst = self.host.get_func(func);
        match func_inst.imp() {
            FuncImpl::Synthetic(synth_fn) => synth_fn.invoke(),
            FuncImpl::Local {
                module: module_addr,
                code: code,
                ..
            } => {
                // TODO: Initialize locals
                self.stack
                    .push(StackItem::Activation(*module_addr, Vec::new()));
                self.execute(code.body.as_slice());
            }
        };
    }

    fn execute(&mut self, _code: &[Instruction]) {
        unimplemented!()
    }
}
