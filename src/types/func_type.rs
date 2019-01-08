use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{types::ValType, utils, Error};

#[derive(Clone, Eq, PartialEq)]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

impl FuncType {
    pub fn empty() -> FuncType {
        FuncType {
            params: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn new(params: Vec<ValType>, results: Vec<ValType>) -> FuncType {
        FuncType { params, results }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<FuncType, Error> {
        let type_code = reader.read_u8()?;
        if type_code != 0x60 {
            Err(Error::InvalidModule)
        } else {
            let params = utils::read_vec(reader, |r| ValType::read(r))?;
            let results = utils::read_vec(reader, |r| ValType::read(r))?;
            Ok(FuncType { params, results })
        }
    }

    pub fn params(&self) -> &[ValType] {
        &self.params
    }

    pub fn results(&self) -> &[ValType] {
        &self.results
    }
}

impl fmt::Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = true;
        if self.params().len() > 0 {
            write!(f, "(param")?;
            start = false;
            for param in self.params().iter() {
                write!(f, " {}", param)?;
            }
            write!(f, ")")?;
        }
        if self.results.len() > 0 {
            if start {
                write!(f, "(result")?;
            } else {
                write!(f, " (result")?;
            }
            for res in self.results().iter() {
                write!(f, " {}", res)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Debug for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
