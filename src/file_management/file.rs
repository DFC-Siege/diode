use std::{
    ffi::OsString,
    fs::{self, Metadata},
    io::{self},
    path::PathBuf,
};

#[derive(Debug)]
pub struct File {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
}

impl TryFrom<PathBuf> for File {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata: fs::metadata(path)?,
        })
    }
}
