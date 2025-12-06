use std::{
    collections::BTreeMap,
    io::ErrorKind,
    ops::Bound,
    path::{Path, PathBuf},
};

use futures::io;
use log::error;

use crate::{
    state::diode::{directory_state::DirectoryState, entry_state::EntryState},
    ui::explorer::explorer_pane::ExplorerPaneState,
};

#[derive(Debug)]
pub struct ExplorerState {
    pub root: DirectoryState,
    pub entries: BTreeMap<PathBuf, EntryState>,
    pub selected: Option<PathBuf>,
    pub pane_state: ExplorerPaneState,
}

impl ExplorerState {
    pub fn try_new(root: DirectoryState) -> io::Result<Self> {
        let entries = Self::get_entries(&root)?;

        Ok(Self {
            root,
            entries,
            selected: None,
            pane_state: ExplorerPaneState::new(),
        })
    }

    fn get_entries(root: &DirectoryState) -> io::Result<BTreeMap<PathBuf, EntryState>> {
        Ok(root
            .load_entry_states()?
            .into_iter()
            .map(|v| (v.path().to_owned(), v))
            .collect())
    }

    pub fn get_selected_entry(&self) -> Option<&EntryState> {
        if let Some(selected) = &self.selected {
            self.entries.get(selected)
        } else {
            None
        }
    }

    pub fn get_selected_entry_mut(&mut self) -> Option<&mut EntryState> {
        if let Some(selected) = &self.selected {
            self.entries.get_mut(selected)
        } else {
            None
        }
    }

    pub fn toggle_dir(&mut self) -> io::Result<()> {
        let selected_path = self
            .selected
            .clone()
            .ok_or_else(|| io::Error::new(ErrorKind::NotADirectory, "No valid entry selected"))?;

        let entry = self
            .entries
            .get_mut(&selected_path)
            .ok_or_else(|| io::Error::new(ErrorKind::NotADirectory, "No valid entry selected"))?;

        match entry {
            EntryState::Directory(v) if v.collapsed => {
                v.collapsed = false;
                let new_entries = ExplorerState::load_dir(v)?;
                self.entries.extend(new_entries);
                Ok(())
            }
            EntryState::Directory(v) => {
                let path = v.directory.path.clone();
                v.collapsed = true;
                self.entries
                    .retain(|key, _| key == &path || !key.starts_with(&path));
                Ok(())
            }
            EntryState::File(_) => Err(io::Error::new(
                ErrorKind::NotADirectory,
                "No valid entry selected",
            )),
        }
    }

    fn unload_dir(&mut self, directory: &DirectoryState) {
        self.entries
            .retain(|key, _| !key.starts_with(&directory.directory.path));
    }

    fn load_dir(directory: &DirectoryState) -> io::Result<BTreeMap<PathBuf, EntryState>> {
        Ok(directory
            .load_entry_states()?
            .into_iter()
            .map(|v| (v.path().to_owned(), v))
            .collect())
    }

    fn navigate_to(&mut self, new_path: Option<PathBuf>) {
        if let Some(current) = &self.selected
            && let Some(entry) = self.entries.get_mut(current)
        {
            entry.set_selected(false);
        }

        if let Some(ref path) = new_path
            && let Some(entry) = self.entries.get_mut(path)
        {
            entry.set_selected(true);
        }

        self.selected = new_path;
    }

    pub fn move_down(&mut self) {
        let next = if let Some(selected) = &self.selected {
            self.entries
                .range::<Path, _>((Bound::Excluded(selected.as_path()), Bound::Unbounded))
                .next()
                .map(|(k, _)| k.clone())
        } else {
            self.entries.keys().next().cloned()
        };

        if next.is_some() {
            self.navigate_to(next);
            self.pane_state.list_state.select_next()
        }
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = &self.selected {
            let prev = self
                .entries
                .range::<Path, _>((Bound::Unbounded, Bound::Excluded(selected.as_path())))
                .next_back()
                .map(|(k, _)| k.clone());

            if prev.is_some() {
                self.navigate_to(prev);
                self.pane_state.list_state.select_previous()
            }
        }
    }

    pub fn set_parent_as_new_root(&mut self) {
        let parent = match self.root.directory.get_parent_directory() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get parent: {}", e);
                return;
            }
        };

        let old_root = self.root.clone();
        self.root = parent.into();

        let entries = match Self::get_entries(&self.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };
        let old_entries = self.entries.clone();
        self.entries = entries;
        self.restore_entry_states(old_entries, old_root);
    }

    fn restore_entry_states(
        &mut self,
        old_entries: BTreeMap<PathBuf, EntryState>,
        old_root: DirectoryState,
    ) {
        for (path, new_state) in &mut self.entries {
            if &old_root.directory.path == path {
                match new_state {
                    EntryState::Directory(v) => {
                        v.collapsed = false;
                    }
                    _ => continue,
                }
                continue;
            }

            let Some(old_state) = old_entries.get(path) else {
                continue;
            };
            match (new_state, old_state) {
                (EntryState::Directory(v), EntryState::Directory(o)) => {
                    v.selected = o.selected;
                    v.collapsed = o.collapsed;
                }
                (EntryState::File(v), EntryState::File(o)) => v.selected = o.selected,
                (_, _) => {
                    continue;
                }
            };
        }

        self.entries.extend(old_entries);
    }
}
