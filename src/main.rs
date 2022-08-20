use std::{io, fs};
use nbt::{Tag, CompoundTag};

fn main() {
    let mut buf = open_file("test/level.dat");
    let root_tag = nbt::decode::read_gzip_compound_tag(&mut buf).expect("Unable to decode!");

    print_tags(0, &root_tag);
}

fn open_file(fname: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(fname).expect("Unable to open file");
    io::BufReader::new(file)
}

/// Prints NBT data to stdout simirral to JSON format.
/// FIX: Move away from recursion.
fn print_tags(i: usize, root: &CompoundTag) {
    print_ctag(i, root);

    let indent = i + 2;
    for el in root.iter() {
        if let Tag::Compound(ctag) = el.1 {
            print_tags(indent, ctag);
        } else {
            print_tag(indent, el.0, el.1);
        }
    }

    if !root.is_empty() {
        println!("{}}}", " ".repeat(i));
    }
}

fn print_ctag(i: usize, ctag: &CompoundTag) {
    let space = " ".repeat(i);
    let name = match &ctag.name {
        Some(name) => name,
        None => ""
    };
    let curly = match ctag.is_empty() {
        true => "{}",
        false => "{"
    };
    println!("{space}\"{name}\" : {curly}");
}

fn print_tag(i: usize, name: &String, tag: &Tag) {
    let space = " ".repeat(i);
    let value = if let Tag::String(val) = tag {
        format!("\"{}\"", val)
    } else {
        tag.to_string()
    };
    println!("{space}\"{name}\" : {value}");
}
