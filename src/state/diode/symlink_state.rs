use std::{
    ffi::OsString,
    fs::Metadata,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::{file_management::symlink::Symlink, state::diode::directory_state::DirectoryState};

#[derive(Debug)]
pub struct SymlinkState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub target: PathBuf,
    pub collapsed: bool,
    pub selected: bool,
    pub parent: Weak<DirectoryState>,
}

impl From<Symlink> for SymlinkState {
    fn from(symlink: Symlink) -> Self {
        let parent: Weak<DirectoryState> = match symlink.parent.upgrade() {
            Some(v) => Rc::downgrade(&Rc::new(DirectoryState::from(v))),
            None => Weak::new(),
        };

        Self {
            name: symlink.name,
            path: symlink.path,
            metadata: symlink.metadata,
            target: symlink.target,
            collapsed: false,
            selected: false,
            parent,
        }
    }
}
