use std::{
    ffi::OsStr,
    fs::DirEntry,
    io::{self},
    path::PathBuf,
};

use crate::file_management::{directory::Directory, file::File, symlink::Symlink};

#[derive(Debug)]
pub enum EntryType {
    File(File),
    Directory(Directory),
    Symlink(Symlink),
}

impl EntryType {
    pub fn name(&self) -> &OsStr {
        match self {
            EntryType::File(v) => v.name(),
            EntryType::Directory(v) => v.name(),
            EntryType::Symlink(v) => v.name(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            EntryType::File(v) => v.path(),
            EntryType::Directory(v) => v.path(),
            EntryType::Symlink(v) => v.path(),
        }
    }

    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        let file_type = entry.file_type()?;
        match (
            file_type.is_file(),
            file_type.is_dir(),
            file_type.is_symlink(),
        ) {
            (true, _, _) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, true, _) => Ok(EntryType::Directory(Directory::try_from_recursive(
                entry, 0, 0,
            )?)),
            (_, _, true) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, _, _) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file type not supported",
            )),
        }
    }

    pub fn try_from_recursive(
        entry: &DirEntry,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Self> {
        let file_type = entry.file_type()?;
        match (
            file_type.is_file(),
            file_type.is_dir(),
            file_type.is_symlink(),
        ) {
            (true, _, _) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, true, _) => Ok(EntryType::Directory(Directory::try_from_recursive(
                entry,
                max_depth,
                current_depth + 1,
            )?)),
            (_, _, true) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, _, _) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file type not supported",
            )),
        }
    }
}
