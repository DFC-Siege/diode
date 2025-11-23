use std::{ffi::OsString, fs::Metadata, path::PathBuf};

use crate::file_management::file::File;

#[derive(Debug)]
pub struct FileState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub selected: bool,
}

impl From<File> for FileState {
    fn from(file: File) -> Self {
        Self {
            name: file.name,
            path: file.path,
            metadata: file.metadata,
            selected: false,
        }
    }
}
