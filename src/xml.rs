use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::io::{BufWriter, Write};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;

use nbt::{CompoundTag, Tag};

use crate::err;

type TagMap<'a> = (&'a String, &'a Tag);

enum NbtIter<'a> {
    Compound(Box<dyn DoubleEndedIterator<Item = TagMap<'a>> + 'a>),
    List(core::slice::Iter<'a, Tag>),
}

#[derive(Debug)]
enum NbtElement<'a> {
    TagMap(&'a str, &'a Tag),
    Tag(&'a Tag),
}

impl<'a> NbtIter<'a> {
    fn from_compound(ctag: &'a CompoundTag) -> Self {
        Self::Compound(Box::new(ctag.iter()))
    }
}

impl<'a> Iterator for NbtIter<'a> {
    type Item = NbtElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use NbtIter::*;

        match self {
            Compound(i) => i
                .next()
                .map(|(name, tag)| NbtElement::TagMap(name.as_str(), tag)),
            List(i) => i.next().map(|el| NbtElement::Tag(el)),
        }
    }
}

impl Debug for NbtIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::List(_) => "List",
            Self::Compound(_) => "Compound",
        };
        write!(f, "Nbt::{}", res)
    }
}

const TAG_COMPOUND: &'static str = "compound";
const TAG_NAME_ATTR: &'static str = "name";

pub fn print_xml<T: Write>(stream: &mut BufWriter<T>, rtag: &CompoundTag) -> err::Result<()> {
    let mut tag_stack: VecDeque<NbtIter> = VecDeque::with_capacity(512);
    tag_stack.push_back(NbtIter::from_compound(rtag));

    let mut writer = Writer::new_with_indent(stream, b' ', 4);

    writer.write_event(Event::Start({
        let mut elem = BytesStart::new(TAG_COMPOUND);
        if let Some(rtag_name) = &rtag.name {
            elem.push_attribute((TAG_NAME_ATTR, rtag_name.as_str()));
        }

        elem
    }))?;

    traverse_nbt(&mut writer, &mut tag_stack)?;

    writer.write_event(Event::Eof)?;

    Ok(())
}

fn traverse_nbt<W: Write>(
    writer: &mut Writer<W>,
    tag_stack: &mut VecDeque<NbtIter>,
) -> err::Result<()> {
    const TAG_BYTE: &'static str = "byte";
    const TAG_SHORT: &'static str = "short";
    const TAG_INT: &'static str = "int";
    const TAG_LONG: &'static str = "long";
    const TAG_FLOAT: &'static str = "float";
    const TAG_DOUBLE: &'static str = "double";
    const TAG_STRING: &'static str = "string";
    const TAG_BYTE_ARR: (&'static str, &'static str) = ("byte_array", TAG_BYTE);
    const TAG_INT_ARR: (&'static str, &'static str) = ("int_array", TAG_INT);
    const TAG_LONG_ARR: (&'static str, &'static str) = ("long_array", TAG_LONG);
    const TAG_LIST: &'static str = "list";

    while !tag_stack.is_empty() {
        let head = tag_stack.len() - 1;

        let (tname, tag) = match tag_stack[head].next() {
            None => {
                let xml_el = match tag_stack[head] {
                    NbtIter::List(_) => TAG_LIST,
                    NbtIter::Compound(_) => TAG_COMPOUND,
                };
                writer.write_event(Event::End(BytesEnd::new(xml_el)))?;

                tag_stack.pop_back();
                continue;
            }
            Some(NbtElement::Tag(tag)) => (None, tag),
            Some(NbtElement::TagMap(tname, tag)) => (Some(tname), tag),
        };

        let mut attr = None;
        if let Some(tname) = tname {
            attr = Some((TAG_NAME_ATTR, tname));
        }

        match tag {
            Tag::Byte(b) => {
                write_xml_el(writer, TAG_BYTE, attr, format!("{}", b).as_str())?;
            }
            Tag::Short(s) => {
                write_xml_el(writer, TAG_SHORT, attr, format!("{}", s).as_str())?;
            }
            Tag::Int(i) => {
                write_xml_el(writer, TAG_INT, attr, format!("{}", i).as_str())?;
            }
            Tag::Long(l) => {
                write_xml_el(writer, TAG_LONG, attr, format!("{}", l).as_str())?;
            }
            Tag::Float(f) => {
                write_xml_el(writer, TAG_FLOAT, attr, format!("{}", f).as_str())?;
            }
            Tag::Double(d) => {
                write_xml_el(writer, TAG_DOUBLE, attr, format!("{}", d).as_str())?;
            }
            Tag::String(s) => {
                write_xml_el(writer, TAG_STRING, attr, format!("{}", s).as_str())?;
            }

            Tag::ByteArray(vb) => {
                write_xml_array(writer, TAG_BYTE_ARR, attr, &vb)?;
            }
            Tag::IntArray(vi) => {
                write_xml_array(writer, TAG_INT_ARR, attr, &vi)?;
            }
            Tag::LongArray(vl) => {
                write_xml_array(writer, TAG_LONG_ARR, attr, &vl)?;
            }

            Tag::List(v) => {
                tag_stack.push_back(NbtIter::List(v.iter()));

                writer.write_event(Event::Start(
                    BytesStart::new(TAG_LIST).with_attributes(attr),
                ))?;
            }
            Tag::Compound(c) => {
                tag_stack.push_back(NbtIter::from_compound(c));

                writer.write_event(Event::Start(
                    BytesStart::new(TAG_COMPOUND).with_attributes(attr),
                ))?;
            }
        }
    }

    Ok(())
}

#[inline(always)]
fn write_xml_el<'a, W: Write>(
    writer: &mut Writer<W>,
    ttype: &'static str,
    attr: Option<(&'static str, &'a str)>,
    content: &'a str,
) -> err::Result<()> {
    let mut elem_w = writer.create_element(ttype);

    if let Some(attr) = attr {
        elem_w = elem_w.with_attributes([attr].into_iter());
    }

    elem_w.write_text_content(BytesText::new(content))?;

    Ok(())
}

#[inline(always)]
fn write_xml_array<'a, W: Write, T: Display>(
    writer: &mut Writer<W>,
    atypes: (&'static str, &'static str),
    attr: Option<(&'static str, &'a str)>,
    content: &'a [T],
) -> err::Result<()> {
    writer.write_event(Event::Start({
        let mut elem = BytesStart::new(atypes.0);
        if let Some(attr) = attr {
            elem = elem.with_attributes([attr].into_iter());
        }

        elem
    }))?;

    for c in content {
        write_xml_el(writer, atypes.1, None, format!("{}", c).as_str())?;
    }

    writer.write_event(Event::End(BytesEnd::new(atypes.0)))?;

    Ok(())
}
