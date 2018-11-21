use std::{fmt, io};

use crate::{
    module::{MemberDesc, MemoryType},
    utils, Error,
};

#[derive(PartialEq, Clone)]
pub struct Export {
    name: String,
    description: MemberDesc,
}

impl Export {
    pub fn func<S: Into<String>>(name: S, idx: usize) -> Export {
        Export::new(name, MemberDesc::Function(idx))
    }

    pub fn mem<S: Into<String>>(name: S, typ: MemoryType) -> Export {
        Export::new(name, MemberDesc::Memory(typ))
    }

    pub fn new<S: Into<String>>(name: S, description: MemberDesc) -> Export {
        Export {
            name: name.into(),
            description,
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<Export, Error> {
        let name = utils::read_name(reader)?;
        let description = MemberDesc::read(reader)?;
        Ok(Export { name, description })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &MemberDesc {
        &self.description
    }
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(export \"{}\" {})", self.name, self.description)
    }
}

impl fmt::Debug for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
