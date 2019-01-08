use crate::types::ValType;

pub struct TypeUse {
    pub id: Option<usize>,
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl TypeUse {
    pub fn new() -> TypeUse {
        TypeUse {
            id: None,
            params: Vec::new(),
            results: Vec::new(),
        }
    }
}
