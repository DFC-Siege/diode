use std::{
    cell::Cell,
    ffi::OsString,
    fs::Metadata,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{file_management::file::File, state::diode::directory_state::DirectoryState};

#[derive(Debug)]
pub struct FileState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub selected: Cell<bool>,
    pub parent: Weak<DirectoryState>,
}

impl From<File> for FileState {
    fn from(file: File) -> Self {
        let parent: Weak<DirectoryState> = match file.parent.upgrade() {
            Some(v) => Rc::downgrade(&Rc::new(DirectoryState::from(v))),
            None => Weak::new(),
        };

        Self {
            name: file.name,
            path: file.path,
            metadata: file.metadata,
            selected: Cell::new(false),
            parent,
        }
    }
}
