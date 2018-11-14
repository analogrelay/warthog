use crate::{
    module::{FuncBody, FuncType},
    runtime::ModuleAddr,
    synth::SyntheticFunc,
};

addr_type!(FuncAddr);

pub struct FuncInst {
    imp: FuncImpl,
}

impl FuncInst {
    pub fn local(typ: FuncType, module: ModuleAddr, func_id: usize, code: FuncBody) -> FuncInst {
        FuncInst {
            imp: FuncImpl::Local { typ, module, code, func_id },
        }
    }

    pub fn synthetic(func: SyntheticFunc) -> FuncInst {
        FuncInst {
            imp: FuncImpl::Synthetic(func),
        }
    }

    pub fn typ(&self) -> &FuncType {
        match self.imp {
            FuncImpl::Local { typ: ref t, .. } => t,
            FuncImpl::Synthetic(ref f) => &f.typ,
        }
    }

    pub fn imp(&self) -> &FuncImpl {
        &self.imp
    }
}

pub enum FuncImpl {
    Local {
        typ: FuncType,
        module: ModuleAddr,
        code: FuncBody,
        func_id: usize,
    },
    Synthetic(SyntheticFunc),
}
