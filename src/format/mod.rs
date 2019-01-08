//! Module containing types that describe the raw WebAssembly format
//! 
//! Types in this module are designed to correspond directly to concepts described
//! in [the WebAssembly specification](https://webassembly.github.io/spec/core/)

mod data_item;
mod export;
mod expr;
mod func_body;
mod import;
mod member_desc;
mod module;
mod module_names;
mod instruction;

pub mod reader;

pub use self::data_item::DataItem;
pub use self::export::Export;
pub use self::expr::Expr;
pub use self::func_body::FuncBody;
pub use self::import::Import;
pub use self::member_desc::MemberDesc;
pub use self::module::Module;
pub use self::module_names::ModuleNames;
pub use self::instruction::Instruction;