use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{module::ValType, utils, Error};

#[derive(Clone)]
pub struct FuncType {
    pub parameters: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
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
        write!(f, "(func")?;
        if self.parameters.len() > 0 {
            write!(f, " (param")?;
            for param in self.parameters.iter() {
                write!(f, " {}", param)?;
            }
            write!(f, ")")?;
        }
        if self.results.len() > 0 {
            write!(f, " (result")?;
            for res in self.results.iter() {
                write!(f, " {}", res)?;
            }
            write!(f, ")")?;
        }
        write!(f, ")")
    }
}
