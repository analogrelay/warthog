use std::io;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use crate::error::Error;
use crate::format::{Section, SectionHeader, SectionId};
use crate::utils;

const EXPECTED_MAGIC: u32 = 0x6D736100;

pub struct Decoder<R: io::Read> {
    source: R,
    version: u32,
}

impl<R: io::Read> Decoder<R> {
    pub fn new(mut source: R) -> Result<Decoder<R>, Error> {
        let version = check_module_header(&mut source)?;

        Ok(Decoder { source, version })
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

impl<R: io::Read + io::Seek> Decoder<R> {
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
