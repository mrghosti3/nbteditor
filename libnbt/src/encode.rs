use std::fmt::Debug;
use std::io::Write;

use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use serde::Serialize;

use crate::err::TagEncodeError;
use crate::ser::Encoder;
use crate::CompoundTag;

#[inline]
pub fn write_nbt<'a, 'b: 'a, W: Write + Debug>(nbt_root: &'a CompoundTag, dst: W) -> Result<(), TagEncodeError> {
    let mut encoder = Encoder::new(dst);
    nbt_root.serialize(&mut encoder)
}

#[inline]
pub fn write_gzip_nbt<W: Write + Debug>(nbt_root: &CompoundTag, dst: W) -> Result<(), TagEncodeError> {
    let mut encoder = Encoder::new(GzEncoder::new(dst, Compression::default()));
    nbt_root.serialize(&mut encoder)
}

pub fn write_zlib_nbt<W: Write + Debug>(nbt_root: &CompoundTag, dst: W) -> Result<(), TagEncodeError> {
    let mut encoder = Encoder::new(ZlibEncoder::new(dst, Compression::default()));
    nbt_root.serialize(&mut encoder)
}
