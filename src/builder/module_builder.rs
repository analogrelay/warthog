use crate::{
    builder::{FuncBuilder, TypeUse},
    format::{DataItem, Export, FuncBody, FuncType, Import, MemberDesc, Module, ModuleNames},
};

pub struct ModuleBuilder {
    pub version: u32,
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
            version: 1,
            types: Vec::new(),
            imports: Vec::new(),
            funcs: Vec::new(),
            exports: Vec::new(),
            code: Vec::new(),
            data: Vec::new(),
            names: None,
        }
    }

    pub fn add_type(&mut self, type_use: TypeUse) -> usize {
        // Check if we need to add a type and get the type id
        match type_use.id {
            Some(id) => id,
            None => {
                // Create a functype
                let typ = FuncType::new(type_use.params, type_use.results);

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
        }
    }

    pub fn add_func(&mut self, func: FuncBuilder) -> usize {
        let type_id = self.add_type(func.type_use);

        // Add the func to the list
        if let Some((module, name)) = func.import {
            assert_eq!(
                0,
                self.funcs.len(),
                "Cannot add imports after local functions are defined!"
            );
            let func_id = self.imports.len();
            self.imports
                .push(Import::new(module, name, MemberDesc::Function(type_id)));
            func_id
        } else {
            let func_id = self.imports.len() + self.funcs.len();
            self.funcs.push(type_id);

            // Add the body
            debug_assert_eq!(func_id - self.imports.len(), self.code.len());
            let body = FuncBody::new(func.locals, func.body);
            self.code.push(body);

            // Export, if the builder is marked as exported
            if let Some(export) = func.export {
                self.exports.push(Export::func(export, func_id));
            }

            func_id
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
