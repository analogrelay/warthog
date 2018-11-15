use crate::{
    builder::FuncBuilder,
    module::{DataItem, Export, FuncBody, FuncType, Import, Module, ModuleNames},
};

pub struct ModuleBuilder {
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub funcs: Vec<usize>,
    pub exports: Vec<Export>,
    pub code: Vec<FuncBody>,
    pub data: Vec<DataItem>,
    pub names: Option<ModuleNames>,
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
            names: None,
        }
    }

    pub fn add_func(&mut self, func: FuncBuilder) {
        // Check if we need to add a type and get the type id
        let type_id = match func.type_id {
            Some(id) => id,
            None => {
                // Create a functype
                let typ = FuncType::new(func.params, func.results);

                // Check if a matching one already exists
                match self.types.iter().position(|f| f == &typ) {
                    Some(id) => id,
                    None => {
                        // If not, add one
                        let id = self.types.len();
                        self.types.push(typ);
                        id
                    }
                }
            }
        };

        // Add the func to the list
        let func_id = self.funcs.len();
        self.funcs.push(type_id);

        // Add the body
        debug_assert_eq!(func_id, self.code.len());
        let body = FuncBody::new(func.locals, func.body);
        self.code.push(body);

        // Export, if the builder is marked as exported
        if let Some(export) = func.export {
            self.exports.push(Export::func(export, func_id));
        }
    }

    /// Adds a function to the builder (chaining variant)
    pub fn func(mut self, func: FuncBuilder) -> Self {
        self.add_func(func);
        self
    }

    pub fn build(self) -> Module {
        Module::from_builder(self)
    }
}
