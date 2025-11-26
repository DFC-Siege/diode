use std::rc::{Rc, Weak};

use crate::state::diode::{directory_state::DirectoryState, entry_state::EntryState};

struct ExplorerState {
    pub root: DirectoryState,
    pub selected_entry: Weak<EntryState>,
}

impl ExplorerState {
    pub fn move_down(&mut self) {
        match self.selected_entry.upgrade() {
            Some(v) => {
                if let Some(new_entry) = v.move_down() {
                    self.selected_entry = new_entry;
                }
            }
            None => {
                if let Some(first_entry) = self.root.entries.first() {
                    self.selected_entry = Rc::downgrade(first_entry);
                }
            }
        }
    }
}
