use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{module::ValType, utils, Error};

#[derive(Clone, Eq, PartialEq)]
pub struct FuncType {
    pub parameters: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub fn empty() -> FuncType {
        FuncType {
            parameters: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn new<P: Into<Vec<ValType>>, R: Into<Vec<ValType>>>(
        parameters: P,
        results: R,
    ) -> FuncType {
        FuncType {
            parameters: parameters.into(),
            results: results.into(),
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<FuncType, Error> {
        let type_code = reader.read_u8()?;
        if type_code != 0x60 {
            Err(Error::InvalidModule)
        } else {
            let parameters = utils::read_vec(reader, |r| ValType::read(r))?;
            let results = utils::read_vec(reader, |r| ValType::read(r))?;
            Ok(FuncType {
                parameters,
                results,
            })
        }
    }
}

impl fmt::Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = true;
        if self.parameters.len() > 0 {
            write!(f, "(param")?;
            start = false;
            for param in self.parameters.iter() {
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
            for res in self.results.iter() {
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
