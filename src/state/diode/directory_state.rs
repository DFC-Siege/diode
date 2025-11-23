use std::{ffi::OsString, fs::Metadata, path::PathBuf};

use crate::{file_management::directory::Directory, state::diode::entry_state::EntryState};

#[derive(Debug)]
pub struct DirectoryState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub entries: Vec<EntryState>,
    pub collapsed: bool,
    pub selected: bool,
}

impl From<Directory> for DirectoryState {
    fn from(directory: Directory) -> Self {
        Self {
            name: directory.name,
            path: directory.path,
            metadata: directory.metadata,
            entries: directory.entries.into_iter().map(|v| v.into()).collect(),
            collapsed: false,
            selected: false,
        }
    }
}
