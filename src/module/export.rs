use std::{fmt, io};

use crate::{
    module::{MemberDesc, MemoryType},
    utils, Error,
};

#[derive(PartialEq, Clone)]
pub struct Export {
    pub name: String,
    pub description: MemberDesc,
}

impl Export {
    pub fn func<S: Into<String>>(name: S, idx: u32) -> Export {
        Export {
            name: name.into(),
            description: MemberDesc::Function(idx),
        }
    }

    pub fn mem<S: Into<String>>(name: S, typ: MemoryType) -> Export {
        Export {
            name: name.into(),
            description: MemberDesc::Memory(typ),
        }
    }

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

impl fmt::Debug for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
