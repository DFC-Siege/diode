use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
};

use crate::file_management::entry::EntryType;

#[derive(Debug)]
pub struct Directory {
    name: OsString,
    path: PathBuf,
    metadata: Metadata,
    entries: Vec<EntryType>,
}

impl Directory {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn try_from_recursive(
        entry: &DirEntry,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            entries: Self::recurse(&entry.path(), max_depth, current_depth)?,
        })
    }

    fn recurse(path: &Path, max_depth: usize, current_depth: usize) -> io::Result<Vec<EntryType>> {
        if current_depth > max_depth {
            return Ok(Vec::new());
        }
        Ok(fs::read_dir(path)?
            .filter_map(|v| {
                v.inspect_err(|e| eprintln!("Failed to read dir entry: {}", e))
                    .ok()
            })
            .filter_map(|v| {
                EntryType::try_from_recursive(&v, max_depth, current_depth)
                    .inspect_err(|e| eprintln!("Failed to process {:?}: {}", v.path(), e))
                    .ok()
            })
            .collect())
    }
}
