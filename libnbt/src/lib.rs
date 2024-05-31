use core::fmt::Display;

use indexmap::map::Iter;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::consts::tag_name;
use crate::err::TagDecodeError;

mod consts;
mod de;
pub mod decode;
pub mod encode;
pub mod err;
#[macro_use]
mod macros;
mod raw;
mod ser;

pub type Map = IndexMap<Box<str>, Tag>;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(Box<str>),
    List(Vec<Self>),
    Compound(Map),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn from_type_default(tag_type: u8) -> Result<Self, TagDecodeError> {
        use crate::consts::*;
        match tag_type {
            TAG_BYTE => Ok(Self::Byte(Default::default())),
            TAG_SHORT => Ok(Self::Short(Default::default())),
            TAG_INT => Ok(Self::Int(Default::default())),
            TAG_LONG => Ok(Self::Long(Default::default())),
            TAG_FLOAT => Ok(Self::Float(Default::default())),
            TAG_DOUBLE => Ok(Self::Double(Default::default())),
            TAG_BYTE_ARRAY => Ok(Self::ByteArray(Default::default())),
            TAG_INT_ARRAY => Ok(Self::IntArray(Default::default())),
            TAG_LONG_ARRAY => Ok(Self::LongArray(Default::default())),
            TAG_STRING => Ok(Self::String(Default::default())),
            TAG_LIST => Ok(Self::List(Default::default())),
            TAG_COMPOUND => Ok(Self::Compound(Default::default())),
            ttype => Err(TagDecodeError::UnknownTagType { tag_type_id: ttype }),
        }
    }
}

impl From<Vec<Tag>> for Tag {
    fn from(value: Vec<Tag>) -> Self {
        let tag = match value.get(0) {
            Some(t) => t,
            _ => return Self::List(vec![]),
        };

        match tag {
            Self::Byte(_) => Self::ByteArray(
                value
                    .into_iter()
                    .map(|val| match val {
                        Self::Byte(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Self::Int(_) => Self::IntArray(
                value
                    .into_iter()
                    .map(|val| match val {
                        Self::Int(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Self::Long(_) => Self::LongArray(
                value
                    .into_iter()
                    .map(|val| match val {
                        Self::Long(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            _ => Tag::List(value)
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::consts::*;
        match self {
            Self::Byte(_) => write!(f, "{}", tag_name![TAG_BYTE]),
            Self::Short(_) => write!(f, "{}", tag_name![TAG_SHORT]),
            Self::Int(_) => write!(f, "{}", tag_name![TAG_INT]),
            Self::Long(_) => write!(f, "{}", tag_name![TAG_LONG]),
            Self::Float(_) => write!(f, "{}", tag_name![TAG_FLOAT]),
            Self::Double(_) => write!(f, "{}", tag_name![TAG_DOUBLE]),
            Self::ByteArray(_) => write!(f, "{}", tag_name![TAG_BYTE_ARRAY]),
            Self::IntArray(_) => write!(f, "{}", tag_name![TAG_INT_ARRAY]),
            Self::LongArray(_) => write!(f, "{}", tag_name![TAG_LONG_ARRAY]),
            Self::String(_) => write!(f, "{}", tag_name![TAG_STRING]),
            Self::List(_) => write!(f, "{}", tag_name![TAG_LIST]),
            Self::Compound(_) => write!(f, "{}", tag_name![TAG_COMPOUND]),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct CompoundTag {
    pub name: Option<Box<str>>,
    pub(crate) map: Map,
}

impl CompoundTag {
    #[inline]
    pub fn new() -> Self {
        CompoundTag {
            name: None,
            map: IndexMap::with_capacity(5),
        }
    }

    #[inline]
    pub fn named(name: Box<str>) -> Self {
        CompoundTag {
            name: Some(name),
            map: IndexMap::with_capacity(5),
        }
    }

    #[inline]
    pub fn with(map: IndexMap<Box<str>, Tag>) -> Self {
        Self { name: None, map }
    }

    #[inline]
    pub fn push(&mut self, key: Box<str>, value: Tag) {
        self.map.insert(key, value);
    }

    pub fn iter(&self) -> Iter<Box<str>, Tag> {
        self.map.iter()
    }

    pub fn get(&self, key: &str) -> Option<&Tag> {
        self.map.get(key)
    }
}

impl Serialize for CompoundTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut state = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            state.serialize_entry(k, v)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for CompoundTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map: Map = serde::de::Deserialize::deserialize(deserializer)?;
        Ok(Self { name: None, map })
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(crate::de::visit::TagVisitor)
    }
}
