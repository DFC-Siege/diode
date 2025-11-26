use std::{
    ffi::OsString,
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
    rc::Weak,
};

use crate::file_management::directory::Directory;

#[derive(Debug)]
pub struct Symlink {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub target: PathBuf,
    pub parent: Weak<Directory>,
}

impl Symlink {
    pub fn try_from(entry: &DirEntry, parent: Weak<Directory>) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            target: fs::read_link(entry.path())?,
            parent,
        })
    }
}

impl TryFrom<&Path> for Symlink {
    type Error = io::Error;

    fn try_from(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            target: fs::read_link(path)?,
            parent: Weak::new(),
        })
    }
}

impl TryFrom<&PathBuf> for Symlink {
    type Error = io::Error;

    fn try_from(path: &PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            target: fs::read_link(path)?,
            parent: Weak::new(),
        })
    }
}
