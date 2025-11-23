use ratatui::widgets::{List, ListItem};

use crate::{
    file_management::entry::EntryType,
    ui::explorer::{directory, file, symlink},
};

pub fn create_list(entries: &[EntryType]) -> List<'_> {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|v| match v {
            EntryType::Directory(dir) => directory::create_list_item(dir),
            EntryType::File(file) => file::create_list_item(file),
            EntryType::Symlink(symlink) => symlink::create_list_item(symlink),
        })
        .collect();

    List::new(items)
}
