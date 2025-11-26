use std::{
    ffi::OsString,
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
    rc::Weak,
};

use crate::file_management::entry::Entry;

#[derive(Debug)]
pub struct File {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub parent: Weak<Entry>,
}

impl File {
    pub fn try_from(entry: &DirEntry, parent: Weak<Entry>) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            parent,
        })
    }
}

impl TryFrom<&Path> for File {
    type Error = io::Error;

    fn try_from(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            parent: Weak::new(),
        })
    }
}

impl TryFrom<&PathBuf> for File {
    type Error = io::Error;

    fn try_from(path: &PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            parent: Weak::new(),
        })
    }
}
