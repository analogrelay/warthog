use crate::{
    module::{Export, FuncType, MemoryType},
    synth::{HostFunc, SyntheticFunc},
};

pub struct SyntheticModule {
    pub funcs: Vec<SyntheticFunc>,
    pub exports: Vec<Export>,
}

impl SyntheticModule {
    pub fn new() -> SyntheticModule {
        SyntheticModule {
            funcs: Vec::new(),
            exports: Vec::new(),
        }
    }

    pub fn func<S: Into<String>>(mut self, name: S, typ: FuncType, imp: HostFunc) -> Self {
        let idx = self.funcs.len();
        self.funcs.push(SyntheticFunc::new(typ, imp));
        self.exports.push(Export::func(name, idx as u32));
        self
    }

    pub fn mem<S: Into<String>>(mut self, name: S, min_size: u32, max_size: Option<u32>) -> Self {
        self.exports
            .push(Export::mem(name, MemoryType::new(min_size, max_size)));
        self
    }
}
