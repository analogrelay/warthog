use crate::module::{Instruction, ValType};

pub struct FuncBuilder {
    pub export: Option<String>,
    pub type_id: Option<usize>,
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
    pub locals: Vec<ValType>,
    pub body: Vec<Instruction>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder {
            export: None,
            type_id: None,
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn export_as<S: Into<String>>(mut self, name: S) -> Self {
        self.export = Some(name.into());
        self
    }

    pub fn type_id(mut self, type_id: usize) -> Self {
        self.type_id = Some(type_id);
        self
    }

    pub fn param(mut self, param: ValType) -> Self {
        self.params.push(param);
        self
    }

    pub fn result(mut self, result: ValType) -> Self {
        self.results.push(result);
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
