use std::io;

use crate::{module::DataItem, reader::Section, utils, Error};

pub struct DataSection {
    pub data: Vec<DataItem>,
}

impl Section for DataSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<DataSection, Error> {
        let data = utils::read_vec(reader, |r| DataItem::read(r))?;

        Ok(DataSection { data })
    }
}
