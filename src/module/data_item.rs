use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{module::Expr, utils, Error};

#[derive(PartialEq, Clone)]
pub struct DataItem {
    index: usize,
    expr: Expr,
    init: Vec<u8>,
}

impl DataItem {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<DataItem, Error> {
        let index = utils::read_leb128_u32(reader)? as usize;
        let expr = Expr::new(utils::read_instructions(reader)?);
        let init = utils::read_vec(reader, |r| Ok(r.read_u8()?))?;
        Ok(DataItem { index, expr, init })
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn init(&self) -> &[u8] {
        &self.init
    }
}

impl fmt::Display for DataItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data")?;
        if self.index > 0 {
            write!(f, " {}", self.index)?;
        }
        if self.expr.len() > 1 {
            write!(f, " (offset")?;
        }
        write!(f, " (")?;
        let mut first = true;
        for inst in self.expr.iter() {
            if first {
                write!(f, "{}", inst)?;
                first = false;
            } else {
                write!(f, " {}", inst)?;
            }
        }
        write!(f, ")")?;
        if self.expr.len() > 1 {
            write!(f, ")")?;
        }

        // Format the data
        write!(f, " \"")?;
        for byt in self.init.iter().cloned() {
            unsafe {
                let bytes: Vec<_> = std::ascii::escape_default(byt).collect();
                let escaped = std::str::from_utf8_unchecked(&bytes);
                write!(f, "{}", escaped)?;
            }
        }
        write!(f, "\")")
    }
}

impl fmt::Debug for DataItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
