use std::io;

use crate::{module::FuncBody, reader::Section, utils, Error};

pub struct CodeSection {
    pub code: Vec<FuncBody>,
}

impl Section for CodeSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<CodeSection, Error> {
        let code = utils::read_vec(reader, |r| FuncBody::read(r))?;

        Ok(CodeSection { code })
    }
}
