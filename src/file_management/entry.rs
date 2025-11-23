use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
};

macro_rules! define_entries {
    ($($name:ident {$($extra:tt)*})*) => {
        #[derive(Debug)]
        pub enum EntryType {
            $($name($name),)*
        }

        $(
        #[derive(Debug)]
        pub struct $name {
            path: PathBuf,
            name: OsString,
            $($extra)*
        }

        impl Entry for $name {
            fn name(&self) -> &OsStr {
                self.name.as_os_str()
            }
            fn path(&self) -> &OsStr {
                self.path.as_os_str()
            }

            fn try_from(entry: &DirEntry) -> io::Result<$name> {
                $name::try_from(entry)
            }
        }
    )*
    };
}

define_entries! {
    File{metadata: Metadata}
    Directory {metadata: Metadata, entries: Vec<EntryType>}
    Symlink {metadata: Metadata, target: PathBuf}
}

impl File {
    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
        })
    }
}

impl Symlink {
    pub fn try_from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            target: fs::read_link(entry.path())?,
        })
    }
}

impl Directory {
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

impl EntryType {
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

pub trait Entry: std::fmt::Debug {
    fn name(&self) -> &OsStr;
    fn path(&self) -> &OsStr;
    fn try_from(entry: &DirEntry) -> io::Result<Self>
    where
        Self: Sized;
}
