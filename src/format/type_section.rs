use std::io;

use error::Error;
use utils;

use super::FuncType;
use super::Section;

pub struct TypeSection {
    types: Vec<FuncType>,
}

impl Section for TypeSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<TypeSection, Error> {
        let types: Vec<FuncType> = utils::read_vec(reader, |r| FuncType::read(r))?;

        Ok(TypeSection { types })
    }
}

impl TypeSection {
    pub fn types(&self) -> &[FuncType] {
        &self.types
    }
}
