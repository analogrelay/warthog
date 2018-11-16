use crate::{
    module::ModuleNames,
    runtime::{ExportInst, FuncAddr, MemAddr},
};

addr_type!(ModuleAddr);

pub struct ModuleInst {
    name: String,
    funcs: Vec<FuncAddr>,
    mems: Vec<MemAddr>,
    exports: Vec<ExportInst>,
    names: Option<ModuleNames>,
}

impl ModuleInst {
    pub fn new<S: Into<String>>(
        name: S,
        funcs: Vec<FuncAddr>,
        mems: Vec<MemAddr>,
        exports: Vec<ExportInst>,
        names: Option<ModuleNames>,
    ) -> ModuleInst {
        ModuleInst {
            name: name.into(),
            funcs,
            mems,
            exports,
            names,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn funcs(&self) -> &[FuncAddr] {
        &self.funcs
    }

    pub fn mems(&self) -> &[MemAddr] {
        &self.mems
    }

    pub fn exports(&self) -> &[ExportInst] {
        &self.exports
    }

    pub fn names(&self) -> Option<&ModuleNames> {
        self.names.as_ref()
    }

    pub fn get_mem(&self, mem_idx: usize) -> MemAddr {
        self.mems[mem_idx]
    }

    pub fn get_func(&self, func_idx: usize) -> FuncAddr {
        self.funcs[func_idx]
    }

    pub fn find_export(&self, name: &str) -> Option<&ExportInst> {
        self.exports.iter().find(|e| e.name() == name)
    }
}
