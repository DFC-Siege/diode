use crate::state::diode::selected::directory::SelectedDirectory;

pub enum SelectedEntry<'a> {
    Directory(SelectedDirectory<'a>),
}
