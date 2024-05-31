use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use crate::err::{TagDecodeError, TagEncodeError};
use crate::consts::TAG_END;

#[inline]
pub fn read_header<R: Read>(src: &mut R) -> Result<(u8, Option<Box<str>>), TagDecodeError> {
    let tag = src.read_u8()?;
    if tag == TAG_END {
        return Ok((tag, None));
    }

    let name = read_string(src)?.map(|n| n.into_boxed_str());
    Ok((tag, name))
}

#[inline]
pub fn read_ubyte<R: Read>(src: &mut R) -> io::Result<u8> {
    src.read_u8()
}

#[inline]
pub fn read_byte<R: Read>(src: &mut R) -> io::Result<i8> {
    src.read_i8()
}

#[inline]
pub fn read_short<R: Read>(src: &mut R) -> io::Result<i16> {
    src.read_i16::<BigEndian>()
}

#[inline]
pub fn read_int<R: Read>(src: &mut R) -> io::Result<i32> {
    src.read_i32::<BigEndian>()
}

#[inline]
pub fn read_long<R: Read>(src: &mut R) -> io::Result<i64> {
    src.read_i64::<BigEndian>()
}

#[inline]
pub fn read_float<R: Read>(src: &mut R) -> io::Result<f32> {
    src.read_f32::<BigEndian>()
}

#[inline]
pub fn read_double<R: Read>(src: &mut R) -> io::Result<f64> {
    src.read_f64::<BigEndian>()
}

#[inline]
pub fn read_string<R: Read>(src: &mut R) -> Result<Option<String>, TagDecodeError> {
    let len = src.read_i16::<BigEndian>()? as usize;

    if len == 0 {
        return Ok(None);
    }

    let mut buf = vec![0; len];
    src.read_exact(&mut buf)?;
    Ok(Some(String::from_utf8(buf)?))
}

#[inline]
pub fn write_str<W: Write>(dst: &mut W, value: &str) -> Result<(), TagEncodeError> {
    dst.write_u16::<BigEndian>(value.len() as u16)?;
    dst.write_all(value.as_ref()).map_err(From::from)
}

#[inline]
pub fn write_byte<W: Write>(dst: &mut W, value: i8) -> Result<(), TagEncodeError> {
    dst.write_i8(value).map_err(From::from)
}

#[inline]
pub fn write_ubyte<W: Write>(dst: &mut W, value: u8) -> Result<(), TagEncodeError> {
    dst.write_u8(value).map_err(From::from)
}

#[inline]
pub fn write_short<W: Write>(dst: &mut W, value: i16) -> Result<(), TagEncodeError> {
    dst.write_i16::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_int<W: Write>(dst: &mut W, value: i32) -> Result<(), TagEncodeError> {
    dst.write_i32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_long<W: Write>(dst: &mut W, value: i64) -> Result<(), TagEncodeError> {
    dst.write_i64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_float<W: Write>(dst: &mut W, value: f32) -> Result<(), TagEncodeError> {
    dst.write_f32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_double<W: Write>(dst: &mut W, value: f64) -> Result<(), TagEncodeError> {
    dst.write_f64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn close_nbt<W: Write>(dst: &mut W) -> Result<(), TagEncodeError> {
    dst.write_u8(TAG_END).map_err(From::from)
}
