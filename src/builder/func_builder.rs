use crate::{builder::TypeUse, module::Instruction, ValType};

pub struct FuncBuilder {
    pub export: Option<String>,
    pub import: Option<(String, String)>,
    pub type_use: TypeUse,
    pub locals: Vec<ValType>,
    pub body: Vec<Instruction>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder {
            export: None,
            import: None,
            type_use: TypeUse::new(),
            locals: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn import_from<S: Into<String>, T: Into<String>>(mut self, module: S, func: T) -> Self {
        self.import = Some((module.into(), func.into()));
        self
    }

    pub fn export_as<S: Into<String>>(mut self, name: S) -> Self {
        self.export = Some(name.into());
        self
    }

    pub fn type_id(mut self, type_id: usize) -> Self {
        self.type_use.id = Some(type_id);
        self
    }

    pub fn param(mut self, param: ValType) -> Self {
        self.type_use.params.push(param);
        self
    }

    pub fn result(mut self, result: ValType) -> Self {
        self.type_use.results.push(result);
        self
    }

    pub fn locals(mut self, mut locals: Vec<ValType>) -> Self {
        self.locals.append(&mut locals);
        self
    }

    pub fn body(mut self, mut inst: Vec<Instruction>) -> Self {
        self.body.append(&mut inst);
        self
    }
}
