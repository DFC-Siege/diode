use std::{fs::DirEntry, io};

use crate::file_management::{directory::Directory, file::File};

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

impl TryFrom<DirEntry> for Entry {
    type Error = io::Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        Ok(match value.path().is_dir() {
            true => Entry::Directory(Directory::try_from(value.path())?),
            false => Entry::File(File::try_from(value.path())?),
        })
    }
}
