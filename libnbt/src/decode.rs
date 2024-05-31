use std::io::Read;

use crate::de::dec::Decoder;
use crate::err::TagDecodeError;
use crate::CompoundTag;

pub fn read_nbt<R: Read>(reader: R) -> Result<CompoundTag, TagDecodeError> {
    let mut decoder = Decoder::new(reader);
    serde::Deserialize::deserialize(&mut decoder)
}

pub fn read_gzip_nbt<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
    let mut greader = flate2::read::GzDecoder::new(reader);
    read_nbt(&mut greader)
}

pub fn read_zlib_nbt<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
    let mut greader = flate2::read::ZlibDecoder::new(reader);
    read_nbt(&mut greader)
}
