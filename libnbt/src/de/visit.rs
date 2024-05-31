use serde::de::Visitor;

use crate::{Map, Tag};

pub(crate) struct TagVisitor;

impl<'de> Visitor<'de> for TagVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Byte(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Short(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Long(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Float(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::Double(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Tag::String(v.into_boxed_str()))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut buf = Map::new();

        while let Some((key, val)) = map.next_entry()? {
            buf.insert(key, val);
        }

        Ok(Tag::Compound(buf))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        let mut buf = Vec::with_capacity(seq.size_hint().unwrap());

        while let Some(el) = seq.next_element::<Tag>()? {
            buf.push(el);
        }

        Ok(buf.into())
    }
}
