use std::{fmt, io};

use crate::{module::MemberDesc, utils, Error};

#[derive(Clone)]
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
        write!(f, "(export \"{}\" {})", self.name, self.description)
    }
}
