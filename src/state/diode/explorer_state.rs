use std::{
    collections::BTreeMap,
    io::ErrorKind,
    ops::Bound,
    path::{Path, PathBuf},
};

use futures::io;
use log::{debug, error};

use crate::{
    state::diode::{directory_state::DirectoryState, entry_state::EntryState},
    ui::explorer::explorer_pane::ExplorerPaneState,
};

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

    pub fn toggle_dir(&mut self) -> io::Result<()> {
        let selected_path = self
            .selected
            .clone()
            .ok_or_else(|| io::Error::new(ErrorKind::NotADirectory, "No valid entry selected"))?;

        let entry = self
            .entries
            .get_mut(&selected_path)
            .ok_or_else(|| io::Error::new(ErrorKind::NotADirectory, "No valid entry selected"))?;

        let (dir_path, was_collapsed) = match entry {
            EntryState::Directory(v) => {
                let was_collapsed = v.collapsed;
                v.collapsed = !v.collapsed;
                (v.directory.path.clone(), was_collapsed)
            }
            EntryState::File(_) => {
                return Err(io::Error::new(
                    ErrorKind::NotADirectory,
                    "No valid entry selected",
                ));
            }
        };

        if was_collapsed {
            let dir_state = match self.entries.get(&dir_path) {
                Some(EntryState::Directory(d)) => d,
                _ => return Ok(()),
            };
            let mut new_entries = ExplorerState::load_dir(dir_state)?;
            new_entries.extend(Self::get_from_cache(&dir_path, &self.entries_cache));
            self.entries.extend(new_entries);
            Self::uncollapse_dirs(&mut self.entries);
        } else {
            self.unload_dir(&dir_path);
        }

        Ok(())
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

    pub fn set_parent_as_new_root(&mut self) {
        let parent = match self.root.directory.get_parent_directory() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get parent: {}", e);
                return;
            }
        };

        self.root = parent.into();

        let entries = match Self::get_entries(&self.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };
        let old_entries = std::mem::take(&mut self.entries);
        self.entries = entries;
        Self::apply_old_entry_states(
            &mut self.entries,
            &self.entries_cache,
            &self.root.directory.path,
        );
        self.entries.extend(old_entries);
        Self::uncollapse_dirs(&mut self.entries);
        self.navigate_to(Some(self.root.directory.path.to_owned()))
    }

    pub fn set_dir_as_root(&mut self) {
        self.root = match self.get_selected_entry() {
            Some(EntryState::Directory(v)) => v.to_owned(),
            _ => {
                return;
            }
        };

        let new_entries = match Self::get_entries(&self.root) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed load entries: {}", e);
                return;
            }
        };

        let old_entries = std::mem::replace(&mut self.entries, new_entries);
        self.entries_cache.extend(old_entries);
        Self::apply_old_entry_states(
            &mut self.entries,
            &self.entries_cache,
            &self.root.directory.path,
        );

        Self::uncollapse_dirs(&mut self.entries);
        self.navigate_to(Some(self.root.directory.path.to_owned()))
    }

    fn uncollapse_dirs(entries: &mut BTreeMap<PathBuf, EntryState>) {
        let paths_with_parents: Vec<(PathBuf, PathBuf)> = entries
            .keys()
            .filter_map(|path| path.parent().map(|p| (path.clone(), p.to_path_buf())))
            .collect();

        for (_, parent_path) in paths_with_parents {
            if let Some(EntryState::Directory(dir)) = entries.get_mut(&parent_path) {
                dir.collapsed = false;
            }
        }
    }

    fn apply_old_entry_states(
        entries: &mut BTreeMap<PathBuf, EntryState>,
        cache: &BTreeMap<PathBuf, EntryState>,
        root: &Path,
    ) {
        let mut states_to_restore: BTreeMap<PathBuf, EntryState> = BTreeMap::new();
        for (path, old_state) in cache {
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

            let Some(new_state) = entries.get_mut(path) else {
                continue;
            };
            match (new_state, old_state) {
                (EntryState::Directory(v), EntryState::Directory(o)) => {
                    v.collapsed = o.collapsed;
                }
                _ => continue,
            }
        }
        entries.extend(states_to_restore);
    }

    fn get_from_cache(
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
