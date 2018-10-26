use std::fmt;
use std::mem::transmute;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
}

impl From<u8> for SectionId {
    fn from(i: u8) -> SectionId {
        if i > 11 {
            panic!("Section ID unknown: {}", i);
        } else {
            unsafe { transmute(i) }
        }
    }
}

impl fmt::Display for SectionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct SectionHeader {
    pub id: SectionId,
    pub size: u32,
}
