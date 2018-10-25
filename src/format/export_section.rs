use std::fmt;
use std::io;

use utils;
use error::Error;

use super::{Section, MemberDesc};

pub struct ExportSection {
    exports: Vec<Export>,
}

impl Section for ExportSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<ExportSection, Error> {
        let exports = utils::read_vec(reader, |r| {
            Export::read(r)
        })?;

        Ok(ExportSection {
            exports
        })
    }
}

impl ExportSection {
    pub fn exports(&self) -> &[Export] {
        &self.exports
    }
}

pub struct Export {
    name: String,
    description: MemberDesc,
}

impl Export {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Export, Error> {
        let name = utils::read_name(reader)?;
        let description = MemberDesc::read(reader)?;
        Ok(Export { name, description })
    }
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(export \"{}\" {})",
            self.name, self.description
        )
    }
}