use std::{
    ffi::{OsStr, OsString},
    fs::{DirEntry, Metadata},
    io,
    path::PathBuf,
};

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
        }
    )*
    };
}

define_entries! {
    File{metadata: Metadata}
    Directory {metadata: Metadata, entries: Vec<Box<dyn Entry>>}
}

impl Directory {
    pub fn from(entry: &DirEntry) -> io::Result<Self> {
        Ok(Self {
            name: entry.file_name(),
            path: entry.path(),
            metadata: entry.metadata()?,
            entries: Vec::new(),
        })
    }
}

pub trait Entry: std::fmt::Debug {
    fn name(&self) -> &OsStr;
    fn path(&self) -> &OsStr;
}
