use crate::{
    builder::{BuilderError, FuncBuilder, TypeUse},
    module::{DataItem, Export, FuncBody, FuncType, Import, Module},
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

    pub fn func(mut self, mut func: FuncBuilder) -> Result<Self, BuilderError> {
        // Check if we need to add a type and get the type id
        let typ = func.typ.take().ok_or(BuilderError::IncompleteFunction(
            "Function is missing a type.".into(),
        ))?;
        let type_id = match typ {
            TypeUse::Inline(typ) => {
                let id = self.types.len();
                self.types.push(typ);
                id as u32
            }
            TypeUse::Ref(x) => x as u32,
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

        Ok(self)
    }

    pub fn build(mut self) -> Module {
        Module::from_builder(self)
    }
}
