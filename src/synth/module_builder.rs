use crate::{
    interp::Thread,
    module::{Export, FuncType, MemoryType},
    synth::SyntheticFunc,
    Value,
};

pub struct ModuleBuilder {
    pub name: String,
    pub funcs: Vec<SyntheticFunc>,
    pub exports: Vec<Export>,
}

impl ModuleBuilder {
    pub fn new<S: Into<String>>(name: S) -> ModuleBuilder {
        ModuleBuilder {
            name: name.into(),
            funcs: Vec::new(),
            exports: Vec::new(),
        }
    }

    pub fn func<S: Into<String>>(
        mut self,
        name: S,
        typ: FuncType,
        imp: fn(&mut Thread) -> Value,
    ) -> Self {
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
