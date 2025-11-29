use crate::file_management::{directory::Directory, file::File};

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(File),
}
