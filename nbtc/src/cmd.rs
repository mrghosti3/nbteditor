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
    use nbt::archive::enflate::read_gzip_compound_tag;
    use nbt::decode::read_compound_tag;

    let mut fin = BufReader::new(config.get_in_file().to_file(false)?);
    let magic_bytes = &fin.fill_buf()?[..DataFormat::BYTE_COUNT]
        .try_into()
        .unwrap();

    let root_tag = match DataFormat::from_magic_bytes(&magic_bytes) {
        DataFormat::Gzip => read_gzip_compound_tag(&mut fin)?,
        DataFormat::NBT => read_compound_tag(&mut fin)?,
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
    use nbt::encode::write_compound_tag;

    let mut fin = BufReader::new(config.get_in_file().to_file(false)?);
    let nbt_data = crate::xml::read::read_xml(&mut fin)?;
    let mut fout = BufWriter::new(config.get_out_file().to_file(true)?);

    write_compound_tag(&mut fout, &nbt_data)?;
    Ok(())
}
