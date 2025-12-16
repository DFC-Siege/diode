use crate::file_management::file::File;

#[derive(Debug, Clone)]
pub struct FileState {
    pub file: File,
    pub focussed: bool,
}

impl FileState {
    pub fn new(file: File) -> Self {
        Self {
            file,
            focussed: false,
        }
    }
}

impl From<File> for FileState {
    fn from(file: File) -> Self {
        FileState::new(file)
    }
}
