use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek};

/// struct used for storing global state of the program.
pub struct State {
    f_nbt: fs::File,
    f_yml: fs::File,
}

impl State {
    pub(crate) fn new(f_nbt: &str, f_yml: &str) -> io::Result<Self> {
        let f_nbt = OpenOptions::new().write(true).read(true).open(f_nbt)?;
        let f_yml = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(f_yml)?;
        Ok(State { f_nbt, f_yml })
    }

    pub(crate) fn get_nbt_file_reader(&mut self) -> io::Result<BufReader<&mut fs::File>> {
        self.f_nbt.rewind()?;
        Ok(BufReader::new(&mut self.f_nbt))
    }

    pub(crate) fn get_nbt_file_writer(&mut self) -> io::Result<BufWriter<&mut fs::File>> {
        self.f_nbt.rewind()?;
        Ok(BufWriter::new(&mut self.f_nbt))
    }

    pub(crate) fn get_yml_file_reader(&mut self) -> io::Result<BufReader<&mut fs::File>> {
        self.f_yml.rewind()?;
        Ok(BufReader::new(&mut self.f_yml))
    }

    pub(crate) fn get_yml_file_writer(&mut self) -> io::Result<BufWriter<&mut fs::File>> {
        self.f_yml.rewind()?;
        Ok(BufWriter::new(&mut self.f_yml))
    }
}
