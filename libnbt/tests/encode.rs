use nbt::{encode::*, CompoundTag, Tag};

#[test]
fn simple_encode() {
    let data = {
        let mut tmp = CompoundTag::new();
        tmp.push("HP".into(), Tag::Int(0x1B));
        tmp
    };

    let mut buf: Vec<u8> = Vec::with_capacity(13);
    write_nbt(&data, &mut buf).unwrap();

    // Compound Header check
    assert_eq!(&buf[..3], &[0x0A, 0, 0]);

    // Int Tag Header check
    assert_eq!(&buf[3..8], &[3, 0, 2, b'H', b'P']);
    assert_eq!(&buf[8..buf.len() - 1], &[0, 0, 0, 0x1B])
}

#[test]
fn simple_encode2() {
    let data = {
        let mut tmp = CompoundTag::new();
        tmp.push("HP".into(), Tag::Long(0x1B));
        tmp
    };

    let mut buf: Vec<u8> = Vec::with_capacity(13);
    write_nbt(&data, &mut buf).unwrap();

    // Compound Header check
    assert_eq!(&buf[..3], &[0x0A, 0, 0]);

    // Long Tag Header check
    assert_eq!(&buf[3..8], &[4, 0, 2, b'H', b'P']);
    assert_eq!(&buf[8..buf.len() - 1], &[0, 0, 0, 0, 0, 0, 0, 0x1B])
}
