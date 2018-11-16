use crate::{
    module::{FuncBody, FuncType},
    runtime::ModuleAddr,
    synth::SyntheticFunc,
};

addr_type!(FuncAddr);

pub struct FuncInst {
    typ: FuncType,
    module: ModuleAddr,
    imp: FuncImpl,
}

impl FuncInst {
    pub fn local(typ: FuncType, module: ModuleAddr, func_id: usize, code: FuncBody) -> FuncInst {
        FuncInst {
            typ,
            module,
            imp: FuncImpl::Local(code, func_id),
        }
    }

    pub fn synthetic(typ: FuncType, module: ModuleAddr, func: SyntheticFunc) -> FuncInst {
        FuncInst {
            typ,
            module,
            imp: FuncImpl::Synthetic(func),
        }
    }

    pub fn typ(&self) -> &FuncType {
        &self.typ
    }

    pub fn module(&self) -> ModuleAddr {
        self.module
    }

    pub fn imp(&self) -> &FuncImpl {
        &self.imp
    }
}

pub enum FuncImpl {
    Local(FuncBody, usize),
    Synthetic(SyntheticFunc),
}
