use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;
use nbt::decode::TagDecodeError;

#[derive(Debug)]
pub(crate) enum MyError<'a> {
    Setup(ConfigErr<'a>),
    Runtime(RuntimeErr),
}

#[derive(Debug)]
pub enum ConfigErr<'a> {
    CommandMissing,
    BadCommand(&'a str),
    ArgError(&'a str),
}

#[derive(Debug)]
pub enum RuntimeErr {
    OSError(io::Error),
    NBTError(TagDecodeError),
    XmlError(quick_xml::Error),
    /// For when unrecognized/unsupported file format is detected
    BadFileFormat {
        file_name: &'static str,
    },
    /// For when unrecognized/unsupported compression algorithm is detected
    BadDataCompression(u8),
    AsciiToUtf8(Utf8Error),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

impl From<io::Error> for RuntimeErr {
    fn from(value: io::Error) -> Self {
        Self::OSError(value)
    }
}

impl From<TagDecodeError> for RuntimeErr {
    fn from(value: TagDecodeError) -> Self {
        Self::NBTError(value)
    }
}

impl From<quick_xml::Error> for RuntimeErr {
    fn from(value: quick_xml::Error) -> Self {
        Self::XmlError(value)
    }
}

impl From<Utf8Error> for RuntimeErr {
    fn from(value: Utf8Error) -> Self {
        Self::AsciiToUtf8(value)
    }
}

impl From<ParseIntError> for RuntimeErr {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<ParseFloatError> for RuntimeErr {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloat(value)
    }
}

pub(crate) type Result<T> = std::result::Result<T, RuntimeErr>;
