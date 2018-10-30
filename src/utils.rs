use std::io;

use byteorder::ReadBytesExt;

use crate::{module::Instruction, Error};

pub fn read_leb128_i32<R: io::Read>(r: &mut R) -> Result<i32, Error> {
    Ok(leb128::read::signed(r)? as i32)
}

pub fn read_leb128_u32<R: io::Read>(r: &mut R) -> Result<u32, Error> {
    Ok(leb128::read::unsigned(r)? as u32)
}

pub fn read_vec<R, F, I>(r: &mut R, mut body: F) -> Result<Vec<I>, Error>
where
    R: io::Read,
    F: FnMut(&mut R) -> Result<I, Error>,
{
    // Read the size and pre-allocate the Vec
    let size = read_leb128_u32(r)?;
    let mut vec = Vec::with_capacity(size as usize);

    // Read the items
    for _ in 0..size {
        vec.push(body(r)?);
    }
    Ok(vec)
}

pub fn read_name<R: io::Read>(r: &mut R) -> Result<String, Error> {
    let byts: Vec<u8> = read_vec(r, |x| Ok(x.read_u8()?))?;
    Ok(String::from_utf8(byts)?)
}

pub fn read_limits<R: io::Read>(r: &mut R) -> Result<(u32, Option<u32>), Error> {
    let kind = r.read_u8()?;
    let min = read_leb128_u32(r)?;

    match kind {
        0x00 => Ok((min, None)),
        0x01 => {
            let max = read_leb128_u32(r)?;
            Ok((min, Some(max)))
        }
        _ => Err(Error::InvalidModule),
    }
}

pub fn read_instructions<R: io::Read>(r: &mut R) -> Result<Vec<Instruction>, Error> {
    let mut insts = Vec::new();
    loop {
        if let Some(i) = Instruction::read(r)? {
            insts.push(i);
        } else {
            return Ok(insts);
        }
    }
}
