use std::io::Read;

use serde::de::{self, Deserializer};
use serde::forward_to_deserialize_any;

use crate::consts::{TAG_BYTE, TAG_COMPOUND};
use crate::err::TagDecodeError as DecodeErr;
use crate::raw;

use super::access::{MapDecoder, SeqDecoder};

pub(crate) struct Decoder<R> {
    pub(super) reader: R,
}

impl<R: Read> Decoder<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<'de, 'a, R: Read> Deserializer<'de> for &'a mut Decoder<R> {
    type Error = DecodeErr;

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (tag, _) = raw::read_header(&mut self.reader)?;

        match tag {
            TAG_COMPOUND => visitor.visit_map(MapDecoder::new(self)),
            _ => Err(DecodeErr::RootMustBeCompound),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DecodeErr::RootMustBeCompound)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str
        string bytes byte_buf unit seq tuple_struct tuple
        option enum identifier ignored_any
    }
}

/// Private inner decoder, for decoding raw (i.e. non-Compound) types.
pub(super) struct InnerDecoder<'a, R: Read + 'a> {
    pub(super) outer: &'a mut Decoder<R>,
    pub(super) tag: u8,
}

impl<'a, 'de, R: Read> de::Deserializer<'de> for &'a mut InnerDecoder<'a, R> {
    type Error = DecodeErr;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let outer = &mut self.outer;

        match self.tag {
            0x01 => visitor.visit_i8(raw::read_byte(&mut outer.reader)?),
            0x02 => visitor.visit_i16(raw::read_short(&mut outer.reader)?),
            0x03 => visitor.visit_i32(raw::read_int(&mut outer.reader)?),
            0x04 => visitor.visit_i64(raw::read_long(&mut outer.reader)?),
            0x05 => visitor.visit_f32(raw::read_float(&mut outer.reader)?),
            0x06 => visitor.visit_f64(raw::read_double(&mut outer.reader)?),
            0x07 => visitor.visit_seq(SeqDecoder::byte_array(outer)?),
            0x08 => {
                visitor.visit_string(raw::read_string(&mut outer.reader)?.unwrap_or("".to_string()))
            }
            0x09 => visitor.visit_seq(SeqDecoder::list(outer)?),
            0x0a => visitor.visit_map(MapDecoder::new(outer)),
            0x0b => visitor.visit_seq(SeqDecoder::int_array(outer)?),
            0x0c => visitor.visit_seq(SeqDecoder::long_array(outer)?),
            tag_type_id => Err(DecodeErr::UnknownTagType { tag_type_id }),
        }
    }

    /// Deserialize bool values from a byte. Fail if that byte is not 0 or 1.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.tag {
            0x01 => {
                let reader = &mut self.outer.reader;
                let value = raw::read_byte(reader)?;
                match value {
                    0 => visitor.visit_bool(false),
                    1 => visitor.visit_bool(true),
                    b => Err(DecodeErr::NonBooleanByte(b)),
                }
            }
            _ => Err(DecodeErr::TagMismatch{ found: self.tag, expected: TAG_BYTE }),
        }
    }

    /// Interpret missing values as None.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// Deserialize newtype structs by their underlying types.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any
    }
}
