use std::io;

use crate::{reader::Section, utils, Error};

pub struct CustomSection {
    pub name: String,
    pub content: Vec<u8>,
}

impl CustomSection {
    pub fn read_content<S: Section>(&self) -> Result<S, Error> {
        S::read(&mut io::Cursor::new(&self.content))
    }
}

impl Section for CustomSection {
    fn read<R: io::Read>(reader: &mut R) -> Result<CustomSection, Error> {
        let name = utils::read_name(reader)?;

        // Read the content
        let mut content = Vec::new();
        reader.read_to_end(&mut content)?;

        Ok(CustomSection { name, content })
    }
}
