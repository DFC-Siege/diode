use std::{
    ffi::OsString,
    fs::{self, Metadata},
    io::{self},
    path::PathBuf,
};

use crate::file_management::entry::Entry;

#[derive(Debug)]
pub struct Directory {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
}

impl Directory {
    pub fn load_entries(&self) -> io::Result<Vec<Entry>> {
        fs::read_dir(&self.path)?
            .map(|e| e.and_then(Entry::try_from))
            .collect()
    }

    pub fn get_parent_directory(&self) -> io::Result<Directory> {
        let mut path = self.path.to_owned();
        if !path.pop() {
            return Err(io::Error::other("already at root directory"));
        }

        path.try_into()
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata: fs::metadata(path)?,
        })
    }
}
