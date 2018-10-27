use std::io;

use crate::{
    module::{Instruction, Local},
    utils, Error,
};

#[derive(Clone)]
pub struct FuncBody {
    pub size: u32,
    pub locals: Vec<Local>,
    pub body: Vec<Instruction>,
}

impl FuncBody {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<FuncBody, Error> {
        let size = utils::read_leb128_u32(reader)?;
        let locals = utils::read_vec(reader, |r| Local::read(r))?;
        let body = utils::read_instructions(reader)?;

        Ok(FuncBody { size, locals, body })
    }
}