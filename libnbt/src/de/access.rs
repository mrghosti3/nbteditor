use std::io::{self, Read};

use serde::de;

use crate::err::TagDecodeError as DecodeErr;
use crate::raw;
use super::dec::{Decoder, InnerDecoder};

/// Decoder for map-like types.
pub(super) struct MapDecoder<'a, R: Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: Option<u8>,
}

impl<'a, R: Read> MapDecoder<'a, R> {
    pub(super) fn new(outer: &'a mut Decoder<R>) -> Self {
        MapDecoder { outer, tag: None }
    }
}

impl<'de: 'a, 'a, R: Read + 'a> de::MapAccess<'de> for MapDecoder<'a, R> {
    type Error = DecodeErr;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let tag = raw::read_ubyte(&mut self.outer.reader)?;

        // NBT indicates the end of a compound type with a 0x00 tag.
        if tag == 0x00 {
            return Ok(None);
        }

        // Keep track of the tag so that we can decode the field correctly.
        self.tag = Some(tag as u8);

        // TODO: Enforce that keys must be String. This is a bit of a hack.
        let mut de = InnerDecoder {
            outer: self.outer,
            tag: 0x08,
        };

        Ok(Some(seed.deserialize(&mut de)?))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let mut de = match self.tag {
            Some(tag) => InnerDecoder {
                outer: self.outer,
                tag,
            },
            None => unimplemented!(),
        };
        Ok(seed.deserialize(&mut de)?)
    }
}
///
/// Decoder for list-like types.
pub(super) struct SeqDecoder<'a, R: Read> {
    outer: &'a mut Decoder<R>,
    tag: u8,
    length: i32,
    current: i32,
}

impl<'a, R: Read> SeqDecoder<'a, R>
{
    pub(super) fn list(outer: &'a mut Decoder<R>) -> io::Result<Self> {
        let tag = raw::read_ubyte(&mut outer.reader)?;
        let length = raw::read_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag,
            length,
            current: 0,
        })
    }

    pub(super) fn byte_array(outer: &'a mut Decoder<R>) -> io::Result<Self> {
        let length = raw::read_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x01,
            length,
            current: 0,
        })
    }

    pub(super) fn int_array(outer: &'a mut Decoder<R>) -> io::Result<Self> {
        let length = raw::read_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x03,
            length,
            current: 0,
        })
    }

    pub(super) fn long_array(outer: &'a mut Decoder<R>) -> io::Result<Self> {
        let length = raw::read_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x04,
            length,
            current: 0,
        })
    }
}

impl<'de: 'a, 'a, R: io::Read + 'a> de::SeqAccess<'de> for SeqDecoder<'a, R> {
    type Error = DecodeErr;

    fn next_element_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, DecodeErr>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.current == self.length {
            return Ok(None);
        }

        let mut de = InnerDecoder {
            outer: self.outer,
            tag: self.tag,
        };
        let value = seed.deserialize(&mut de)?;

        self.current += 1;

        Ok(Some(value))
    }

    /// We always know the length of an NBT list in advance.
    fn size_hint(&self) -> Option<usize> {
        Some(self.length as usize)
    }
}
