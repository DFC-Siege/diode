use crate::file_management::file::File;

#[derive(Debug)]
pub struct FileState {
    pub file: File,
    pub selected: bool,
}

impl FileState {
    pub fn new(file: File) -> Self {
        Self {
            file,
            selected: false,
        }
    }
}

impl From<File> for FileState {
    fn from(file: File) -> Self {
        FileState::new(file)
    }
}
