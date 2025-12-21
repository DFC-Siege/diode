use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

use crate::file_management::{directory::Directory, file::File};

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

pub fn move_entry(current: &Path, destination: &Path) -> io::Result<()> {
    let dir = if destination.is_file() {
        let Some(dir) = destination.parent() else {
            return Err(io::Error::other("Destination is file but has no parent?"));
        };
        dir
    } else {
        destination
    };

    let filename = current
        .file_name()
        .ok_or_else(|| io::Error::other("Source path has no filename"))?;

    fs::rename(current, dir.join(filename))
}

impl TryFrom<DirEntry> for Entry {
    type Error = io::Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        Ok(match value.file_type()?.is_dir() {
            true => Entry::Directory(Directory::try_from(value.path())?),
            false => Entry::File(File::try_from(value.path())?),
        })
    }
}
