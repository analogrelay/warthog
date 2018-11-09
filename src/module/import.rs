use std::{fmt, io};

use crate::{module::MemberDesc, utils, Error};

#[derive(PartialEq, Clone)]
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

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &MemberDesc {
        &self.description
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

impl fmt::Debug for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
