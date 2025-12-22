use std::io;

use crate::state::diode::{
    entry_state::EntryState,
    explorer_state::{ExplorerState, get_entry, get_entry_mut},
};

use log::error;

pub struct SelectedDirectory<'a> {
    pub state: &'a mut ExplorerState,
}

impl SelectedDirectory<'_> {
    pub fn toggle_dir(&mut self) -> io::Result<()> {
        let selected_path = self
            .state
            .selected
            .as_ref()
            .expect("SelectedDirectory guarantees selection exists");

        let directory_state = get_entry_mut!(self.state, selected_path, Directory);

        directory_state.collapsed = !directory_state.collapsed;

        if !directory_state.collapsed {
            for (k, v) in ExplorerState::load_dir(directory_state)? {
                self.state.entries.entry(k).or_insert(v);
            }
        }
        Ok(())
    }

    pub fn set_dir_as_root(&mut self) {
        let selected_path = self
            .state
            .selected
            .as_ref()
            .expect("SelectedDirectory guarantees selection exists");

        self.state.root = get_entry!(self.state, selected_path, Directory).clone();

        let new_entries = match ExplorerState::get_entries(&self.state.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };

        for (k, v) in new_entries {
            self.state.entries.entry(k).or_insert(v);
        }

        self.state
            .navigate_to(Some(self.state.root.directory.path.to_owned()))
    }
}
