pub mod read;
pub mod write;

mod consts {
    pub(super) const TAG_BYTE: &'static str = "byte";
    pub(super) const TAG_SHORT: &'static str = "short";
    pub(super) const TAG_INT: &'static str = "int";
    pub(super) const TAG_LONG: &'static str = "long";
    pub(super) const TAG_FLOAT: &'static str = "float";
    pub(super) const TAG_DOUBLE: &'static str = "double";
    pub(super) const TAG_STRING: &'static str = "string";
    pub(super) const TAG_BYTE_ARR: (&'static str, &'static str) = ("byte_array", TAG_BYTE);
    pub(super) const TAG_INT_ARR: (&'static str, &'static str) = ("int_array", TAG_INT);
    pub(super) const TAG_LONG_ARR: (&'static str, &'static str) = ("long_array", TAG_LONG);
    pub(super) const TAG_LIST: &'static str = "list";
    pub(super) const TAG_COMPOUND: &'static str = "compound";
    pub(super) const TAG_NAME_ATTR: &'static str = "name";

    pub(super) const TAG_BYTE_B: &[u8] = b"byte";
    pub(super) const TAG_SHORT_B: &[u8] = b"short";
    pub(super) const TAG_INT_B: &[u8] = b"int";
    pub(super) const TAG_LONG_B: &[u8] = b"long";
    pub(super) const TAG_FLOAT_B: &[u8] = b"float";
    pub(super) const TAG_DOUBLE_B: &[u8] = b"double";
    pub(super) const TAG_STRING_B: &[u8] = b"string";
    pub(super) const TAG_LIST_B: &[u8] = b"list";
    pub(super) const TAG_COMPOUND_B: &[u8] = b"compound";
    pub(super) const TAG_BYTE_ARR_B: &[u8] = b"byte_array";
    pub(super) const TAG_INT_ARR_B: &[u8] = b"int_array";
    pub(super) const TAG_LONG_ARR_B: &[u8] = b"long_array";
}

trait TagExtras {
    fn type_name(&self) -> &'static str;
}

impl TagExtras for nbt::Tag {
    fn type_name(&self) -> &'static str {
        use self::consts::*;

        match self {
            Self::Byte(_) => TAG_BYTE,
            Self::Short(_) => TAG_SHORT,
            Self::Int(_) => TAG_INT,
            Self::Long(_) => TAG_LONG,
            Self::Float(_) => TAG_FLOAT,
            Self::Double(_) => TAG_DOUBLE,
            Self::String(_) => TAG_STRING,

            Self::ByteArray(_) => TAG_BYTE_ARR.0,
            Self::IntArray(_) => TAG_INT_ARR.0,
            Self::LongArray(_) => TAG_LONG_ARR.0,

            Self::List(_) => TAG_LIST,
            Self::Compound(_) => TAG_COMPOUND,
        }
    }
}
