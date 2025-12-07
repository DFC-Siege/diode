use std::{
    ffi::OsString,
    fs::{self, Metadata},
    io::{self},
    path::PathBuf,
};

use log::error;

use crate::file_management::entry::Entry;

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: OsString,
    pub path: PathBuf,

    #[allow(dead_code)]
    pub metadata: Metadata,
}

impl Directory {
    pub fn load_entries(&self) -> io::Result<Vec<Entry>> {
        Ok(fs::read_dir(&self.path)?
            .filter_map(|e| {
                let entry = e.ok()?;
                let path = entry.path();
                Entry::try_from(entry)
                    .map_err(|err| error!("Skipping {:?}: {}", path, err))
                    .ok()
            })
            .collect())
    }

    pub fn get_parent_directory(&self) -> io::Result<Directory> {
        let mut path = self.path.to_owned();
        if !path.pop() {
            return Err(io::Error::other("already at root directory"));
        }

        path.try_into()
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            name: path.file_name().unwrap_or_default().to_owned(),
            path: path.to_path_buf(),
            metadata: fs::metadata(path)?,
        })
    }
}
