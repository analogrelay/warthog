use crate::{
    module::{DataItem, Export, FuncBody, FuncType, Import, MemoryType, Module},
    synth::{HostFunc, SyntheticFunc},
};

pub struct ModuleBuilder {
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub funcs: Vec<u32>,
    pub exports: Vec<Export>,
    pub code: Vec<FuncBody>,
    pub data: Vec<DataItem>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            types: Vec::new(),
            imports: Vec::new(),
            funcs: Vec::new(),
            exports: Vec::new(),
            code: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn build(self) -> Module {
        Module::from_builder(self)
    }
}
