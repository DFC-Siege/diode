use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use futures::io;
use log::error;

use crate::{
    file_management::entry,
    state::diode::{
        directory_state::DirectoryState, entry_state::EntryState,
        selected::directory::SelectedDirectory, selected_entry::SelectedEntry,
    },
    ui::explorer::explorer_pane::ExplorerPaneState,
};

macro_rules! get_entry {
    ($self:expr, $path:expr, $variant:ident) => {
        match $self
            .entries
            .get($path)
            .expect("Path must exist in entries")
        {
            EntryState::$variant(entry) => entry,
            _ => unreachable!(concat!("Entry must be a ", stringify!($variant))),
        }
    };
}

macro_rules! get_entry_mut {
    ($self:expr, $path:expr, $variant:ident) => {
        match $self
            .entries
            .get_mut($path)
            .expect("Path must exist in entries")
        {
            EntryState::$variant(entry) => entry,
            _ => unreachable!(concat!("Entry must be a ", stringify!($variant))),
        }
    };
}

pub(crate) use get_entry;
pub(crate) use get_entry_mut;

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

    pub fn with_selected(&mut self) -> Option<SelectedEntry<'_>> {
        match self.get_selected_entry()? {
            EntryState::Directory(_) => {
                Some(SelectedEntry::Directory(SelectedDirectory { state: self }))
            }
            EntryState::File(_) => None,
        }
    }

    pub fn get_entries(root: &DirectoryState) -> io::Result<BTreeMap<PathBuf, EntryState>> {
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

    pub fn clear_marked(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|(_, v)| v.set_marked(false));
    }

    pub fn toggle_marked(&mut self) {
        let (path, is_marked) = {
            let Some(selected) = self.get_selected_entry() else {
                return;
            };
            (selected.path().to_owned(), selected.is_marked())
        };

        self.entries
            .iter_mut()
            .filter(|(k, _)| k.starts_with(&path))
            .for_each(|(_, v)| v.set_marked(!is_marked));
    }

    pub fn move_marked(&mut self, destination: &Path) -> io::Result<Vec<EntryState>> {
        let paths: Vec<PathBuf> = self
            .entries
            .iter()
            .filter(|(_, v)| v.is_marked())
            .map(|(v, _)| v.clone())
            .collect();

        let mut moved_paths: Vec<PathBuf> = Vec::new();
        let mut moved_entries: Vec<EntryState> = Vec::new();

        let dir = if destination.is_file() {
            let Some(dir) = destination.parent() else {
                return Err(io::Error::other("Destination is file but has no parent?"));
            };
            dir
        } else {
            destination
        };

        for p in paths {
            let is_child_of_moved = moved_paths.iter().any(|v| p.starts_with(v));

            if !is_child_of_moved {
                entry::move_entry(&p, dir)?;
                moved_paths.push(p.clone());
            }

            if let Some(mut entry) = self.entries.remove(&p) {
                let Some(file_name) = p.file_name() else {
                    error!(
                        "Failed to get file name of path: {}",
                        entry.path().to_string_lossy()
                    );
                    continue;
                };
                let new_path = dir.join(file_name);
                entry.set_path(new_path);
                entry.set_selected(false);
                moved_entries.push(entry);
            }
        }

        Ok(moved_entries)
    }

    pub fn reload(&mut self, entries: Vec<EntryState>) {
        for entry in entries {
            self.entries.insert(entry.path().to_owned(), entry);
        }
    }

    pub fn load_dir(directory: &DirectoryState) -> io::Result<BTreeMap<PathBuf, EntryState>> {
        let is_marked = directory.marked;
        Ok(directory
            .load_entry_states()?
            .into_iter()
            .map(|mut v| {
                v.set_marked(is_marked);
                (v.path().to_owned(), v)
            })
            .collect())
    }

    pub fn navigate_to(&mut self, new_path: Option<PathBuf>) {
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
        let next = {
            let mut entries = self.get_visible_entries();
            match &self.selected {
                Some(selected) => entries
                    .skip_while(|(k, _)| *k != selected)
                    .nth(1)
                    .map(|(k, _)| k.clone()),
                None => entries.next().map(|(k, _)| k.clone()),
            }
        };

        if let Some(path) = next {
            self.navigate_to(Some(path));
            self.pane_state.list_state.select_next();
        }
    }

    pub fn move_up(&mut self) {
        let prev = {
            let entries = self.get_visible_entries().rev();
            match &self.selected {
                Some(selected) => entries
                    .skip_while(|(k, _)| *k != selected)
                    .nth(1)
                    .map(|(k, _)| k.clone()),
                None => None,
            }
        };

        if let Some(path) = prev {
            self.navigate_to(Some(path));
            self.pane_state.list_state.select_previous();
        }
    }

    pub fn get_visible_entries(&self) -> impl DoubleEndedIterator<Item = (&PathBuf, &EntryState)> {
        let collapsed: Vec<&PathBuf> = self
            .entries
            .iter()
            .filter_map(|(k, v)| match v {
                EntryState::Directory(d) if d.collapsed => Some(k),
                _ => None,
            })
            .collect();

        self.entries
            .iter()
            .filter(move |(k, _)| !collapsed.iter().any(|c| k.starts_with(c) && k != c))
    }

    pub fn set_parent_as_new_root(&mut self) {
        let parent = match self.root.directory.get_parent_directory() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get parent: {}", e);
                return;
            }
        };

        self.root = parent.into();

        let entries = match ExplorerState::get_entries(&self.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };
        let old_entries = std::mem::take(&mut self.entries);
        self.entries = entries;
        self.entries.extend(old_entries);
        let first_key = self.entries.keys().next().cloned();
        self.navigate_to(first_key)
    }
}
