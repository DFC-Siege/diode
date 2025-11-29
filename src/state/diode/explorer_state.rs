use futures::io;

use crate::state::diode::{directory_state::DirectoryState, entry_state::EntryState};
use std::{
    collections::BTreeMap,
    ops::Bound,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ExplorerState {
    pub root: DirectoryState,
    pub entries: BTreeMap<PathBuf, EntryState>,
    pub selected: Option<PathBuf>,
}

impl ExplorerState {
    pub fn try_new(root: DirectoryState) -> io::Result<Self> {
        let entries: BTreeMap<PathBuf, EntryState> = root
            .load_entry_states()?
            .into_iter()
            .map(|v| (v.path().to_owned(), v))
            .collect();

        Ok(Self {
            root,
            entries,
            selected: None,
        })
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = &self.selected {
            self.selected = self
                .entries
                .range::<Path, _>((Bound::Excluded(selected.as_path()), Bound::Unbounded))
                .next()
                .map(|(k, _)| k.clone());
        } else {
            self.selected = self.entries.keys().next().cloned();
        }
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = &self.selected {
            self.selected = self
                .entries
                .range::<Path, _>((Bound::Unbounded, Bound::Excluded(selected.as_path())))
                .next_back()
                .map(|(k, _)| k.clone());
        }
    }
}
