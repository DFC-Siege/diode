use std::io;

use crate::{file_management::directory::Directory, state::diode::entry_state::EntryState};

#[derive(Debug)]
pub struct DirectoryState {
    pub directory: Directory,
    pub collapsed: bool,
    pub selected: bool,
}

impl DirectoryState {
    pub fn new(directory: Directory) -> Self {
        Self {
            directory,
            collapsed: true,
            selected: false,
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
