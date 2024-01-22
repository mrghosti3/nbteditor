use std::path::Path;

use crate::err;
use crate::state;

/// Creates new filename to for YAML file that contains desrialised NBT file
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn make_fname(ifile: &Path) -> Result<String, err::MyError> {
    if !ifile.is_file() {
        return Result::Err(err::MyError::BadFilePath);
    }

    let mut ofile: String = ifile.to_str().ok_or(err::MyError::BadFilePath)?.into();
    ofile.push_str(".yml");

    Ok(ofile)
}

use nix::sys::inotify::{AddWatchFlags, Inotify, WatchDescriptor};

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn inotify_init() -> nix::Result<Inotify> {
    use nix::sys::inotify::InitFlags;

    Inotify::init(InitFlags::empty())
}

/// TODO: Fill me
///
/// # Errors
///
/// This function will return an error if .
pub(crate) fn inotify_add_watch(inotif: &Inotify, fname: &str) -> nix::Result<WatchDescriptor> {
    inotif.add_watch(fname, AddWatchFlags::IN_MODIFY)
}

/// Takes given `rtag` and outputs it into a yaml file with name specified in `file` parameter.
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
