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
    pub parent: Weak<DirectoryState>,
}

impl DirectoryState {
    pub fn move_down(&self, entry: &EntryState) -> Option<Weak<EntryState>> {
        todo!()
    }
}

impl From<Directory> for DirectoryState {
    fn from(directory: Directory) -> Self {
        let parent: Weak<DirectoryState> = match directory.parent.upgrade() {
            Some(v) => Rc::downgrade(&Rc::new(DirectoryState::from(v))),
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

impl From<Rc<Directory>> for DirectoryState {
    fn from(entry: Rc<Directory>) -> Self {
        Rc::try_unwrap(entry)
            .expect("Directory has multiple strong references")
            .into()
    }
}
