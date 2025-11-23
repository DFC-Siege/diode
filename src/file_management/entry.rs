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
            target: fs::read_link("/path/to/symlink")?,
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
            entries: Self::read_directory_recursive(&entry.path(), 0)?,
        })
    }

    fn from_without_entries(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            entries: Vec::new(),
        })
    }

    fn read_directory_recursive(path: &Path, max_depth: usize) -> io::Result<Vec<Box<dyn Entry>>> {
        fn recurse(
            path: &Path,
            max_depth: usize,
            current_depth: usize,
            paths: &mut Vec<Box<dyn Entry>>,
        ) -> io::Result<()> {
            if current_depth > max_depth {
                return Ok(());
            }

            let read_dir = graceful_return!(fs::read_dir(path));

            for entry in read_dir {
                let entry = graceful_return!(entry);
                let file_type = graceful_return!(entry.file_type());
                match (
                    file_type.is_file(),
                    file_type.is_dir(),
                    file_type.is_symlink(),
                ) {
                    (true, _, _) => {
                        paths.push(Box::new(graceful_return!(File::from(&entry))));
                    }
                    (_, true, _) => match current_depth >= max_depth {
                        true => paths.push(Box::new(graceful_return!(
                            Directory::from_without_entries(&entry)
                        ))),
                        false => paths.push(Box::new(graceful_return!(Directory::from(&entry)))),
                    },
                    (_, _, true) => {
                        paths.push(Box::new(graceful_return!(Symlink::from(&entry))));
                    }
                    _ => continue,
                };

                if path.is_dir() && current_depth < max_depth {
                    recurse(path, max_depth, current_depth + 1, paths)?;
                }
            }

            Ok(())
        }

        let mut paths = Vec::new();
        recurse(path, max_depth, 0, &mut paths)?;
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
