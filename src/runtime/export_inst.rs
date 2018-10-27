use std::fmt;

use crate::runtime::{FuncAddr, MemAddr};

pub struct ExportInst {
    name: String,
    value: ExternVal,
}

impl ExportInst {
    pub fn func<S: Into<String>>(name: S, addr: FuncAddr) -> ExportInst {
        ExportInst {
            name: name.into(),
            value: ExternVal::Func(addr),
        }
    }

    pub fn mem<S: Into<String>>(name: S, addr: MemAddr) -> ExportInst {
        ExportInst {
            name: name.into(),
            value: ExternVal::Mem(addr),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ExternVal {
        &self.value
    }
}

pub enum ExternVal {
    Func(FuncAddr),
    Mem(MemAddr),
}

impl fmt::Debug for ExternVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExternVal::Func(a) => write!(f, "{}", a),
            ExternVal::Mem(a) => write!(f, "{}", a),
        }
    }
}
