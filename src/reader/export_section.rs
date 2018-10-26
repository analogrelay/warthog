use std::io;

use crate::{module::Export, reader::Section, utils, Error};

pub struct ExportSection {
    pub exports: Vec<Export>,
}

impl Section for ExportSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<ExportSection, Error> {
        let exports = utils::read_vec(reader, |r| Export::read(r))?;

        Ok(ExportSection { exports })
    }
}
