use crate::{builder::FuncBuilder, module::Module};

pub struct ModuleBuilder {
    funcs: Vec<FuncBuilder>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder { funcs: Vec::new() }
    }

    pub fn func(&mut self, func: FuncBuilder) {
        self.funcs.push(func);
    }

    pub fn build(self) -> Module {
        Module::from_builder(self)
    }
}
