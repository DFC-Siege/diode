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
        todo!()
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
