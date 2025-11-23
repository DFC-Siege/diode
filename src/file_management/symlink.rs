use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Symlink {
    name: OsString,
    path: PathBuf,
    metadata: Metadata,
    target: PathBuf,
}

impl Symlink {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            target: fs::read_link(entry.path())?,
        })
    }
}
