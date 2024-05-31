use std::collections::VecDeque;
use std::io::{BufReader, Read};
use std::str::from_utf8;

use nbt::{CompoundTag, Tag};
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;

use super::{consts::*, TagExtras};
use crate::err::{self, RuntimeErr};

pub fn read_xml<F: Read>(reader: &mut BufReader<F>) -> err::Result<CompoundTag> {
    let mut reader = Reader::from_reader(reader);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut el_buf: VecDeque<NbtElement> = VecDeque::with_capacity(10);
    let mut head = 0usize;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Eof => break,
            Event::Start(ev) => {
                let el = process_event(&ev);
                el_buf.push_back(el);
            }
            Event::End(_) => {
                if el_buf.len() < 2 {
                    break;
                }

                let temp = el_buf.pop_back().unwrap();
                head = el_buf.len() - 1;
                add_to_parent(&mut el_buf[head], temp)?;
            }
            Event::Text(text) => parse_text(text, &mut el_buf[head])?,
            _ => unimplemented!(),
        }

        head = el_buf.len() - 1;
        buf.clear();
    }

    match el_buf.pop_front().unwrap().tag {
        Tag::Compound(ctag) => Ok(CompoundTag::with(ctag)),
        _ => Err(RuntimeErr::NBTDecode(
            nbt::err::TagDecodeError::RootMustBeCompound,
        )),
    }
}

#[inline]
fn parse_text(text: BytesText, el: &mut NbtElement) -> Result<(), RuntimeErr> {
    let inner = from_utf8(&text)?;

    match &mut el.tag {
        Tag::String(text) => *text = inner.into(),
        Tag::Byte(byte) => *byte = inner.parse()?,
        Tag::Short(short) => *short = inner.parse()?,
        Tag::Int(int) => *int = inner.parse()?,
        Tag::Long(long) => *long = inner.parse()?,
        Tag::Float(float) => *float = inner.parse()?,
        Tag::Double(double) => *double = inner.parse()?,
        _ => unreachable!(),
    };

    Ok(())
}

fn process_event(ev: &BytesStart) -> NbtElement {
    let name: Option<Box<str>> = ev
        .attributes()
        .filter(|at| {
            let at = match at {
                Ok(at) => at,
                Err(_) => return false,
            };

            if at.key.as_ref() == TAG_NAME_ATTR.as_bytes() {
                return true;
            }

            false
        })
        .take(1)
        .map(|at| from_utf8(at.unwrap().value.as_ref()).unwrap().into())
        .next();

    let tag = match ev.name().as_ref() {
        TAG_BYTE_B => Tag::Byte(0),
        TAG_SHORT_B => Tag::Short(0),
        TAG_INT_B => Tag::Int(0),
        TAG_LONG_B => Tag::Long(0),
        TAG_FLOAT_B => Tag::Float(0.0),
        TAG_DOUBLE_B => Tag::Double(0.0),
        TAG_STRING_B => Tag::String("".into()),
        TAG_LIST_B => Tag::List([].into()),
        TAG_COMPOUND_B => Tag::Compound(Default::default()),
        TAG_BYTE_ARR_B => Tag::ByteArray([].into()),
        TAG_INT_ARR_B => Tag::IntArray([].into()),
        TAG_LONG_ARR_B => Tag::LongArray([].into()),
        _ => unreachable!(),
    };

    NbtElement { tag, name }
}

#[inline]
fn add_to_parent(parent: &mut NbtElement, el: NbtElement) -> err::Result<()> {
    let current_tag_tname = el.tag.type_name();

    match (&mut parent.tag, el.tag) {
        (Tag::Compound(ctag), tag) => {
            ctag.insert(el.name.unwrap(), tag);
        }
        (Tag::List(ltag), tag) => ltag.push(tag),
        (Tag::ByteArray(array), Tag::Byte(val)) => array.push(val),
        (Tag::IntArray(array), Tag::Int(val)) => array.push(val),
        (Tag::LongArray(array), Tag::Long(val)) => array.push(val),
        _ => {
            return Err(RuntimeErr::XmlError(quick_xml::Error::UnexpectedToken(
                format!("<{}>", current_tag_tname),
            )));
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct NbtElement {
    tag: Tag,
    name: Option<Box<str>>,
}
