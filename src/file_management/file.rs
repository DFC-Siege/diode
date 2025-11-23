use std::{
    ffi::{OsStr, OsString},
    fs::{DirEntry, Metadata},
    io::{self},
    path::PathBuf,
};

#[derive(Debug)]
pub struct File {
    name: OsString,
    path: PathBuf,
    metadata: Metadata,
}

impl File {
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
        })
    }
}
