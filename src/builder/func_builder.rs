use crate::{
    builder::TypeUse,
    module::{Instruction, ValType},
};

pub struct FuncBuilder {
    pub export: Option<String>,
    pub typ: Option<TypeUse>,
    pub locals: Vec<ValType>,
    pub body: Vec<Instruction>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder {
            export: None,
            typ: None,
            locals: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn export_as<S: Into<String>>(mut self, name: S) -> Self {
        self.export = Some(name.into());
        self
    }

    pub fn typ(mut self, typ: TypeUse) -> Self {
        self.typ = Some(typ);
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
