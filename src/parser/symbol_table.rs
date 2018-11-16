use std::collections::HashMap;

pub struct SymbolTable(HashMap<String, usize>);

lazy_static! {
    static ref EMPTY_TABLE: SymbolTable = SymbolTable::new();
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable(HashMap::new())
    }

    pub fn empty() -> &'static SymbolTable {
        &EMPTY_TABLE
    }

    pub fn assign<S: Into<String>>(&mut self, name: S) -> usize {
        let next_id = self.0.len();
        self.0.insert(name.into(), next_id);
        next_id
    }

    pub fn get(&self, name: &str) -> Option<usize> {
        self.0.get(name).cloned()
    }
}
