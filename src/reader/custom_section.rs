use std::io;

use crate::{reader::Section, utils, Error};

pub struct CustomSection {
    pub name: String
}

impl Section for CustomSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<CustomSection, Error> {
        let name = utils::read_name(reader)?;

        // Read to end and dump it
        let mut byts = Vec::new();
        reader.read_to_end(&mut byts)?;

        Ok(CustomSection { name })
    }
}
