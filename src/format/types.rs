use std::fmt;
use std::io;

use byteorder::ReadBytesExt;

use error::Error;
use utils;

pub enum ValType {
    Integer32,
    Integer64,
    Float32,
    Float64,
}

impl ValType {
    pub fn from_u8(v: u8) -> Result<ValType, Error> {
        match v {
            0x7F => Ok(ValType::Integer32),
            0x7E => Ok(ValType::Integer64),
            0x7D => Ok(ValType::Float32),
            0x7C => Ok(ValType::Float64),
            _ => Err(Error::InvalidModule),
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<ValType, Error> {
        let v = reader.read_u8()?;
        ValType::from_u8(v)
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            ValType::Integer32 => "i32",
            ValType::Integer64 => "i64",
            ValType::Float32 => "f32",
            ValType::Float64 => "f64",
        };
        write!(f, "{}", v)
    }
}

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

pub struct TableType {
    pub elem_type: u8,
    pub min: u32,
    pub max: Option<u32>,
}

impl TableType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<TableType, Error> {
        let elem_type = reader.read_u8()?;
        if elem_type != 0x70 {
            Err(Error::InvalidModule)
        } else {
            let (min, max) = utils::read_limits(reader)?;
            Ok(TableType { elem_type, min, max })
        }
    }
}

impl fmt::Display for TableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(table {}", self.min)?;
        if let Some(max) = self.max {
            write!(f, " {}", max)?;
        }
        write!(f, " anyfunc)")
    }
}

pub struct MemoryType {
    pub min: u32,
    pub max: Option<u32>,
}

impl MemoryType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<MemoryType, Error> {
        let (min, max) = utils::read_limits(reader)?;
        Ok(MemoryType { min, max })
    }
}

impl fmt::Display for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(memory {}", self.min)?;
        if let Some(max) = self.max {
            write!(f, " {}", max)?;
        }
        write!(f, ")")
    }
}

pub struct GlobalType {
    pub valtype: ValType,
    pub mutable: bool
}

impl GlobalType {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<GlobalType, Error> {
        let valtype = ValType::read(reader)?;
        let mutable = match reader.read_u8()? {
            0x00 => false,
            0x01 => true,
            _ => return Err(Error::InvalidModule)
        };
        Ok(GlobalType { valtype, mutable })
    }
}

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(global ")?;
        if self.mutable {
            write!(f, "(mut {})", self.valtype)?;
        } else {
            write!(f, "{}", self.valtype)?;
        }
        write!(f, ")")
    }
}