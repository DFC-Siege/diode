use std::{
    ffi::OsString,
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
};

use crate::file_management::entry::Entry;

#[derive(Debug)]
pub struct Directory {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub entries: Vec<Entry>,
}

impl Directory {
    pub fn try_from_recursive(
        entry: &DirEntry,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            entries: Self::recurse(&entry.path(), max_depth, current_depth),
        })
    }

    pub fn load_entries(&mut self) -> io::Result<()> {
        self.entries = Self::load_entries_with_depth(&self.path, 0, 0)?;

        Ok(())
    }

    fn recurse(path: &Path, max_depth: usize, current_depth: usize) -> Vec<Entry> {
        if current_depth > max_depth {
            return Vec::new();
        }

        match Self::load_entries_with_depth(path, max_depth, current_depth) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                Vec::new()
            }
        }
    }

    fn load_entries_with_depth(
        path: &Path,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Vec<Entry>> {
        let read_dir = fs::read_dir(path)?;

        Ok(read_dir
            .filter_map(|v| {
                v.inspect_err(|e| eprintln!("Failed to read dir entry: {}", e))
                    .ok()
            })
            .filter_map(|v| {
                Entry::try_from_recursive(&v, max_depth, current_depth)
                    .inspect_err(|e| eprintln!("Failed to process {:?}: {}", v.path(), e))
                    .ok()
            })
            .collect())
    }
}

impl TryFrom<&Path> for Directory {
    type Error = io::Error;

    fn try_from(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            entries: Self::recurse(path, 0, 0),
        })
    }
}

impl TryFrom<&PathBuf> for Directory {
    type Error = io::Error;

    fn try_from(path: &PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            entries: Self::recurse(path, 0, 0),
        })
    }
}
