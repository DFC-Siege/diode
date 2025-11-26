use std::{
    ffi::OsString,
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

use crate::file_management::entry::Entry;

#[derive(Debug)]
pub struct Directory {
    pub name: OsString,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub parent: Weak<Directory>,
    pub entries: Vec<Rc<Entry>>,
}

impl Directory {
    pub fn try_from_recursive(
        entry: &DirEntry,
        parent: Weak<Directory>,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Self> {
        let dir = Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            parent,
            entries: Vec::new(),
        };

        if current_depth >= max_depth {
            return Ok(dir);
        }

        let dir_rc = Rc::new(dir);
        let weak_self = Rc::downgrade(&dir_rc);

        let mut entries = Vec::new();
        for entry_result in fs::read_dir(entry.path())? {
            let entry = entry_result?;
            match Entry::try_from_recursive(&entry, weak_self.clone(), max_depth, current_depth) {
                Ok(child_entry) => entries.push(Rc::new(child_entry)),
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
                Err(e) => return Err(e),
            }
        }

        let dir_mut =
            Rc::try_unwrap(dir_rc).map_err(|_| io::Error::other("failed to unwrap Rc"))?;

        Ok(Self { entries, ..dir_mut })
    }
}

impl TryFrom<&Path> for Directory {
    type Error = io::Error;

    fn try_from(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        let mut dir = Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata,
            parent: Weak::new(),
            entries: Vec::new(),
        };

        let temp_rc = Rc::new(Self {
            name: dir.name.clone(),
            path: dir.path.clone(),
            metadata: dir.metadata.clone(),
            parent: Weak::new(),
            entries: Vec::new(),
        });
        let weak_self = Rc::downgrade(&temp_rc);

        for entry_result in fs::read_dir(path)? {
            let entry = entry_result?;
            match Entry::try_from_recursive(&entry, weak_self.clone(), usize::MAX, 0) {
                Ok(child_entry) => dir.entries.push(Rc::new(child_entry)),
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(dir)
    }
}

impl TryFrom<&PathBuf> for Directory {
    type Error = io::Error;

    fn try_from(path: &PathBuf) -> io::Result<Self> {
        Self::try_from(path.as_path())
    }
}
