use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct File {
    name: OsString,
    path: PathBuf,
    metadata: Metadata,
}

impl File {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
        })
    }
}

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

    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            // TODO: Implement config with serde using file at ~/.config/diode/config.toml
            // Increase to preload more directories
            entries: Self::recurse(&entry.path(), 0, 0)?,
        })
    }

    fn try_from_recursive(
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

#[derive(Debug)]
pub struct Symlink {
    name: OsString,
    path: PathBuf,
    metadata: Metadata,
    target: PathBuf,
}

impl Symlink {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            target: fs::read_link(entry.path())?,
        })
    }
}

#[derive(Debug)]
pub enum EntryType {
    File(File),
    Directory(Directory),
    Symlink(Symlink),
}

impl EntryType {
    pub fn name(&self) -> &OsStr {
        match self {
            EntryType::File(v) => v.name(),
            EntryType::Directory(v) => v.name(),
            EntryType::Symlink(v) => v.name(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            EntryType::File(v) => v.path(),
            EntryType::Directory(v) => v.path(),
            EntryType::Symlink(v) => v.path(),
        }
    }

    pub fn try_from_recursive(
        entry: &DirEntry,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Self> {
        let file_type = entry.file_type()?;
        match (
            file_type.is_file(),
            file_type.is_dir(),
            file_type.is_symlink(),
        ) {
            (true, _, _) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, true, _) => Ok(EntryType::Directory(Directory::try_from_recursive(
                entry,
                max_depth,
                current_depth + 1,
            )?)),
            (_, _, true) => Ok(EntryType::File(File::try_from(entry)?)),
            (_, _, _) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file type not supported",
            )),
        }
    }
}
