use std::{
    path::Path,
    rc::{Rc, Weak},
};

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
            EntryState::Directory(v) => v.selected = value,
            EntryState::File(v) => v.selected = value,
            EntryState::Symlink(v) => v.selected = value,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            EntryState::Directory(v) => &v.path,
            EntryState::File(v) => &v.path,
            EntryState::Symlink(v) => &v.path,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            EntryState::Directory(v) => v.selected,
            EntryState::File(v) => v.selected,
            EntryState::Symlink(v) => v.selected,
        }
    }

    pub fn move_down(&self) -> Option<Weak<EntryState>> {
        if let Some(parent) = match self {
            EntryState::Directory(v) => v.parent.upgrade(),
            EntryState::File(v) => v.parent.upgrade(),
            EntryState::Symlink(v) => v.parent.upgrade(),
        } {
            parent.move_down(self.path())
        } else {
            None
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
