use crate::file_management::directory::Directory;

#[derive(Debug)]
pub struct FileState {
    pub left_dir: Directory,
    pub right_dir: Directory,
}

impl FileState {
    pub fn new(left_dir: Directory, right_dir: Directory) -> Self {
        Self {
            left_dir,
            right_dir,
        }
    }
}
