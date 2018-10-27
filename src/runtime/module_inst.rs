use crate::runtime::{ExportInst, FuncAddr};

addr_type!(ModuleAddr);

pub struct ModuleInst {
    name: String,
    funcs: Vec<FuncAddr>,
    exports: Vec<ExportInst>,
}

impl ModuleInst {
    pub fn new<S: Into<String>>(
        name: S,
        funcs: Vec<FuncAddr>,
        mems: Vec<MemAddr>,
        exports: Vec<ExportInst>,
    ) -> ModuleInst {
        ModuleInst {
            name: name.into(),
            funcs,
            mems,
            exports,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn funcs(&self) -> &[FuncAddr] {
        &self.funcs
    }

    pub fn exports(&self) -> &[ExportInst] {
        &self.exports
    }

    pub fn find_export(&self, name: &str) -> Option<&ExportInst> {
        self.exports.iter().find(|e| e.name() == name)
    }
}
