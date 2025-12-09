use std::{
    collections::BTreeMap,
    ops::Bound,
    path::{Path, PathBuf},
};

use futures::io;
use log::{debug, error};

use crate::{
    state::diode::{directory_state::DirectoryState, entry_state::EntryState},
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

pub enum SelectedEntry<'a> {
    Directory(SelectedDirectory<'a>),
}

pub struct SelectedDirectory<'a> {
    state: &'a mut ExplorerState,
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
        let path = selected_path.clone();

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

        let old_entries = std::mem::replace(&mut self.state.entries, new_entries);
        self.state.entries_cache.extend(old_entries);
        self.state.apply_old_entry_states();

        self.state.uncollapse_dirs();
        self.state
            .navigate_to(Some(self.state.root.directory.path.to_owned()))
    }

    pub fn set_parent_as_new_root(&mut self) {
        let parent = match self.state.root.directory.get_parent_directory() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get parent: {}", e);
                return;
            }
        };

        self.state.root = parent.into();

        let entries = match ExplorerState::get_entries(&self.state.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };
        let old_entries = std::mem::take(&mut self.state.entries);
        self.state.entries = entries;
        self.state.apply_old_entry_states();
        self.state.entries.extend(old_entries);
        self.state.uncollapse_dirs();
        self.state
            .navigate_to(Some(self.state.root.directory.path.to_owned()))
    }
}

#[derive(Debug)]
pub struct ExplorerState {
    pub root: DirectoryState,
    pub entries: BTreeMap<PathBuf, EntryState>,
    pub entries_cache: BTreeMap<PathBuf, EntryState>,
    pub selected: Option<PathBuf>,
    pub pane_state: ExplorerPaneState,
}

impl ExplorerState {
    pub fn try_new(root: DirectoryState) -> io::Result<Self> {
        let entries = Self::get_entries(&root)?;
        let entries_cache = entries.clone();

        Ok(Self {
            root,
            entries,
            entries_cache,
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

    fn unload_dir(&mut self, path: &Path) {
        let removed: BTreeMap<PathBuf, EntryState> = self
            .entries
            .iter()
            .filter(|(key, _)| key.starts_with(path))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        self.entries_cache.extend(removed);
        self.entries
            .retain(|key, _| !key.starts_with(path) || key == path);
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

        debug!("{:?}", new_path);
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

    pub fn uncollapse_dirs(&mut self) {
        let paths_with_parents: Vec<(PathBuf, PathBuf)> = self
            .entries
            .keys()
            .filter_map(|path| path.parent().map(|p| (path.clone(), p.to_path_buf())))
            .collect();

        for (_, parent_path) in paths_with_parents {
            if let Some(EntryState::Directory(dir)) = self.entries.get_mut(&parent_path) {
                dir.collapsed = false;
            }
        }
    }

    fn apply_old_entry_states(&mut self) {
        let mut states_to_restore: BTreeMap<PathBuf, EntryState> = BTreeMap::new();
        let root = &self.root.directory.path;
        for (path, old_state) in &self.entries_cache {
            if !path.starts_with(root) || path == root {
                continue;
            }

            match old_state {
                EntryState::Directory(v) => {
                    if !v.collapsed {
                        states_to_restore.insert(path.to_owned(), old_state.to_owned());
                    }
                }
                EntryState::File(_) => {
                    if let Some(parent) = path.parent()
                        && states_to_restore.contains_key(parent)
                    {
                        states_to_restore.insert(path.to_owned(), old_state.to_owned());
                    }
                }
            };

            let Some(new_state) = self.entries.get_mut(path) else {
                continue;
            };
            match (new_state, old_state) {
                (EntryState::Directory(v), EntryState::Directory(o)) => {
                    v.collapsed = o.collapsed;
                }
                _ => continue,
            }
        }
        self.entries.extend(states_to_restore);
    }

    pub fn get_from_cache(
        path: &Path,
        cache: &BTreeMap<PathBuf, EntryState>,
    ) -> BTreeMap<PathBuf, EntryState> {
        cache
            .iter()
            .filter(|(k, _)| k.starts_with(path) && k != &path)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
