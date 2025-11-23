use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, Metadata},
    io::{self},
    path::{Path, PathBuf},
};

macro_rules! graceful_return {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Warning: {}", e);
                return Ok(());
            }
        }
    };
    ($expr:expr, $ret:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Warning: {}", e);
                return $ret;
            }
        }
    };
}

macro_rules! define_entries {
    ($($name:ident {$($extra:tt)*})*) => {
        #[derive(Debug)]
        pub enum EntryType {
            $($name,)*
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

            fn from(entry: &DirEntry) -> io::Result<$name> {
                $name::from(entry)
            }
        }
    )*
    };
}

define_entries! {
    File{metadata: Metadata}
    Directory {metadata: Metadata, entries: Vec<Box<dyn Entry>>}
    Symlink {metadata: Metadata, target: PathBuf}
}

impl File {
    pub fn from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
        })
    }
}

impl Symlink {
    pub fn from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            target: fs::read_link(entry.path())?,
        })
    }
}

impl Directory {
    pub fn from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            // TODO: Implement config with serde using file at ~/.config/diode/config.toml
            // Increase to preload more directories
            entries: Self::recurse(&entry.path(), 1, 0)?,
        })
    }

    fn from_recursive(
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

    fn recurse(
        path: &Path,
        max_depth: usize,
        current_depth: usize,
    ) -> io::Result<Vec<Box<dyn Entry>>> {
        let mut paths: Vec<Box<dyn Entry>> = Vec::new();

        if current_depth > max_depth {
            return Ok(paths);
        }

        let read_dir = graceful_return!(fs::read_dir(path), Ok(paths));

        for entry in read_dir {
            let entry = graceful_return!(entry, Ok(paths));
            let file_type = graceful_return!(entry.file_type(), Ok(paths));

            match (
                file_type.is_file(),
                file_type.is_dir(),
                file_type.is_symlink(),
            ) {
                (true, _, _) => {
                    paths.push(Box::new(graceful_return!(File::from(&entry), Ok(paths))));
                }
                (_, true, _) => paths.push(Box::new(graceful_return!(
                    Directory::from_recursive(&entry, max_depth, current_depth + 1),
                    Ok(paths)
                ))),
                (_, _, true) => {
                    paths.push(Box::new(graceful_return!(Symlink::from(&entry), Ok(paths))));
                }
                _ => continue,
            };
        }

        Ok(paths)
    }
}

pub trait Entry: std::fmt::Debug {
    fn name(&self) -> &OsStr;
    fn path(&self) -> &OsStr;
    fn from(entry: &DirEntry) -> io::Result<Self>
    where
        Self: Sized;
}
