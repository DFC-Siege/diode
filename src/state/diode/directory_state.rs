use std::io;

use crate::{file_management::directory::Directory, state::diode::entry_state::EntryState};

#[derive(Debug, Clone)]
pub struct DirectoryState {
    pub directory: Directory,
    pub collapsed: bool,
    pub focussed: bool,
}

impl DirectoryState {
    pub fn new(directory: Directory) -> Self {
        Self {
            directory,
            collapsed: true,
            focussed: false,
        }
    }

    pub fn load_entry_states(&self) -> io::Result<Vec<EntryState>> {
        Ok(self
            .directory
            .load_entries()?
            .into_iter()
            .map(|v| v.into())
            .collect())
    }
}

impl From<Directory> for DirectoryState {
    fn from(directory: Directory) -> Self {
        DirectoryState::new(directory)
    }
}
