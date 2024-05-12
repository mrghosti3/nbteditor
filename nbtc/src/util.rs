use crate::err::RuntimeErr;

#[repr(u8)]
pub enum DataFormat {
    Gzip = 1,
    Zlib = 2,
    NBT = 3,
    LZ4 = 4,
    Custom = 127,
}

impl DataFormat {
    pub const BYTE_COUNT: usize = 5;

    /// Matches known bytes from a files' start to identify how data is stored inside.
    ///
    /// Supported formats by magic bytes:
    /// - [Gzip](https://en.wikipedia.org/wiki/Gzip)
    /// - [Zlib](https://en.wikipedia.org/wiki/Zlib)
    /// - [NBT (fallback)](https://wiki.vg/NBT)
    pub fn from_magic_bytes(magic: &[u8; Self::BYTE_COUNT]) -> Self {
        match magic {
            [0x1F, 0x8B, 0x08, ..] => Self::Gzip,
            [b'P', b'K', 0x03, 0x04, ..] => Self::Zlib,
            _ => Self::NBT,
        }
    }
}

impl TryFrom<u8> for DataFormat {
    type Error = RuntimeErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Gzip),
            3 => Ok(Self::NBT),
            4 => Ok(Self::LZ4),
            127 => Ok(Self::Custom),
            _ => Err(RuntimeErr::BadDataCompression(value))
        }
    }
}
