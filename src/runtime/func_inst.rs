use crate::{
    module::{FuncBody, FuncType},
    runtime::InstanceAddr,
};

addr_type!(FuncAddr);

pub struct FuncInst {
    typ: FuncType,
    imp: FuncImpl,
}

enum FuncImpl {
    Local { module: InstanceAddr, code: FuncBody },
    // TODO: Host
}

impl FuncInst {
    pub fn local(typ: FuncType, module: InstanceAddr, code: FuncBody) -> FuncInst {
        FuncInst {
            typ,
            imp: FuncImpl::Local{ module, code }
        }
    }

    pub fn typ(&self) -> &FuncType {
        &self.typ
    }

    pub fn is_local(&self) -> bool {
        match self.imp {
            FuncImpl::Local { .. } => true,
            _ => false
        }
    }

    pub fn module(&self) -> Option<InstanceAddr> {
        match self.imp {
            FuncImpl::Local { module: m, .. } => Some(m),
            _ => None
        }
    }

    pub fn code(&self) -> Option<&FuncBody> {
        match self.imp {
            FuncImpl::Local { code: ref c, .. } => Some(c),
            _ => None
        }
    }
}
