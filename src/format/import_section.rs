use std::fmt;
use std::io;

use error::Error;
use utils;

use super::{MemberDesc, Section};

pub struct ImportSection {
    imports: Vec<Import>,
}

impl Section for ImportSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<ImportSection, Error> {
        let imports = utils::read_vec(reader, |r| Import::read(r))?;

        Ok(ImportSection { imports })
    }
}

impl ImportSection {
    pub fn imports(&self) -> &[Import] {
        &self.imports
    }
}

pub struct Import {
    module: String,
    name: String,
    description: MemberDesc,
}

impl Import {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Import, Error> {
        let module = utils::read_name(reader)?;
        let name = utils::read_name(reader)?;
        let description = MemberDesc::read(reader)?;
        Ok(Import {
            module,
            name,
            description,
        })
    }
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(import \"{}\" \"{}\" {})",
            self.module, self.name, self.description
        )
    }
}
