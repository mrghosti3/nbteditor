use std::io::Cursor;

use nbt::{decode::*, Tag};

#[test]
fn simple_decode() {
    let mut buff = Cursor::new([0x0A, 0, 0, 3, 0, 2, b'H', b'P', 0, 0, 0, 0x1B, 0]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(data.get("HP"), Some(&Tag::Int(0x1B)));
}

#[test]
fn simple_decode2() {
    let mut buff = Cursor::new([
        0x0A, 0, 0, 4, 0, 2, b'H', b'P', 0, 0, 0, 0, 0, 0, 0, 0x1B, 0,
    ]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(data.get("HP"), Some(&Tag::Long(0x1B)));
}

#[test]
fn simple_decode_byte_seq() {
    let mut buff = Cursor::new([
        0x0A, 0, 0, 7, 0, 3, b'b', b'u', b'f', // headers
        0, 0, 0, 4, // length
        0, 0, 0x1B, 0, // Byte tags
        0, // NBT end
    ]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(data.get("buf"), Some(&Tag::ByteArray(vec![0, 0, 0x1B, 0])));
}

#[test]
fn simple_decode_byte_seq_empty() {
    let mut buff = Cursor::new([
        0x0A, 0, 0, 7, 0, 3, b'b', b'u', b'f', //headers
        0, 0, 0, 0, // length 0
        0, // NBT end
    ]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(data.get("buf"), Some(&Tag::ByteArray(vec![])));
}

#[test]
fn simple_decode_list_seq() {
    let mut buff = Cursor::new([
        0x0A, 0, 0, 9, 0, 8, b'D', b'i', b's', b'a', b'b', b'l', b'e', b'd', // headers
        2,    // TAG ID for TAG_Short
        0, 0, 0, 3, // Length
        0, 1, 0, 2, 0, 3, // 3 2 byte sized data
        0, // NBT end
    ]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(
        data.get("Disabled"),
        Some(&Tag::List(vec![
            Tag::Short(1),
            Tag::Short(2),
            Tag::Short(3)
        ]))
    );
}

#[test]
fn simple_decode_list_seq_empty() {
    let mut buff = Cursor::new([
        0x0A, 0, 0, 9, 0, 8, b'D', b'i', b's', b'a', b'b', b'l', b'e', b'd', 0, // No TAG ID
        0, 0, 0, 0, // Length 0
        0, // NBT end
    ]);
    let data = read_nbt(&mut buff).unwrap();
    assert_eq!(data.name.as_deref(), None);
    assert_eq!(data.get("Disabled"), Some(&Tag::List(vec![])));
}
