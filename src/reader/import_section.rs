use std::io;

use crate::{module::Import, reader::Section, utils, Error};

pub struct ImportSection {
    pub imports: Vec<Import>,
}

impl Section for ImportSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<ImportSection, Error> {
        let imports = utils::read_vec(reader, |r| Import::read(r))?;

        Ok(ImportSection { imports })
    }
}
