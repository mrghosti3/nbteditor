use std::{io, fs};

fn main() {
    let mut buf = open_file("test/level.dat");
    let root_tag = nbt::decode::read_gzip_compound_tag(&mut buf).expect("Unable to decode!");
    println!("Tag: {:?}", root_tag);
}

fn open_file(fname: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(fname).expect("Unable to open file");
    io::BufReader::new(file)
}
