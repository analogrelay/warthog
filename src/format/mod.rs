mod instruction;
mod member_desc;
mod section_header;
mod types;

mod code_section;
mod data_section;
mod export_section;
mod function_section;
mod import_section;
mod type_section;

use std::io;

use error::Error;

pub use self::code_section::CodeSection;
pub use self::data_section::DataSection;
pub use self::export_section::{Export, ExportSection};
pub use self::function_section::FunctionSection;
pub use self::import_section::{Import, ImportSection};
pub use self::instruction::Instruction;
pub use self::member_desc::MemberDesc;
pub use self::section_header::{SectionHeader, SectionId};
pub use self::type_section::TypeSection;
pub use self::types::{FuncType, GlobalType, MemoryType, TableType, ValType};

pub trait Section: Sized {
    fn read<R: io::Read>(reader: &mut R) -> Result<Self, Error>;
}
