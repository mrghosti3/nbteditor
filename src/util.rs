use std::io::{Read, Seek, Write};

use crate::{err, state};

/// Takes opened NBT file and outputs it into a file with name specified in `file` parameter.
///
/// # Errors
///
/// This function will return an error if .
///
/// TODO: Fill up the doc.
pub(crate) fn decompile(state: &mut state::State) -> err::Result<()> {
    use nbt::archive::enflate;

    let mut buf = state.get_nbt_file_reader()?;
    let root_tag = enflate::read_gzip_compound_tag(&mut buf)?;
    drop(&buf);

    let out_fd = state.get_yml_file_writer()?;
    Ok(serde_yaml::to_writer(out_fd, &root_tag)?)
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
