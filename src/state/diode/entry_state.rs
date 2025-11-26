use std::rc::Rc;

use crate::{
    file_management::entry::Entry,
    state::diode::{
        directory_state::DirectoryState, file_state::FileState, symlink_state::SymlinkState,
    },
};

#[derive(Debug)]
pub enum EntryState {
    Directory(DirectoryState),
    File(FileState),
    Symlink(SymlinkState),
}

impl EntryState {
    pub fn set_selected(&mut self, value: bool) {
        match self {
            EntryState::Directory(d) => d.selected = value,
            EntryState::File(f) => f.selected = value,
            EntryState::Symlink(s) => s.selected = value,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            EntryState::Directory(d) => d.selected,
            EntryState::File(f) => f.selected,
            EntryState::Symlink(s) => s.selected,
        }
    }
}

impl From<Entry> for EntryState {
    fn from(entry: Entry) -> Self {
        match entry {
            Entry::Directory(d) => EntryState::Directory(d.into()),
            Entry::File(f) => EntryState::File(f.into()),
            Entry::Symlink(s) => EntryState::Symlink(s.into()),
        }
    }
}

impl From<Rc<Entry>> for EntryState {
    fn from(entry: Rc<Entry>) -> Self {
        match Rc::try_unwrap(entry).expect("Entry has multiple strong references") {
            Entry::Directory(d) => EntryState::Directory(d.into()),
            Entry::File(f) => EntryState::File(f.into()),
            Entry::Symlink(s) => EntryState::Symlink(s.into()),
        }
    }
}
