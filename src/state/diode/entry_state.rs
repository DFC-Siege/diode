use std::{ffi::OsStr, path::Path};

use crate::{
    file_management::entry::Entry,
    state::diode::{directory_state::DirectoryState, file_state::FileState},
};

#[derive(Debug, Clone)]
pub enum EntryState {
    Directory(DirectoryState),
    File(FileState),
}

impl EntryState {
    pub fn path(&self) -> &Path {
        match self {
            EntryState::Directory(v) => &v.directory.path,
            EntryState::File(v) => &v.file.path,
        }
    }

    pub fn name(&self) -> &OsStr {
        match self {
            EntryState::Directory(v) => &v.directory.name,
            EntryState::File(v) => &v.file.name,
        }
    }

    pub fn get_indent(&self, base_path: &Path) -> u8 {
        let path = self.path();
        path.strip_prefix(base_path)
            .map(|p| p.components().count() as u8)
            .unwrap_or(0)
    }

    pub fn set_selected(&mut self, value: bool) {
        match self {
            EntryState::Directory(v) => v.selected = value,
            EntryState::File(v) => v.selected = value,
        }
    }

    pub fn toggle_marked(&mut self) {
        match self {
            EntryState::Directory(v) => v.marked = !v.marked,
            EntryState::File(v) => v.marked = !v.marked,
        }
    }
}

impl PartialEq for EntryState {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl From<Entry> for EntryState {
    fn from(entry: Entry) -> Self {
        match entry {
            Entry::Directory(v) => EntryState::Directory(DirectoryState::from(v)),
            Entry::File(v) => EntryState::File(FileState::from(v)),
        }
    }
}
