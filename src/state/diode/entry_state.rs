use std::path::Path;

use crate::{
    file_management::entry::Entry,
    state::diode::{directory_state::DirectoryState, file_state::FileState},
};

#[derive(Debug)]
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

    pub fn set_selected(&mut self, value: bool) {
        match self {
            EntryState::Directory(v) => v.selected = value,
            EntryState::File(v) => v.selected = value,
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
