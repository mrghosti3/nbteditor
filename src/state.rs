use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek};

use crate::err::Result;

/// struct used for storing global state of the program.
pub struct IOManager {
    fin: File,
    fout: File,
}

impl IOManager {
    pub(crate) fn new(cnf: &crate::cli::Config) -> Result<Self> {
        let fin = OpenOptions::new()
            .write(true)
            .read(true)
            .open(cnf.get_in_file())?;
        let fout = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(cnf.get_out_file())?;
        Ok(IOManager { fin, fout })
    }

    #[inline]
    pub(crate) fn get_fin_reader(&mut self) -> Result<BufReader<&mut File>> {
        self.fin.rewind()?;
        Ok(BufReader::new(&mut self.fin))
    }

    #[inline]
    pub(crate) fn get_fin_writer(&mut self) -> Result<BufWriter<&mut File>> {
        self.fin.rewind()?;
        Ok(BufWriter::new(&mut self.fin))
    }

    #[inline]
    pub(crate) fn get_fout_reader(&mut self) -> Result<BufReader<&mut File>> {
        self.fout.rewind()?;
        Ok(BufReader::new(&mut self.fout))
    }

    #[inline]
    pub(crate) fn get_fout_writer(&mut self) -> Result<BufWriter<&mut File>> {
        self.fout.rewind()?;
        Ok(BufWriter::new(&mut self.fout))
    }
}
