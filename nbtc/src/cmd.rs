use std::io::{stdout, BufRead, BufReader, BufWriter, Write};

use crate::util::DataFormat;
use crate::{cli, err};

const HELP_TEXT: &[u8] = include_bytes!("help.txt");

pub(crate) fn help() -> err::Result<()> {
    stdout().write_all(HELP_TEXT)?;

    Ok(())
}

/// Takes opened NBT file and outputs it into a file with name specified in `file` parameter.
///
/// # Errors
///
/// This function will return an error if .
///
/// TODO: Fill up the doc.
pub(crate) fn decompile(config: &cli::Config) -> err::Result<()> {
    use nbt::decode::*;

    let mut fin = BufReader::new(config.get_in_file().to_file(false)?);

    let dformat = if config.get_data_format().is_default() {
        let buf_ref = fin.fill_buf()?;
        if buf_ref.len() < DataFormat::BYTE_COUNT {
            return Err(err::RuntimeErr::BadFileFormat {
                file_name: config.get_in_file().to_str(),
            });
        }

        let magic_bytes = &buf_ref[..DataFormat::BYTE_COUNT].try_into().unwrap();
        DataFormat::from_magic_bytes(&magic_bytes)
    } else {
        *config.get_data_format()
    };

    let root_tag = match dformat {
        DataFormat::Gzip => read_gzip_nbt(&mut fin)?,
        DataFormat::NBT => read_nbt(&mut fin)?,
        DataFormat::Zlib => read_zlib_nbt(&mut fin)?,
        _ => {
            return Err(err::RuntimeErr::BadFileFormat {
                file_name: config.get_in_file().to_str(),
            });
        }
    };

    let mut fout = BufWriter::new(config.get_out_file().to_file(true)?);

    crate::xml::write::print_xml(&mut fout, &root_tag)
}

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn compile(config: &cli::Config) -> err::Result<()> {
    use nbt::encode::*;

    let mut fin = BufReader::new(config.get_in_file().to_file(false)?);
    let nbt_data = crate::xml::read::read_xml(&mut fin)?;
    let mut fout = BufWriter::new(config.get_out_file().to_file(true)?);

    let dformat = if config.get_data_format().is_default() {
        DataFormat::NBT
    } else {
        *config.get_data_format()
    };

    match dformat {
        DataFormat::NBT => write_nbt(&nbt_data, &mut fout),
        DataFormat::Gzip => write_gzip_nbt(&nbt_data, &mut fout),
        DataFormat::Zlib => write_zlib_nbt(&nbt_data, &mut fout),
        _ => return Err(err::RuntimeErr::BadDataCompression(dformat as u8)),
    }
    .map_err(From::from)
}
