use crate::file_management::file::File;

#[derive(Debug, Clone)]
pub struct FileState {
    pub file: File,
    pub selected: bool,
    pub marked: bool,
}

impl FileState {
    pub fn new(file: File) -> Self {
        Self {
            file,
            selected: false,
            marked: false,
        }
    }
}

impl From<File> for FileState {
    fn from(file: File) -> Self {
        FileState::new(file)
    }
}
