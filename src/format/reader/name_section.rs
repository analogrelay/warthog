use std::io;

use byteorder::ReadBytesExt;

use crate::{format::reader::Section, utils, Error};

pub struct NameSection {
    pub module_name: Option<String>,
    pub func_names: Vec<NameAssoc>,
    pub local_names: Vec<IndirectNameAssoc>,
}

impl Section for NameSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<NameSection, Error> {
        let mut module_name = None;
        let mut func_names = None;
        let mut local_names = None;

        while let Some(subsection_id) = read_subsection_id(reader)? {
            let size = utils::read_leb128_u32(reader)? as usize;
            match subsection_id {
                0x00 => module_name = Some(utils::read_name(reader)?),
                0x01 => func_names = Some(read_name_map(reader)?),
                0x02 => local_names = Some(read_ind_name_map(reader)?),
                _ => {
                    // Skip by dumping bytes into a buffer
                    reader.read_exact(&mut vec![0u8; size])?;
                }
            }
        }

        Ok(NameSection {
            module_name,
            func_names: func_names.unwrap_or_else(|| Vec::new()),
            local_names: local_names.unwrap_or_else(|| Vec::new()),
        })
    }
}

pub struct NameAssoc(usize, String);

impl NameAssoc {
    pub fn new<S: Into<String>>(index: usize, name: S) -> NameAssoc {
        NameAssoc(index, name.into())
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn name(&self) -> &str {
        &self.1
    }
}

pub struct IndirectNameAssoc(usize, Vec<NameAssoc>);

impl IndirectNameAssoc {
    pub fn new(index: usize, names: Vec<NameAssoc>) -> IndirectNameAssoc {
        IndirectNameAssoc(index, names)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn names(&self) -> &[NameAssoc] {
        &self.1
    }
}

fn read_subsection_id<R: io::Read>(reader: &mut R) -> Result<Option<u8>, Error> {
    match reader.read_u8() {
        Ok(id) => Ok(Some(id)),
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn read_name_map<R: io::Read>(reader: &mut R) -> Result<Vec<NameAssoc>, Error> {
    utils::read_vec(reader, |r| {
        let idx = utils::read_leb128_u32(r)? as usize;
        let name = utils::read_name(r)?;
        Ok(NameAssoc::new(idx, name))
    })
}

fn read_ind_name_map<R: io::Read>(reader: &mut R) -> Result<Vec<IndirectNameAssoc>, Error> {
    utils::read_vec(reader, |r| {
        let idx = utils::read_leb128_u32(r)? as usize;
        let names = read_name_map(r)?;
        Ok(IndirectNameAssoc::new(idx, names))
    })
}
