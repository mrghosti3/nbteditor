pub(crate) const TAG_END: u8 = 0;
pub(crate) const TAG_BYTE: u8 = 1;
pub(crate) const TAG_SHORT: u8 = 2;
pub(crate) const TAG_INT: u8 = 3;
pub(crate) const TAG_LONG: u8 = 4;
pub(crate) const TAG_FLOAT: u8 = 5;
pub(crate) const TAG_DOUBLE: u8 = 6;
pub(crate) const TAG_BYTE_ARRAY: u8 = 7;
pub(crate) const TAG_STRING: u8 = 8;
pub(crate) const TAG_LIST: u8 = 9;
pub(crate) const TAG_COMPOUND: u8 = 10;
pub(crate) const TAG_INT_ARRAY: u8 = 11;
pub(crate) const TAG_LONG_ARRAY: u8 = 12;

pub(crate) const TAGS: [&str; 13] = [
    "TAG_END",
    "TAG_BYTE",
    "TAG_SHORT",
    "TAG_INT",
    "TAG_LONG",
    "TAG_FLOAT",
    "TAG_DOUBLE",
    "TAG_BYTE_ARRAY",
    "TAG_INT_ARRAY",
    "TAG_LONG_ARRAY",
    "TAG_STRING",
    "TAG_LIST",
    "TAG_COMPOUND",
];

macro_rules! tag_name {
    [$offset:expr] => {
        TAGS[$offset as usize]
    };
}

pub(crate) use tag_name;
