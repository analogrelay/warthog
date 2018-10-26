mod code_section;
mod data_section;
mod export_section;
mod function_section;
mod import_section;
mod section_header;
mod type_section;

pub use self::code_section::CodeSection;
pub use self::data_section::DataSection;
pub use self::export_section::ExportSection;
pub use self::function_section::FunctionSection;
pub use self::import_section::ImportSection;
pub use self::section_header::{SectionHeader, SectionId};
pub use self::type_section::TypeSection;

use std::io;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use crate::{utils, Error};

pub trait Section: Sized {
    fn read<R: io::Read>(reader: &mut R) -> Result<Self, Error>;
}

const EXPECTED_MAGIC: u32 = 0x6D736100;

pub struct Reader<R: io::Read> {
    source: R,
    version: u32,
}

impl<R: io::Read> Reader<R> {
    pub fn new(mut source: R) -> Result<Reader<R>, Error> {
        let version = check_module_header(&mut source)?;

        Ok(Reader { source, version })
    }

    pub fn read_section_header(&mut self) -> Result<Option<SectionHeader>, Error> {
        let id = match self.source.read_u8() {
            Ok(i) => SectionId::from(i),
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        Ok(Some(SectionHeader {
            id,
            size: utils::read_leb128_u32(&mut self.source)?,
        }))
    }

    pub fn read_section<S: Section>(&mut self) -> Result<S, Error> {
        S::read(&mut self.source)
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

impl<R: io::Read + io::Seek> Reader<R> {
    pub fn skip(&mut self, amount: usize) -> Result<(), Error> {
        self.source.seek(io::SeekFrom::Current(amount as i64))?;
        Ok(())
    }
}

fn check_module_header<R: io::Read>(r: &mut R) -> Result<u32, Error> {
    // Read the header data
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    let magic_num = LittleEndian::read_u32(&magic);

    if magic_num != EXPECTED_MAGIC {
        return Err(Error::InvalidModule);
    }

    let mut version = [0u8; 4];
    r.read_exact(&mut version)?;
    let version_num = LittleEndian::read_u32(&version);
    Ok(version_num)
}
