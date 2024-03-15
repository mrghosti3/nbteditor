use std::fs::File;
use std::io::{stdout, BufReader, BufWriter};
use std::os::fd::{AsRawFd, FromRawFd};

use crate::{cli, err};

/// Takes opened NBT file and outputs it into a file with name specified in `file` parameter.
///
/// # Errors
///
/// This function will return an error if .
///
/// TODO: Fill up the doc.
pub(crate) fn decompile(config: &cli::Config) -> err::Result<()> {
    use cli::FdArgument as FdArg;
    use nbt::archive::enflate::read_gzip_compound_tag;

    let mut fin = BufReader::new(File::open(config.get_in_file())?);
    let fout = match config.get_out_file() {
        FdArg::StdIO => unsafe { File::from_raw_fd(stdout().as_raw_fd()) },
        FdArg::File(ref file_name) => File::create(file_name.as_ref())?,
    };

    let mut fout = BufWriter::new(fout);

    let root_tag = read_gzip_compound_tag(&mut fin)?;

    crate::xml::print_xml(&mut fout, &root_tag)
}

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn compile(config: &cli::Config) -> err::Result<()>
{
    todo!("To be implemented")
}
