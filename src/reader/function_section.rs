use std::io;

use crate::{reader::Section, utils, Error};

pub struct FunctionSection {
    funcs: Vec<u32>,
}

impl Section for FunctionSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<FunctionSection, Error> {
        let funcs = utils::read_vec(reader, |r| utils::read_leb128_u32(r))?;

        Ok(FunctionSection { funcs })
    }
}

impl FunctionSection {
    pub fn funcs(&self) -> &[u32] {
        &self.funcs
    }
}
