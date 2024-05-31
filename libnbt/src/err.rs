use std::error::Error as StdErr;
use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

pub type DecodeResult<T> = Result<T, TagDecodeError>;
pub type EncodeResult<T> = Result<T, TagEncodeError>;

#[derive(Debug)]
pub enum TagDecodeError {
    /// Contains element of found tag that isn't of type TAG_Compound
    RootMustBeCompound,
    UnknownTagType {
        tag_type_id: u8,
    },
    IOError(io::Error),
    Serde(String),
    TextDecodeErr(Utf8Error),
    StringDecodeErr(FromUtf8Error),
    TagMismatch {
        found: u8,
        expected: u8,
    },
    NonBooleanByte(i8),
}

#[derive(Debug)]
pub enum TagEncodeError {
    IOError(io::Error),
    Serde(String),
    RootMustBeCompound,
    UnrepresentableType(&'static str),
    NonStringMapKey,
}

impl Display for TagDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "libnbt: ")?;
        match self {
            Self::IOError(io_error) => write!(f, "IO ERROR: {}", io_error),
            Self::UnknownTagType { tag_type_id } => {
                write!(f, "Unknown tag type: {:#}", tag_type_id)
            }
            Self::RootMustBeCompound => {
                write!(f, "The initial TAG must of type TAG_Compound",)
            }
            Self::Serde(txt) => write!(f, "Error with serde operation: {}", txt),
            Self::TextDecodeErr(txt) => write!(f, "Error while parsing text: {}", txt),
            Self::StringDecodeErr(txt) => write!(f, "Error while parsing text: {}", txt),
            Self::NonBooleanByte(b) => write!(f, "Non boolean byte found: {:x}", b),
            Self::TagMismatch { found, expected } => {
                write!(f, "Was exptecting {:x} but found {:x}", expected, found)
            }
        }
    }
}

impl Display for TagEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "libnbt: ")?;
        match self {
            Self::IOError(io_err) => write!(f, "IO ERROR: {}", io_err),
            Self::Serde(txt) => write!(f, "Error with serde: {}", txt),
            Self::RootMustBeCompound => {
                write!(f, "The initial TAG must of type TAG_Compound",)
            }
            Self::UnrepresentableType(txt) => write!(f, "Found unrepresentable type: {}", txt),
            Self::NonStringMapKey => todo!(),
        }
    }
}

impl From<io::Error> for TagDecodeError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<io::Error> for TagEncodeError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl serde::de::Error for TagDecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Serde(msg.to_string())
    }
}

impl serde::ser::Error for TagEncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Serde(msg.to_string())
    }
}

impl StdErr for TagDecodeError {
    fn source(&self) -> Option<&(dyn StdErr + 'static)> {
        match self {
            Self::IOError(e) => e.source(),
            _ => None,
        }
    }
}

impl StdErr for TagEncodeError {
    fn source(&self) -> Option<&(dyn StdErr + 'static)> {
        match self {
            Self::IOError(e) => e.source(),
            _ => None,
        }
    }
}

impl From<Utf8Error> for TagDecodeError {
    fn from(value: Utf8Error) -> Self {
        Self::TextDecodeErr(value)
    }
}

impl From<FromUtf8Error> for TagDecodeError {
    fn from(value: FromUtf8Error) -> Self {
        Self::StringDecodeErr(value)
    }
}
