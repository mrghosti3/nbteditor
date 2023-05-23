use nbt::archive::enflate;
use std::{env, fs, io};

fn main() {
    let args = env::args()
        .skip(1)
        .next()
        .expect("Missing filename to NBT data.");
    let mut buf = open_file(&args);
    let root_tag = enflate::read_gzip_compound_tag(&mut buf).expect("Unable to decode!");

    println!("{}", serde_yaml::to_string(&root_tag).unwrap());
}

fn open_file(fname: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(fname).expect("Unable to open file");
    io::BufReader::new(file)
}
