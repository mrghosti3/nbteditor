use core::f64;
use std::fmt::Debug;
use std::i16;
use std::io::Write;

use serde::ser;

use crate::consts::*;
use crate::err::TagEncodeError;
use crate::raw;

/// Encode data to NBT format.
///
/// Implements `serde::Serialize` to serialize objects into NBT data.
#[derive(Debug)]
pub(crate) struct Encoder<W: Write> {
    writer: W,
}

#[derive(Debug)]
pub(crate) struct Compound<'a, W: Write> {
    outer: &'a mut Encoder<W>,
    length: i32,
    sigil: bool,
}

struct TagEncoder<'a, W: Write, K> {
    outer: &'a mut Encoder<W>,
    key: Option<K>,
}

struct InnerEncoder<'a, W: Write> {
    outer: &'a mut Encoder<W>,
}

struct MapKeyEncoder<'a, W: Write> {
    outer: &'a mut Encoder<W>,
}

impl<'a, W: Write + Debug> Encoder<W> {
    /// Create encoder
    #[inline]
    pub(crate) fn new(writer: W) -> Self {
        Self { writer }
    }

    #[inline]
    fn write_header(&mut self, tag: u8, header: Option<&str>) -> Result<(), TagEncodeError> {
        raw::write_ubyte(&mut self.writer, tag)?;
        match header {
            None => raw::write_short(&mut self.writer, 0)?,
            Some(head) => raw::write_str(&mut self.writer, head)?,
        }
        Ok(())
    }
}

impl<'a, W: Write> Compound<'a, W> {
    #[inline]
    fn from_outer(outer: &'a mut Encoder<W>) -> Self {
        Self {
            outer,
            length: 0,
            sigil: false,
        }
    }

    #[inline]
    fn for_seq(
        outer: &'a mut Encoder<W>,
        length: i32,
        array: bool,
    ) -> Result<Self, TagEncodeError> {
        if length == 0 || array {
            if !array {
                raw::write_ubyte(&mut outer.writer, TAG_END)?;
            }

            raw::write_int(&mut outer.writer, length)?;
        }

        Ok(Self {
            outer,
            length,
            sigil: false,
        })
    }
}

impl<'a, W, K> TagEncoder<'a, W, K>
where
    W: Write + 'a,
    K: serde::Serialize,
{
    #[inline]
    fn from_outer(outer: &'a mut Encoder<W>, key: Option<K>) -> Self {
        Self { outer, key }
    }

    #[inline]
    fn write_header(&mut self, tag: u8) -> Result<(), TagEncodeError> {
        use serde::Serialize;
        raw::write_ubyte(&mut self.outer.writer, tag)?;
        self.key
            .serialize(&mut MapKeyEncoder::from_outer(self.outer))
    }
}

impl<'a, W: Write> InnerEncoder<'a, W> {
    #[inline]
    pub fn from_outer(outer: &'a mut Encoder<W>) -> Self {
        Self { outer }
    }
}

impl<'a, W: Write> MapKeyEncoder<'a, W> {
    #[inline]
    fn from_outer(outer: &'a mut Encoder<W>) -> Self {
        Self { outer }
    }
}

impl<'a, W: Write + Debug> serde::Serializer for &'a mut Encoder<W> {
    type Ok = ();
    type Error = TagEncodeError;
    type SerializeSeq = serde::ser::Impossible<(), TagEncodeError>;
    type SerializeTuple = serde::ser::Impossible<(), TagEncodeError>;
    type SerializeTupleStruct = serde::ser::Impossible<(), TagEncodeError>;
    type SerializeTupleVariant = serde::ser::Impossible<(), TagEncodeError>;
    type SerializeStructVariant = serde::ser::Impossible<(), TagEncodeError>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), TagEncodeError> {
        self.write_header(TAG_COMPOUND, None)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_header(TAG_COMPOUND, None)?;
        Ok(Compound::from_outer(self))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_header(TAG_COMPOUND, None)?;
        Ok(Compound::from_outer(self))
    }

    return_expr_for_serialized_types!(
        Err(TagEncodeError::RootMustBeCompound); bool i8 i16 i32 i64 u8 u16 u32 u64
        f32 f64 char str bytes none some unit unit_variant newtype_variant
        seq tuple tuple_struct tuple_variant struct_variant
    );
}

impl<'a, W: Write + Debug> ser::SerializeSeq for Compound<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if !self.sigil {
            value.serialize(&mut TagEncoder::from_outer(
                self.outer,
                Option::<String>::None,
            ))?;
            raw::write_int(&mut self.outer.writer, self.length)?;
            self.sigil = true;
        }
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, W: Write + Debug> ser::SerializeTupleStruct for Compound<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: Write + Debug> ser::SerializeStruct for Compound<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(key)))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        raw::close_nbt(&mut self.outer.writer)
    }
}

impl<'a, W: Write + Debug> ser::SerializeMap for Compound<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(key)))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        raw::close_nbt(&mut self.outer.writer)
    }

    #[inline]
    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        unimplemented!()
    }
}

impl<'a, W: Write + Debug> serde::Serializer for &'a mut InnerEncoder<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;
    type SerializeSeq = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTuple = ser::Impossible<(), TagEncodeError>;
    type SerializeTupleVariant = ser::Impossible<(), TagEncodeError>;
    type SerializeStructVariant = ser::Impossible<(), TagEncodeError>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as i8)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        raw::write_byte(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        raw::write_short(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        raw::write_int(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        raw::write_long(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        raw::write_float(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        raw::write_double(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        raw::write_str(&mut self.outer.writer, v)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        raw::close_nbt(&mut self.outer.writer)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound::from_outer(self.outer))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound::from_outer(self.outer))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match name {
            LIBNT_I8_ARRAY | LIBNT_I32_ARRAY | LIBNT_I64_ARRAY => {
                Compound::for_seq(self.outer, len as i32, true)
            }
            _ => Err(TagEncodeError::UnrepresentableType(stringify!(
                tuple_struct
            ))),
        }
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(l) => Compound::for_seq(self.outer, l as i32, false),
            None => Err(TagEncodeError::UnrepresentableType("unsized list")),
        }
    }

    return_expr_for_serialized_types!(Err(TagEncodeError::UnrepresentableType("u8")); bytes);
    return_expr_for_serialized_types!(Ok(()); none);
    unrepresentable!(
        u8 u16 u32 u64 char unit newtype_variant
        tuple tuple_variant struct_variant
    );
}

impl<'a, W, K> serde::Serializer for &'a mut TagEncoder<'a, W, K>
where
    W: Write,
    K: serde::Serialize,
{
    type Ok = ();
    type Error = TagEncodeError;
    type SerializeSeq = NoOp;
    type SerializeTupleStruct = NoOp;
    type SerializeMap = NoOp;
    type SerializeStruct = NoOp;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as i8)
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_BYTE)
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_SHORT)
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_INT)
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_LONG)
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_FLOAT)
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_DOUBLE)
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_STRING)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.write_header(TAG_COMPOUND)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(_) => {
                self.write_header(TAG_LIST)?;
                Ok(NoOp)
            }
            None => Err(TagEncodeError::UnrepresentableType("unsized list")),
        }
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match name {
            LIBNT_I8_ARRAY => self.write_header(TAG_BYTE_ARRAY)?,
            LIBNT_I32_ARRAY => self.write_header(TAG_INT_ARRAY)?,
            LIBNT_I64_ARRAY => self.write_header(TAG_LONG_ARRAY)?,
            _ => return Err(TagEncodeError::UnrepresentableType("tuple struct")),
        }

        Ok(NoOp)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_header(TAG_COMPOUND)?;
        Ok(NoOp)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeMap, Self::Error> {
        self.write_header(TAG_COMPOUND)?;
        Ok(NoOp)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TagEncodeError::UnrepresentableType("u8"))
    }

    return_expr_for_serialized_types!(Ok(()); none);
    unrepresentable!(
        u8 u16 u32 u64 char unit newtype_variant
        tuple tuple_variant struct_variant
    );
}

impl<'a, W: Write> serde::Serializer for &'a mut MapKeyEncoder<'a, W> {
    type Ok = ();
    type Error = TagEncodeError;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        raw::write_str(&mut self.outer.writer, v)
    }

    return_expr_for_serialized_types!(Ok(()); none);
    return_expr_for_serialized_types!(
        Err(TagEncodeError::NonStringMapKey); bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
        char bytes unit unit_variant unit_struct newtype_variant newtype_struct seq map
        tuple tuple_variant tuple_struct struct struct_variant
    );
}

struct NoOp;

impl ser::SerializeSeq for NoOp {
    type Ok = ();
    type Error = TagEncodeError;

    #[inline]
    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeMap for NoOp {
    type Ok = ();
    type Error = TagEncodeError;

    #[inline]
    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeStruct for NoOp {
    type Ok = ();
    type Error = TagEncodeError;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for NoOp {
    type Ok = ();
    type Error = TagEncodeError;

    #[inline]
    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

const LIBNT_I8_ARRAY: &str = "__libnbt_i8_array__";
const LIBNT_I32_ARRAY: &str = "__libnbt_i32_array__";
const LIBNT_I64_ARRAY: &str = "__libnbt_i64_array__";

pub fn i8_array<T, S>(array: T, serial: S) -> Result<S::Ok, S::Error>
where
    T: IntoIterator,
    S: ser::Serializer,
    <T as IntoIterator>::Item: std::borrow::Borrow<i8>,
{
    array_serializer!("i8_array", array, serial)
}

pub fn i32_array<T, S>(array: T, serial: S) -> Result<S::Ok, S::Error>
where
    T: IntoIterator,
    S: ser::Serializer,
    <T as IntoIterator>::Item: std::borrow::Borrow<i32>,
{
    array_serializer!("i32_array", array, serial)
}

pub fn i64_array<T, S>(array: T, serial: S) -> Result<S::Ok, S::Error>
where
    T: IntoIterator,
    S: ser::Serializer,
    <T as IntoIterator>::Item: std::borrow::Borrow<i64>,
{
    array_serializer!("i64_array", array, serial)
}
