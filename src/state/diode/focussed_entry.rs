use crate::state::diode::focussed::directory::FocussedDirectory;

pub enum FocussedEntry<'a> {
    Directory(FocussedDirectory<'a>),
}
