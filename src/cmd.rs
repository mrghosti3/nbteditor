use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter};
use std::os::fd::{AsRawFd, FromRawFd};

use crate::cli::FdArgument as FdArg;
use crate::util::DataFormat;
use crate::{cli, err};

/// Takes opened NBT file and outputs it into a file with name specified in `file` parameter.
///
/// # Errors
///
/// This function will return an error if .
///
/// TODO: Fill up the doc.
pub(crate) fn decompile(config: &cli::Config) -> err::Result<()> {
    use nbt::archive::enflate::read_gzip_compound_tag;
    use nbt::decode::read_compound_tag;

    let mut fin = BufReader::new(File::open(config.get_in_file())?);
    let magic_bytes = &fin.fill_buf()?[..DataFormat::BYTE_COUNT]
        .try_into()
        .unwrap();

    let root_tag = match DataFormat::from_magic_bytes(&magic_bytes) {
        DataFormat::Gzip => read_gzip_compound_tag(&mut fin)?,
        DataFormat::NBT => read_compound_tag(&mut fin)?,
        _ => {
            let tmp = Box::from(config.get_in_file());
            return Err(err::RuntimeErr::BadFileFormat {
                file_name: Box::leak(tmp),
            });
        }
    };

    let fout = match config.get_out_file() {
        FdArg::StdIO => unsafe { File::from_raw_fd(stdout().as_raw_fd()) },
        FdArg::File(ref file_name) => File::create(file_name.as_ref())?,
    };

    let mut fout = BufWriter::new(fout);

    crate::xml::write::print_xml(&mut fout, &root_tag)
}

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn compile(config: &cli::Config) -> err::Result<()> {
    use nbt::encode::write_compound_tag;

    let mut fin = BufReader::new(File::open(config.get_in_file())?);
    let nbt_data = crate::xml::read::read_xml(&mut fin)?;
    let mut fout = BufWriter::new(unsafe {
        File::from_raw_fd(stdout().as_raw_fd())
    });

    write_compound_tag(&mut fout, &nbt_data)?;
    Ok(())
}
