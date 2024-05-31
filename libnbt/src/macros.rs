macro_rules! return_expr_for_serialized_types_method {
    ($rtrn:expr, $func:ident($($arg:ty),*)) => {
        #[inline]
        fn $func(self, $(_: $arg,)*) -> ::std::result::Result<Self::Ok, Self::Error> {
            $rtrn
        }
    };
    ($rtrn:expr, $func:ident($($arg:ty),*), result: $result:path) => {
        #[inline]
        fn $func(self, $(_: $arg,)*) -> ::std::result::Result<$result, Self::Error>
        {
            $rtrn
        }
    };
    ($rtrn:expr, $func:ident($($arg:ty),*), where: $where:path) => {
        #[inline]
        fn $func<__T: ?Sized>(self, $(_: $arg,)*) -> ::std::result::Result<Self::Ok, Self::Error>
        where __T: $where
        {
            $rtrn
        }
    };
}

macro_rules! return_expr_for_serialized_types_helper {
    ($rtrn:expr, bool) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_bool(bool)}
    };
    ($rtrn:expr, i8) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_i8(i8)}
    };
    ($rtrn:expr, i16) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_i16(i16)}
    };
    ($rtrn:expr, i32) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_i32(i32)}
    };
    ($rtrn:expr, i64) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_i64(i64)}
    };
    ($rtrn:expr, u8) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_u8(u8)}
    };
    ($rtrn:expr, u8) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_u8(u8)}
    };
    ($rtrn:expr, u16) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_u16(u16)}
    };
    ($rtrn:expr, u32) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_u32(u32)}
    };
    ($rtrn:expr, u64) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_u64(u64)}
    };
    ($rtrn:expr, f32) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_f32(f32)}
    };
    ($rtrn:expr, f64) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_f64(f64)}
    };
    ($rtrn:expr, char) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_char(char)}
    };
    ($rtrn:expr, str) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_str(&str)}
    };
    ($rtrn:expr, bytes) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_bytes(&[u8])}
    };
    ($rtrn:expr, none) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_none()}
    };
    ($rtrn:expr, unit) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_unit()}
    };
    ($rtrn:expr, unit_struct) => {
        return_expr_for_serialized_types_method! {$rtrn, serialize_unit_struct(&'static str)}
    };
    ($rtrn:expr, unit_variant) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_unit_variant(&'static str, u32, &'static str)
        }
    };
    ($rtrn:expr, some) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_some(&__T),
            where: ::serde::ser::Serialize
        }
    };
    ($rtrn:expr, newtype_struct) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_newtype_struct(&'static str, &__T),
            where: ::serde::ser::Serialize
        }
    };
    ($rtrn:expr, newtype_variant) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_newtype_variant(&'static str, u32, &'static str, &__T),
            where: ::serde::ser::Serialize
        }
    };
    ($rtrn:expr, seq) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_seq(Option<usize>),
            result: Self::SerializeSeq
        }
    };
    ($rtrn:expr, tuple) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_tuple(usize),
            result: Self::SerializeTuple
        }
    };
    ($rtrn:expr, tuple_struct) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_tuple_struct(&'static str, usize),
            result: Self::SerializeTupleStruct
        }
    };
    ($rtrn:expr, tuple_variant) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_tuple_variant(&'static str, u32, &'static str, usize),
            result: Self::SerializeTupleVariant
        }
    };
    ($rtrn:expr, map) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_map(Option<usize>),
            result: Self::SerializeMap
        }
    };
    ($rtrn:expr, struct) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_struct(&'static str, usize),
            result: Self::SerializeStruct
        }
    };
    ($rtrn:expr, struct_variant) => {
        return_expr_for_serialized_types_method! {
            $rtrn,
            serialize_struct_variant(&'static str, u32, &'static str, usize),
            result: Self::SerializeStructVariant
        }
    };
}

macro_rules! return_expr_for_serialized_types {
    ($rtrn:expr; $($type:tt)*) => {
        $(return_expr_for_serialized_types_helper!{$rtrn, $type})*
    };
}

macro_rules! unrepresentable {
    ($($type:tt)*) => {
        $(return_expr_for_serialized_types_helper!{Err(TagEncodeError::UnrepresentableType(stringify!($type))), $type})*
    };
}

/// Serde `serialize_with` implementation for array serialization.
///
/// This macro provides the function body for `i8_array`, `i32_array` and `i64_array`
/// in [`self::ser`], providing NBT `ByteArray`, `IntArray` and `LongArray`
/// serialization with serde.
macro_rules! array_serializer {
    ($func_name:literal, $arr: ident, $serializer: ident) => {{
        use serde::ser::SerializeTupleStruct;
        use serde::ser::Error as SerError;
        use std::borrow::Borrow;

        let error = concat!(
            $func_name,
            " serializer may only be used with known-length collections"
        );
        let magic = concat!("__libnbt_", $func_name, "__");

        let mut iter = $arr.into_iter();
        let (length, max_length) = iter.size_hint();

        if max_length.is_none() || length != max_length.unwrap() {
            return Err(SerError::custom(error));
        }

        let mut seq = $serializer.serialize_tuple_struct(magic, length)?;
        for _i in 0..length {
            seq.serialize_field(iter.next().ok_or_else(|| SerError::custom(error))?.borrow())?;
        }

        if iter.next().is_some() {
            Err(SerError::custom(error))
        } else {
            seq.end()
        }
    }};
}
