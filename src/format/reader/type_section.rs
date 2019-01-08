use std::io;

use crate::{format::reader::Section, types::FuncType, utils, Error};

pub struct TypeSection {
    pub types: Vec<FuncType>,
}

impl Section for TypeSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<TypeSection, Error> {
        let types: Vec<FuncType> = utils::read_vec(reader, |r| FuncType::read(r))?;

        Ok(TypeSection { types })
    }
}
