use std::{ffi::OsString, fs::Metadata, path::PathBuf};

use crate::file_management::symlink::Symlink;

#[derive(Debug)]
pub struct SymlinkState {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub target: PathBuf,
    pub collapsed: bool,
    pub selected: bool,
}

impl From<Symlink> for SymlinkState {
    fn from(symlink: Symlink) -> Self {
        Self {
            name: symlink.name,
            path: symlink.path,
            metadata: symlink.metadata,
            target: symlink.target,
            collapsed: false,
            selected: false,
        }
    }
}
