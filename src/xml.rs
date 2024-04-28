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
}
