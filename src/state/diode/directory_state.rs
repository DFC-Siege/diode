use std::{
    ffi::OsString,
    fs::Metadata,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{file_management::directory::Directory, state::diode::entry_state::EntryState};

#[derive(Debug)]
pub struct DirectoryState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub entries: Vec<Rc<EntryState>>,
    pub collapsed: bool,
    pub selected: bool,
    pub parent: Weak<EntryState>,
}

impl From<Directory> for DirectoryState {
    fn from(directory: Directory) -> Self {
        let parent: Weak<EntryState> = match directory.parent.upgrade() {
            Some(v) => Rc::downgrade(&Rc::new(EntryState::from(v))),
            None => Weak::new(),
        };
        Self {
            name: directory.name,
            path: directory.path,
            metadata: directory.metadata,
            entries: directory
                .entries
                .into_iter()
                .map(|v| Rc::new(v.into()))
                .collect(),
            collapsed: true,
            selected: false,
            parent,
        }
    }
}
