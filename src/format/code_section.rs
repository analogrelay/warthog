use std::io;
use std::fmt;

use error::Error;
use utils;

use super::{Instruction, Section, ValType};

pub struct CodeSection {
    code: Vec<CodeEntry>,
}

impl Section for CodeSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<CodeSection, Error> {
        let code = utils::read_vec(reader, |r| CodeEntry::read(r))?;

        Ok(CodeSection { code })
    }
}

impl CodeSection {
    pub fn code(&self) -> &[CodeEntry] {
        &self.code
    }
}

pub struct CodeEntry {
    pub size: u32,
    pub locals: Vec<Local>,
    pub body: Vec<Instruction>,
}

impl CodeEntry {
    fn read<R: io::Read>(reader: &mut R) -> Result<CodeEntry, Error> {
        let size = utils::read_leb128_u32(reader)?;
        let locals = utils::read_vec(reader, |r| Local::read(r))?;
        let body = utils::read_instructions(reader)?;

        Ok(CodeEntry {
            size,
            locals,
            body
        })
    }
}

pub struct Local {
    pub count: u32,
    pub typ: ValType,
}

impl Local {
    fn read<R: io::Read>(reader: &mut R) -> Result<Local, Error> {
        let count = utils::read_leb128_u32(reader)?;
        let typ = ValType::read(reader)?;

        Ok(Local { count, typ })
    }
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(local")?;
        for x in 0..self.count {
            write!(f, " {}", self.typ)?;
        }
        write!(f, ")")
    }
}
