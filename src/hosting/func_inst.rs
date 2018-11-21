use std::sync::Arc;

use crate::{
    hosting::{ExternalFunc, ModuleAddr},
    module::{FuncBody, FuncType},
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

    pub fn external(typ: FuncType, module: ModuleAddr, func: Arc<ExternalFunc>) -> FuncInst {
        FuncInst {
            typ,
            module,
            imp: FuncImpl::External(func),
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
    External(Arc<ExternalFunc>),
}
