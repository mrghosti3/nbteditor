use std::io;
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

pub(crate) type Result<T> = std::result::Result<T, RuntimeErr>;
