use nbt::decode::TagDecodeError;

#[derive(Debug)]
pub(crate) enum MyError {
    BadFilePath,
    OSError(std::io::Error),
    YMLError(serde_yaml::Error),
    NBTError(TagDecodeError),
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::OSError(value)
    }
}

impl From<serde_yaml::Error> for MyError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::YMLError(value)
    }
}

impl From<TagDecodeError> for MyError {
    fn from(value: TagDecodeError) -> Self {
        Self::NBTError(value)
    }
}

pub(crate) type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
pub enum ConfigErr<'a> {
    CommandMissing,
    BadCommand(&'a str),
    ArgError(&'a str),
}
