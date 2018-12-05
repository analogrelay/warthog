use std::io;

use byteorder::ReadBytesExt;

use crate::Error;

pub trait FromLeb128 {
    fn from_leb128_u(leb: u64) -> Self;
    fn from_leb128_s(leb: i64) -> Self;
}

macro_rules! impl_from_leb {
    ($target: ty) => {
        impl FromLeb128 for $target {
            fn from_leb128_u(leb: u64) -> $target {
                leb as $target
            }

            fn from_leb128_s(leb: i64) -> $target {
                leb as $target
            }
        }
    };
}

impl_from_leb!(usize);
impl_from_leb!(isize);
impl_from_leb!(u32);
impl_from_leb!(u64);
impl_from_leb!(i32);
impl_from_leb!(i64);

pub fn read_leb128_s<R: io::Read, T: FromLeb128>(r: &mut R) -> Result<T, Error> {
    Ok(T::from_leb128_s(leb128::read::signed(r)?))
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

pub fn read_limits<R: io::Read>(r: &mut R) -> Result<(usize, Option<usize>), Error> {
    let kind = r.read_u8()?;
    let min = read_leb128_u32(r)? as usize;

    match kind {
        0x00 => Ok((min, None)),
        0x01 => {
            let max = read_leb128_u32(r)? as usize;
            Ok((min, Some(max)))
        }
        _ => Err(Error::InvalidModule),
    }
}
