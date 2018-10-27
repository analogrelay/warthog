use crate::runtime::FuncAddr;

addr_type!(InstanceAddr);

pub struct Instance {
    funcs: Vec<FuncAddr>,
}

impl Instance {
    pub fn new(funcs: Vec<FuncAddr>) -> Instance {
        Instance {
            funcs
        }
    }

    pub fn funcs(&self) -> &[FuncAddr] {
        &self.funcs
    }
}
