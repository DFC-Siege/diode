use std::io;

use crate::state::diode::{
    entry_state::EntryState,
    explorer_state::{ExplorerState, get_entry, get_entry_mut},
};

use log::error;

pub struct FocussedDirectory<'a> {
    pub state: &'a mut ExplorerState,
}

impl FocussedDirectory<'_> {
    pub fn toggle_dir(&mut self) -> io::Result<()> {
        let focussed_path = self
            .state
            .focussed
            .as_ref()
            .expect("focussed directory guarantees selection exists");

        let directory_state = get_entry_mut!(self.state, focussed_path, Directory);

        directory_state.collapsed = !directory_state.collapsed;
        let path = focussed_path.clone();

        if !directory_state.collapsed {
            let mut new_entries = ExplorerState::load_dir(directory_state)?;
            new_entries.extend(ExplorerState::get_from_cache(
                &path,
                &self.state.entries_cache,
            ));
            self.state.entries.extend(new_entries);
            self.state.uncollapse_dirs();
        } else {
            self.state.unload_dir(&path);
        }

        Ok(())
    }

    pub fn set_dir_as_root(&mut self) {
        let focussed_path = self
            .state
            .focussed
            .as_ref()
            .expect("focussed directory guarantees selection exists");

        self.state.root = get_entry!(self.state, focussed_path, Directory).clone();

        let new_entries = match ExplorerState::get_entries(&self.state.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };

        let old_entries = std::mem::replace(&mut self.state.entries, new_entries);
        self.state.entries_cache.extend(old_entries);
        self.state.apply_old_entry_states();

        self.state.uncollapse_dirs();
        self.state
            .navigate_to(Some(self.state.root.directory.path.to_owned()))
    }
}
