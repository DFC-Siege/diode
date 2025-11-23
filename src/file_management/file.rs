use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
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

impl TryFrom<&Path> for File {
    type Error = io::Error;

    fn try_from(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
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
        })
    }
}
