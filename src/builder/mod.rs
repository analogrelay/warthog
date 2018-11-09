mod func_builder;
mod module_builder;
mod builder_error;

pub use self::func_builder::FuncBuilder;
pub use self::module_builder::ModuleBuilder;
pub use self::builder_error::BuilderError;

use crate::module::FuncType;

pub enum TypeUse {
    Ref(usize),
    Inline(FuncType)
}
