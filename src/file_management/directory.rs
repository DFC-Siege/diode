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
    pub parent: Weak<Entry>,
    pub entries: Vec<Rc<Entry>>,
}

impl Directory {
    pub fn try_from_recursive(
        entry: &DirEntry,
        parent: Weak<Entry>,
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

        let dir_rc = Rc::new(Entry::Directory(dir));
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

        if let Entry::Directory(dir_mut) =
            Rc::try_unwrap(dir_rc).map_err(|_| io::Error::other("failed to unwrap Rc"))?
        {
            Ok(Self { entries, ..dir_mut })
        } else {
            unreachable!()
        }
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
            parent: Weak::new(),
            entries: Vec::new(),
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
            parent: Weak::new(),
            entries: Vec::new(),
        })
    }
}
