use crate::module::FuncType;

pub struct SyntheticFunc {
    pub typ: FuncType,
    pub imp: fn() -> (),
}

impl SyntheticFunc {
    pub fn new(typ: FuncType, imp: fn() -> ()) -> SyntheticFunc {
        SyntheticFunc { typ, imp }
    }

    pub fn invoke(&self) -> () {
        (self.imp)()
    }
}
