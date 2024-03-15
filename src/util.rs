use std::io::{Read, Seek, Write};

use crate::{err, state};

/// Takes opened NBT file and outputs it into a file with name specified in `file` parameter.
///
/// # Errors
///
/// This function will return an error if .
///
/// TODO: Fill up the doc.
pub(crate) fn decompile<'a, W>(state: &'a mut state::IOManager<W>) -> err::Result<()>
where
    W: Write + Seek + Read,
{
    use nbt::archive::enflate::read_gzip_compound_tag;

    let mut breader = state.get_fin_reader()?;
    let root_tag = read_gzip_compound_tag(&mut breader)?;
    let mut out = state.get_fout_writer()?;

    crate::xml::print_xml(&mut out, &root_tag)
}

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn compile(state: &mut state::State) -> err::Result<()> {
    use nbt::archive::deflate;
    use nbt::CompoundTag;

    let file = state.get_yml_file_reader()?;
    let rtag: CompoundTag = serde_yaml::from_reader(file)?;

    let mut file = state.get_nbt_file_writer()?;
    Ok(deflate::write_gzip_compound_tag(&mut file, &rtag)?)
}
