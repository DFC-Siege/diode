use crate::file_management::directory::Directory;

#[derive(Debug)]
pub struct FileSystemState {
    pub left_state: FileState,
    pub right_state: FileState,
}

#[derive(Debug)]
pub struct FileState {
    pub root: Directory,
    pub selected_index: Option<usize>,
}

impl FileState {
    pub fn new(root: Directory) -> Self {
        Self {
            root,
            selected_index: None,
        }
    }
}

impl FileSystemState {
    pub fn new(left_dir: Directory, right_dir: Directory) -> Self {
        Self {
            left_state: FileState::new(left_dir),
            right_state: FileState::new(right_dir),
        }
    }
}
